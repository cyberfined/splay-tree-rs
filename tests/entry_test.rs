use splay_tree::SplayTree;

mod common;

#[test]
fn or_insert_test() {
    let elems = [13, 4, 3, 89, 12, 11];
    let mut tree = SplayTree::<u32, u32>::new();
    for i in elems {
        let node = tree.entry(i).or_insert(i);
        assert_eq!(node.key(), &i);
    }
    common::check_tree_structure(&tree);

    let node = tree.entry(89).or_insert(11);
    assert_eq!(node.key(), &89);
    common::check_tree_structure(&tree);
}

#[test]
fn or_insert_with_test() {
    let elems = [9065, 1234, 432, 90, 11, 4, 2, 3, 0];
    let mut tree = SplayTree::<u32, u32>::new();
    for i in elems {
        let node = tree.entry(i).or_insert_with(|| i);
        assert_eq!(node.key(), &i);
    }
    common::check_tree_structure(&tree);

    let node = tree.entry(432).or_insert_with(|| 15);
    assert_eq!(node.key(), &432);
    common::check_tree_structure(&tree);
}

#[test]
fn or_insert_with_key_test() {
    let elems = [111, 234, 12, 90, 0, 1, 2, 4, 56];
    let mut tree = SplayTree::<u32, u32>::new();
    for i in elems {
        let node = tree.entry(i).or_insert_with_key(|k| k * 2);
        assert_eq!(node.key(), &i);
        assert_eq!(*node.value(), i * 2);
    }
    common::check_tree_structure(&tree);

    let node = tree.entry(0).or_insert_with_key(|k| k + 2);
    assert_eq!(node.key(), &0);
    assert_eq!(node.value(), &0);
    common::check_tree_structure(&tree);
}

#[test]
fn and_modify_test() {
    let elems = [134, 45, 91, 34, 21, 27];
    let mut tree = common::create_tree(&elems);
    for i in elems {
        let node = tree.entry(i).and_modify(|x| *x += 2).or_insert(2);
        assert_eq!(*node.value(), i + 2);
    }
    common::check_tree_structure(&tree);

    let node = tree.entry(666).and_modify(|x| *x *= 2).or_insert(1);
    assert_eq!(node.value(), &1);
    common::check_tree_structure(&tree);
}

#[test]
fn insert_test() {
    let elems = [56, 89, 11, 14, 90];
    let mut tree = common::create_tree(&elems);
    for i in elems {
        let node = tree.entry(i).insert(i * 2 + 2);
        assert_eq!(node.key(), &i);
        assert_eq!(*node.value(), i * 2 + 2);
    }
    common::check_tree_structure(&tree);

    let node = tree.entry(777).insert(888);
    assert_eq!(node.value(), &888);
    common::check_tree_structure(&tree);
}

#[test]
fn or_default_test() {
    let elems = [1114, 11, 1111, 34, 65, 72, 89, 91];
    let mut tree = SplayTree::<u32, u32>::new();
    for i in elems {
        let node = tree.entry(i).or_default();
        assert_eq!(node.value(), &0);
    }
    common::check_tree_structure(&tree);

    tree.insert(1234, 56);
    let node = tree.entry(1234).or_default();
    assert_eq!(node.value(), &56);
    common::check_tree_structure(&tree);
}

#[test]
fn key_test() {
    let elems = [1234, 543, 877, 777777, 99981];
    let mut tree = SplayTree::<u32, u32>::new();
    for i in elems {
        tree.insert(i, i);
        assert_eq!(tree.entry(i).key(), &i);
    }
    assert_eq!(tree.entry(3333).key(), &3333);
}
