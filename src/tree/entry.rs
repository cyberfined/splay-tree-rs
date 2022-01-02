use crate::{SplayTree, Node};

pub enum Entry<'a, K: Ord, V> {
    Occupied(OccupiedEntry<'a, K, V>),
    Vacant(VacantEntry<'a, K, V>),
}

use Entry::{Occupied, Vacant};

impl<'a, K: Ord, V> Entry<'a, K, V> {
    /// Ensures a value is in the entry by inserting the default if empty,
    /// and returns a mutable reference to the node.
    #[inline]
    pub fn or_insert(self, value: V) -> &'a mut Node<K, V> {
        match self {
            Occupied(entry) => entry.elem,
            Vacant(entry) => entry.insert(value),
        }
    }

    /// Ensures a value is in the entry by inserting the result of the default function
    /// if empty, and returns a mutable reference to the node.
    #[inline]
    pub fn or_insert_with<F: FnOnce() -> V>(self, calc: F) -> &'a mut Node<K, V> {
        match self {
            Occupied(entry) => entry.elem,
            Vacant(entry) => entry.insert(calc()),
        }
    }

    /// Ensures a value is in the entry by inserting, if empty,
    /// the result of the default function. This method allows for generating
    /// key-derived values for insertion by providing the default function a reference
    /// to the key that was moved during the .entry(key) method call.
    #[inline]
    pub fn or_insert_with_key<F: FnOnce(&K) -> V>(self, calc: F) -> &'a mut Node<K, V> {
        match self {
            Occupied(entry) => entry.elem,
            Vacant(entry) => {
                let val = calc(&entry.key);
                entry.insert(val)
            }
        }
    }

    /// Returns a reference to this entry's key.
    #[inline]
    pub fn key(&self) -> &K {
        match self {
            Occupied(entry) => entry.elem.key(),
            Vacant(entry) => &entry.key,
        }
    }

    /// Provides in-place mutable access to an occupied entry before any potential
    /// inserts into the map.
    #[inline]
    pub fn and_modify<F: FnOnce(&mut V)>(self, f: F) -> Self {
        match self {
            Occupied(entry) => {
                f(entry.elem.value_mut());
                Occupied(entry)
            },
            _ => self,
        }
    }

    /// Sets the value of the entry, and returns mutable reference to the node.
    #[inline]
    pub fn insert(self, value: V) -> &'a mut Node<K, V> {
        match self {
            Occupied(entry) =>  {
                *entry.elem.value_mut() = value;
                entry.elem
            },
            Vacant(entry) => {
                entry.insert(value)
            },
        }
    }
}

impl<'a, K: Ord, V: Default> Entry<'a, K, V> {
    /// Ensures a value is in the entry by inserting the default value
    /// if empty, and returns a mutable reference to the node.
    #[inline]
    pub fn or_default(self) -> &'a mut Node<K, V> {
        match self {
            Occupied(entry) => entry.elem,
            Vacant(entry) => entry.insert(V::default()),
        }
    }
}

pub struct OccupiedEntry<'a, K: Ord, V> {
    elem: &'a mut Node<K, V>,
}

impl<'a, K: Ord, V> OccupiedEntry<'a, K, V> {
    #[inline]
    pub(crate) fn new(elem: &'a mut Node<K, V>) -> Self {
        OccupiedEntry { elem }
    }
}

pub struct VacantEntry<'a, K: Ord, V> {
    tree: &'a mut SplayTree<K, V>,
    parent: Option<&'a mut Node<K, V>>,
    key: K,
}

impl<'a, K: Ord, V> VacantEntry<'a, K, V> {
    #[inline]
    pub(crate) fn new_root(tree: &'a mut SplayTree<K, V>, key: K) -> Self {
        VacantEntry {
            tree: tree,
            parent: None,
            key: key,
        }
    }

    #[inline]
    pub(crate) fn new_elem(
        tree: &'a mut SplayTree<K, V>,
        parent: &'a mut Node<K, V>,
        key: K
    ) -> Self {
        VacantEntry {
            tree: tree,
            parent: Some(parent),
            key: key,
        }
    }

    #[inline]
    fn insert(self, value: V) -> &'a mut Node<K, V> {
        self.tree.insert_child(self.parent, self.key, value).unwrap()
    }
}
