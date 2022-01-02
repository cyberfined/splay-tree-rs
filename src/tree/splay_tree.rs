use std::marker::PhantomData;
use std::cmp::Ordering;
use std::mem;

use crate::{Node, NodePtr, Entry, VacantEntry, OccupiedEntry};
use crate::Entry::*;

/// Splay tree. [Read more](https://en.wikipedia.org/wiki/Splay_tree).
pub struct SplayTree<K: Ord, V> {
    root: NodePtr<K, V>,
    length: usize,
    marker: PhantomData<Box<Node<K, V>>>,
}

enum FindResult<K: Ord, V> {
    Found(*mut Node<K, V>),
    GoDown(*mut Node<K, V>),
    NotFound,
}

use self::FindResult::{Found, GoDown, NotFound};

impl<K: Ord, V> SplayTree<K, V> {
    /// Creates an empty `SplayTree`.
    #[inline]
    pub fn new() -> Self {
        SplayTree {
            root: None,
            length: 0,
            marker: PhantomData,
        }
    }

    /// Returns a mutable reference to the root node, or `None` if the tree is empty.
    ///
    /// This operation should compute in *O*(1) time.
    #[inline]
    pub fn root_mut(&mut self) -> Option<&mut Node<K, V>> {
        self.root.map(|mut r| unsafe { r.as_mut() })
    }

    /// Returns a reference to the root node, or `None` if the tree is empty.
    ///
    /// This operation should compute in *O*(1) time.
    #[inline]
    pub fn root(&self) -> Option<&Node<K, V>> {
        self.root.map(|r| unsafe { r.as_ref() })
    }

    /// Returns a mutable reference to the node by a key,
    /// or `None` if the tree doesn't contain that key.
    ///
    /// This operation should compute in amortized *O*(*log n*) time.
    pub fn get_mut(&mut self, key: &K) -> Option<&mut Node<K, V>> {
        match self.find_ptr(key) {
            Found(node_ptr) => unsafe { Some(&mut *node_ptr) },
            _ => None,
        }
    }

    /// Returns a reference to the node by a key,
    /// or `None` if the tree doesn't contain that key
    ///
    /// This operation should compute in amortized *O*(*log n*) time.
    pub fn get(&mut self, key: &K) -> Option<&Node<K, V>> {
        match self.find_ptr(key) {
            Found(node_ptr) => unsafe { Some(&*node_ptr) },
            _ => None,
        }
    }

    /// Returns a mutable reference to the node with a maximum key,
    /// or `None` if the tree is empty.
    ///
    /// This operation should compute in amortized *O*(*log n*) time.
    #[inline]
    pub fn get_max_mut(&mut self) -> Option<&mut Node<K, V>> {
        let max = self.root_mut()?.find_max();
        self.root = max.splay();
        self.root_mut()
    }

    /// Returns a reference to the node with a maximum key,
    /// or `None` if the tree is empty.
    ///
    /// This operation should compute in amortized *O*(*log n*) time.
    #[inline]
    pub fn get_max(&mut self) -> Option<&Node<K, V>> {
        let max = self.root_mut()?.find_max();
        self.root = max.splay();
        self.root()
    }


    /// Returns a mutable reference to the node with a minimum key,
    /// or `None` if the tree is empty.
    ///
    /// This operation should compute in amortized *O*(*log n*) time.
    #[inline]
    pub fn get_min_mut(&mut self) -> Option<&mut Node<K, V>> {
        let min = self.root_mut()?.find_min();
        self.root = min.splay();
        self.root_mut()
    }

    /// Returns a reference to the node with a minimum key,
    /// or `None` if the tree is empty.
    ///
    /// This operation should compute in amortized *O*(*log n*) time.
    #[inline]
    pub fn get_min(&mut self) -> Option<&Node<K, V>> {
        let min = self.root_mut()?.find_min();
        self.root = min.splay();
        self.root()
    }

