mod common;

use splay_tree::SplayTree;

#[test]
fn test_get() {
    let elems = [11, 17, 9, 0, 13, 2, 4, 3];
    let mut tree = common::create_tree(&elems);

    for i in elems.iter() {
        let node = tree.get(i).unwrap();
        assert_eq!(node.key(), i);
        assert_eq!(tree.root().unwrap().key(), i);
    }

    let missing = [127, 1134, 9089, 1, 5, 6];
    for i in missing.iter() {
        assert!(tree.get(i).is_none());
    }

    common::check_tree_structure(&tree);
}

#[test]
fn test_get_min() {
    let elems = [12, 45, 11, 90, 43, 34, 78, 91];
    let mut tree = common::create_tree(&elems);
    let min = tree.get_min().unwrap();
    assert_eq!(*min.key(), 11);
    assert_eq!(*tree.root().unwrap().key(), 11);
    common::check_tree_structure(&tree);
}

#[test]
fn test_get_min_empty() {
    let mut tree = SplayTree::<u32, u32>::new();
    assert!(tree.get_min().is_none());
}

#[test]
fn test_get_max() {
    let elems = [123, 0, 134, 7896, 1003, 3456, 90, 34, 1, 2, 3];
    let mut tree = common::create_tree(&elems);
    let max = tree.get_max().unwrap();
    assert_eq!(*max.key(), 7896);
    assert_eq!(*tree.root().unwrap().key(), 7896);
    common::check_tree_structure(&tree);
}

#[test]
fn test_get_max_empty() {
    let mut tree = SplayTree::<u32, u32>::new();
    assert!(tree.get_max().is_none());
}

#[test]
fn test_remove() {
    let elems = [23, 45, 12, 90, 46, 89, 78, 91];
    let mut tree = common::create_tree(&elems);
    let mut length = tree.len();

    for i in elems[0..5].iter() {
        let node = tree.remove(i).unwrap();
        assert_eq!(node.key(), i);
        assert_eq!(node.value(), i);
        assert!(node.parent().is_none());
        assert!(node.left().is_none());
        assert!(node.right().is_none());
        assert!(tree.get(i).is_none());
        length -= 1;
        assert_eq!(tree.len(), length);
    }

    common::check_tree_structure(&tree);

    for i in elems[5..].iter() {
        let node = tree.remove(i).unwrap();
        assert_eq!(node.key(), i);
        assert_eq!(node.value(), i);
        assert!(node.parent().is_none());
        assert!(node.left().is_none());
        assert!(node.right().is_none());
        assert!(tree.get(i).is_none());
        length -= 1;
        assert_eq!(tree.len(), length);
    }

    assert!(tree.root().is_none());
}

#[test]
fn is_empty_test() {
    let mut tree = SplayTree::<u32, u32>::new();
    assert!(tree.is_empty());
    tree.insert(4, 4);
    assert!(!tree.is_empty());
}

#[test]
fn contains_key_test() {
    let elems = [34, 22, 11, 90, 321, 7778, 991, 1111];
    let mut tree = common::create_tree(&elems);
    for i in elems {
        assert!(tree.contains_key(&i));
    }
    let missing = [7, 8, 9, 10, 12];
    for i in missing {
        assert!(!tree.contains_key(&i));
    }
}
