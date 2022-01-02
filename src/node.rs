#[cfg(feature = "recursive_debug")]
use std::fmt;

use std::mem;
use std::fmt::Debug;
use std::ptr::NonNull;
use std::cmp::Ordering;

pub(crate) type NodePtr<K, V> = Option<NonNull<Node<K, V>>>;

/// Splay tree's node.
#[cfg_attr(not(feature = "recursive_debug"), derive(Debug))]
pub struct Node<K: Ord, V> {
    key: K,
    value: V,
    pub(crate) left: NodePtr<K, V>,
    pub(crate) right: NodePtr<K, V>,
    pub(crate) parent: NodePtr<K, V>,
}

#[cfg(feature = "recursive_debug")]
impl<K: Ord + Debug, V: Debug> Debug for Node<K, V> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> Result<(), fmt::Error> {
        self.print_tree_prefix(f, String::from(""), String::from(""))
    }
}

#[cfg(feature = "recursive_debug")]
impl<K: Ord + Debug, V: Debug> Node<K, V> {
    fn print_tree_prefix(
        &self,
        f: &mut fmt::Formatter<'_>,
        prefix: String,
        c_prefix: String
    ) -> Result<(), fmt::Error> {
        f.write_fmt(format_args!("{}{:?}({:?})\n", prefix, self.key, self.value))?;
        let right_c_prefix;

        if let Some(left) = self.left() {
            right_c_prefix = c_prefix.clone();
            let (new_prefix, new_c_prefix) = if self.right.is_some() {
                (c_prefix.clone() + "├── ", c_prefix + "│   ")
            } else {
                (c_prefix.clone() + "└── ", c_prefix + "    ")
            };
            left.print_tree_prefix(f, new_prefix, new_c_prefix)?;
        } else {
            right_c_prefix = c_prefix;
        }

        if let Some(right) = self.right() {
            let new_prefix = right_c_prefix.clone() + "└── ";
            let new_c_prefix = right_c_prefix + "    ";
            right.print_tree_prefix(f, new_prefix, new_c_prefix)?;
        }

        Ok(())
    }
}

impl<K: Ord, V> Node<K, V> {
    #[inline]
    pub(crate) fn new(key: K, value: V) -> Self {
        Node {
            key: key,
            value: value,
            left: None,
            right: None,
            parent: None,
        }
    }

    /// Returns a mutable reference to the node's parent, or `None` if the node is a root.
    ///
    /// This operation should compute in *O*(1) time.
    #[inline]
    pub fn parent_mut(&mut self) -> Option<&mut Self> {
        self.parent.map(|mut p| unsafe { p.as_mut() })
    }

    /// Returns a reference to the node's parent, or `None` if the node is a root.
    ///
    /// This operation should compute in *O*(1) time.
    #[inline]
    pub fn parent(&self) -> Option<&Self> {
        self.parent.map(|p| unsafe { p.as_ref() })
    }

    /// Returns a mutable reference to the node's left child, or `None`
    /// if the node doesn't have one.
    ///
    /// This operation should compute in *O*(1) time.
    #[inline]
    pub fn left_mut(&mut self) -> Option<&mut Self> {
        self.left.map(|mut l| unsafe { l.as_mut() })
    }

    /// Returns a reference to the node's left child, or `None`
    /// if the node doesn't have one.
    ///
    /// This operation should compute in *O*(1) time.
    #[inline]
    pub fn left(&self) -> Option<&Self> {
        self.left.map(|l| unsafe { l.as_ref() })
    }

    /// Returns a mutable reference to the node's right child, or `None`
    /// if the node doesn't have one.
    ///
    /// This operation should compute in *O*(1) time.
    #[inline]
    pub fn right_mut(&mut self) -> Option<&mut Self> {
        self.right.map(|mut r| unsafe { r.as_mut() })
    }

    /// Returns a reference to the node's right child, or `None`
    /// if the node doesn't have one.
    ///
    /// This operation should compute in *O*(1) time.
    #[inline]
    pub fn right(&self) -> Option<&Self> {
        self.right.map(|r| unsafe { r.as_ref() })
    }

    /// Returns a reference to the node's key.
    #[inline]
    pub fn key(&self) -> &K {
        &self.key
    }

    /// Returns a mutable reference to the node's value.
    #[inline]
    pub fn value_mut(&mut self) -> &mut V {
        &mut self.value
    }

    /// Returns a reference to the node's value.
    #[inline]
    pub fn value(&self) -> &V {
        &self.value
    }

    /// Returns `true` if the node is a root.
    #[inline]
    pub fn is_root(&self) -> bool {
        self.parent.is_none()
    }

    /// Returns `true` if the node is a left child.
    #[inline]
    pub fn is_left(&self) -> bool {
        if let Some(parent) = self.parent() {
            parent.left.map(|l| unsafe {
                mem::transmute::<*mut Self, *const Self>(l.as_ptr()) == self
            }).unwrap_or(false)
        } else {
            false
        }
    }

    /// Returns `true` if the node is a right child.
    #[inline]
    pub fn is_right(&self) -> bool {
        if let Some(parent) = self.parent() {
            parent.right.map(|l| unsafe {
                mem::transmute::<*mut Self, *const Self>(l.as_ptr()) == self
            }).unwrap_or(false)
        } else {
            false
        }
    }

    #[inline]
    pub(crate) unsafe fn ref_into_box(&mut self) -> Box<Self> {
        Box::from_raw(self)
    }