    /// Inserts a value to the tree with a key. If the tree is already contains a key
    /// a value is replaced.
    ///
    /// This operation should compute in amortized *O*(*log n*) time.
    #[inline]
    pub fn insert<'a>(&'a mut self, key: K, value: V) -> &'a mut Node<K, V> {
        self.entry(key).insert(value)
    }

    #[inline]
    pub(crate) fn insert_child<'a>(
        &'a mut self,
        maybe_parent: Option<&'a mut Node<K, V>>,
        key: K,
        value: V
    ) -> Option<&'a mut Node<K, V>> {
        if let Some(parent) = maybe_parent {
            let node = parent.insert_child(key, value)?;
            self.root = node.splay();
            self.length += 1;
            Some(node)
        } else if self.root.is_none() {
            let node = Box::leak(Box::new(Node::new(key, value))).into();
            self.root = Some(node);
            self.length += 1;
            self.root_mut()
        } else {
            None
        }
    }

    /// Removes a node with a given key and returns it, or `None` if the tree
    /// doesn't contain that key.
    ///
    /// This operation should compute in amortized *O*(*log n*) time.
    #[inline]
    pub fn remove(&mut self, key: &K) -> Option<Box<Node<K, V>>> {
        let node = self.get_mut(key)?;
        let node_ptr: *const Node<K, V> = node;
        let left = node.left().map(|l| unsafe {
            &mut *mem::transmute::<*const Node<K, V>, *mut Node<K, V>>(l)
        });
        let right = node.right_mut();

        self.root = match (left, right) {
            (Some(l), Some(r)) => {
                l.parent = None;
                r.parent = None;
                l.merge(r)
            },
            (Some(l), None) => {
                l.parent = None;
                Some(l.into())
            }
            (None, Some(r)) => {
                r.parent = None;
                Some(r.into())
            }
            _ => None,
        };

        self.length -= 1;
        
        unsafe {
            let node = &mut *mem::transmute::<*const Node<K, V>, *mut Node<K, V>>(
                node_ptr
            );
            node.left = None;
            node.right = None;
            Some(node.ref_into_box())
        }
    }

    /// Returns `true` if the map contains no elements.
    #[inline]
    pub fn is_empty(&self) -> bool {
        self.root.is_none()
    }

    /// Returns `true` if the map contains a value for the specified key.
    #[inline]
    pub fn contains_key(&mut self, key: &K) -> bool {
        self.get(&key).is_some()
    }

    /// Gets the given key’s corresponding entry in the tree for in-place manipulation.
    ///
    /// This operation should compute in amortized *O*(*log n*) time.
    pub fn entry<'a>(&'a mut self, key: K) -> Entry<'a, K, V> {
        match self.find_ptr(&key) {
            NotFound => {
                Vacant(VacantEntry::new_root(self, key))
            },
            GoDown(parent_ptr) => {
                Vacant(VacantEntry::new_elem(self, unsafe { &mut *parent_ptr }, key))
            },
            Found(node_ptr) => {
                Occupied(OccupiedEntry::new(unsafe { &mut *node_ptr }))
            },
        }
    }

    #[inline]
    fn find_ptr(&mut self, key: &K) -> FindResult<K, V> {
        let mut cur_node = if let Some(root) = self.root_mut() {
            root
        } else {
            return NotFound
        };
        let mut is_found = false;

        loop {
            let ptr: *mut Node<K, V> = cur_node;
            let next_node = match key.cmp(cur_node.key()) {
                Ordering::Less => cur_node.left_mut(),
                Ordering::Equal => {
                    is_found = true;
                    self.root = cur_node.splay();
                    None
                },
                Ordering::Greater => cur_node.right_mut(),
            };

            cur_node = if let Some(next) = next_node {
                next
            } else if is_found {
                return Found(ptr)
            } else {
                return GoDown(ptr)
            };
        }
    }

    /// Returns the length of the `SplayTree`.
    ///
    /// This operation should compute in *O*(1) time.
    #[inline]
    pub fn len(&self) -> usize {
        self.length
    }
}

impl<K: Ord, V> Drop for SplayTree<K, V> {
    #[inline]
    fn drop(&mut self) {
        match self.root_mut() {
            None => {},
            Some(r) => r.free(),
        }
    }
}
