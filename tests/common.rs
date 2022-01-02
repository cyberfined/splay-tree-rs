use std::fmt::Debug;

use splay_tree::{SplayTree, Node};

pub fn create_tree(buf: &[u32]) -> SplayTree<u32, u32> {
    let mut tree = SplayTree::new();

    for i in buf.iter().cloned() {
        tree.insert(i, i);

        assert!(tree.root().is_some());
        if let Some(root) = tree.root() {
            assert_eq!(*root.value(), i);
        }
    }

    assert_eq!(tree.len(), buf.len());
    check_tree_structure(&tree);

    tree
}

pub fn check_tree_structure<K: Ord + Debug, V>(tree: &SplayTree<K, V>) {
    if let Some(root) = tree.root() {
        assert!(root.parent().is_none());
        let length = check_node_structure(root, 0);
        assert_eq!(length, tree.len());
    }
}

fn check_node_structure<K: Ord + Debug, V>(node: &Node<K, V>, mut length: usize) -> usize {
    length += 1;

    if let Some(left) = node.left() {
        assert!(left.key() < node.key());
        assert_eq!(left.parent().unwrap().key(), node.key());
        length = check_node_structure(left, length);
    }

    if let Some(right) = node.right() {
        assert!(right.key() > node.key());
        assert_eq!(right.parent().unwrap().key(), node.key());
        length = check_node_structure(right, length);
    }

    length
}