    #[inline]
    pub(crate) fn insert_child<'a>(
        &'a mut self,
        key: K,
        value: V
    ) -> Option<&'a mut Self> {
        match key.cmp(&self.key) {
            Ordering::Less if self.left.is_none() => {
                let node = Node {
                    key: key,
                    value: value,
                    parent: Some(self.into()),
                    left: None,
                    right: None,
                };
                let node_ptr = Box::leak(Box::new(node)).into();
                self.left = Some(node_ptr);
                self.left_mut()
            },
            Ordering::Equal => {
                self.value = value;
                Some(self)
            },
            Ordering::Greater if self.right.is_none() => {
                let node = Node {
                    key: key,
                    value: value,
                    parent: Some(self.into()),
                    left: None,
                    right: None,
                };
                let node_ptr = Box::leak(Box::new(node)).into();
                self.right = Some(node_ptr);
                self.right_mut()
            },
            _ => None,
        }
    }

    #[inline]
    pub(crate) fn splay(&mut self) -> NodePtr<K, V> {
        loop {
            if let Some(new_root) = self.splay_step() {
                return Some(new_root)
            }
        }
    }

    fn splay_step(&mut self) -> NodePtr<K, V> {
        let self_ptr = self.into();
        let is_left = self.is_left();

        if let Some(splay_type) = self.splay_type() {
            match splay_type {
                SplayType::Zig => {
                    self.parent_mut().map(|p|
                        if is_left {
                            p.rotate_right();
                        } else {
                            p.rotate_left();
                        }
                    );
                },
                SplayType::ZigZig => {
                    self.parent_mut().map(|p|
                        if is_left {
                            p.parent_mut().map(|g| g.rotate_right());
                            p.rotate_right();
                        } else {
                            p.parent_mut().map(|g| g.rotate_left());
                            p.rotate_left();
                        }
                    );
                },
                SplayType::ZigZag => {
                    if is_left {
                        self.parent_mut().map(|p| p.rotate_right());
                        self.parent_mut().map(|g| g.rotate_left());
                    } else {
                        self.parent_mut().map(|p| p.rotate_left());
                        self.parent_mut().map(|g| g.rotate_right());
                    }
                },
            }
        }

        if self.is_root() {
            Some(self_ptr)
        } else {
            None
        }
    }

    #[inline]
    pub(crate) fn merge(&mut self, right: &mut Self) -> NodePtr<K, V> {
        let left_max = self.find_max();
        let res = left_max.splay();
        right.parent = res;
        left_max.right = Some(right.into());
        res
    }

    #[inline]
    pub(crate) fn find_max(&mut self) -> &mut Self {
        let mut cur_node = self;

        loop {
            let ptr: *const Self = cur_node;
            cur_node = if let Some(next) = cur_node.right_mut() {
                next
            } else {
                break unsafe {
                    &mut *mem::transmute::<*const Self, *mut Self>(ptr)
                };
            }
        }
    }

    #[inline]
    pub(crate) fn find_min(&mut self) -> &mut Self {
        let mut cur_node = self;

        loop {
            let ptr: *const Self = cur_node;
            cur_node = if let Some(next) = cur_node.left_mut() {
                next
            } else {
                break unsafe {
                    &mut *mem::transmute::<*const Self, *mut Self>(ptr)
                };
            }
        }
    }

    #[inline]
    fn rotate_left(&mut self) {
        let self_ptr = self.into();
        let parent_ptr = self.parent;
        let right_ptr = self.right;
        let right = match self.right_mut() {
            Some(ptr) => ptr,
            None => return,
        };
        let left_ptr = right.left;

        right.parent = parent_ptr;

        right.left = Some(self_ptr);

        let is_left = self.is_left();
        if let Some(parent) = self.parent_mut() {
            if is_left {
                parent.left = right_ptr;
            } else {
                parent.right = right_ptr;
            }
        }

        self.parent = right_ptr;

        self.right = left_ptr;
        self.right_mut().map(|mut r| { r.parent = Some(self_ptr); r });
    }

    #[inline]
    fn rotate_right(&mut self) {
        let self_ptr = self.into();
        let parent_ptr = self.parent;
        let left_ptr = self.left;
        let left = match self.left_mut() {
            Some(ptr) => ptr,
            None => return,
        };
        let right_ptr = left.right;

        left.parent = parent_ptr;

        left.right = Some(self_ptr);

        let is_left = self.is_left();
        if let Some(parent) = self.parent_mut() {
            if is_left {
                parent.left = left_ptr;
            } else {
                parent.right = left_ptr;
            }
        }

        self.parent = left_ptr;

        self.left = right_ptr;
        self.left_mut().map(|mut l| l.parent = Some(self_ptr));
    }

    #[inline]
    fn splay_type(&self) -> Option<SplayType> {
        if self.parent()?.is_root() {
            Some(SplayType::Zig)
        } else if (self.is_left() && self.parent()?.is_left()) ||
                  (self.is_right() && self.parent()?.is_right()) {
            Some(SplayType::ZigZig)
        } else if !self.parent()?.is_root() {
            Some(SplayType::ZigZag)
        } else {
            None
        }
    }

    pub(crate) fn free(&mut self) {
        match self.left {
            Some(left_ptr) => {
                let mut left_box = unsafe { Box::from_raw(left_ptr.as_ptr()) };
                left_box.free();
            },
            None => {},
        }

        match self.right {
            Some(right_ptr) => {
                let mut right_box = unsafe { Box::from_raw(right_ptr.as_ptr()) };
                right_box.free();
            },
            None => {},
        }
    }
}

#[derive(Debug, PartialEq)]
enum SplayType {
    Zig,
    ZigZig,
    ZigZag
}

#[cfg(test)]
mod tests;
