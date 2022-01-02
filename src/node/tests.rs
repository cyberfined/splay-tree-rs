use super::*;
use std::collections::LinkedList;
use std::ptr::NonNull;
use std::fmt::Debug;

struct Root<K: Ord, V> {
    root: NodePtr<K, V>,
}

impl<K: Ord, V> Root<K, V> {
    pub fn root_mut(&mut self) -> Option<&mut Node<K, V>> {
        self.root.map(|mut r| unsafe { r.as_mut() })
    }

    fn root(&self) -> Option<&Node<K, V>> {
        self.root.map(|r| unsafe { r.as_ref() })
    }
}

fn check_tree_structure<K: Ord + Debug, V: Debug>(root: &Node<K, V>, keys: &[K]) {
    assert!(root.parent.is_none());
    check_node_structure(root, keys);
}

fn check_node_structure<'a, K, V>(root: &Node<K, V>, mut keys: &'a [K]) -> &'a [K]
    where K: Ord + Debug, V: Debug
{
    assert!(keys.len() != 0);
    assert_eq!(root.key, keys[0]);

    keys = &keys[1..];

    if let Some(left) = root.left() {
        let parent_ptr = unsafe {
            mem::transmute::<*mut Node<K, V>, *const Node<K, V>>(
                left.parent.unwrap().as_ptr()
            )
        };
        assert_eq!(parent_ptr, root);

        keys = check_node_structure(left, keys);
    }

    if let Some(right) = root.right() {
        let parent_ptr = unsafe {
            mem::transmute::<*mut Node<K, V>, *const Node<K, V>>(
                right.parent.unwrap().as_ptr()
            )
        };
        assert_eq!(parent_ptr, root);

        keys = check_node_structure(right, keys);
    }

    keys
}

fn tree_from_slice<K: Ord + Copy>(mut keys: &[Option<K>]) -> Option<Root<K, K>> {
    let root_ptr = NonNull::new(Box::leak(Box::new(Node {
        key: keys[0]?,
        value: keys[0]?,
        left: None,
        right: None,
        parent: None,
    })))?;
    let mut queue = LinkedList::new();
    queue.push_back(root_ptr);

    keys = &keys[1..];

    for i in (0..keys.len()).step_by(2) {
        let mut cur_node_ptr = queue.pop_front()?;
        let cur_node = unsafe { cur_node_ptr.as_mut() };

        if let Some(left_key) = keys[i] {
            let left_ptr = NonNull::new(Box::leak(Box::new(Node {
                key: left_key,
                value: left_key,
                left: None,
                right: None,
                parent: Some(cur_node_ptr),
            })))?;

            queue.push_back(left_ptr);
            cur_node.left = Some(left_ptr);
        }

        if i + 1 >= keys.len() {
            break
        }

        if let Some(right_key) = keys[i + 1] {
            let right_ptr = NonNull::new(Box::leak(Box::new(Node {
                key: right_key,
                value: right_key,
                left: None,
                right: None,
                parent: Some(cur_node_ptr),
            })))?;

            queue.push_back(right_ptr);
            cur_node.right = Some(right_ptr);
        }
    }

    Some(Root { root: Some(root_ptr) })
}

#[test]
fn splay_zig_left() {
    let mut tree = tree_from_slice(&[Some(10u32), Some(5), Some(12), Some(3), Some(6)]).unwrap();
    let elem = tree.root_mut().unwrap().left_mut().unwrap(); // 5
    assert_eq!(elem.splay_type(), Some(SplayType::Zig));
    elem.splay().map(|r| tree.root = Some(r));
    check_tree_structure(tree.root().unwrap(), &[5, 3, 10, 6, 12]);
}

#[test]
fn splay_zig_right() {
    let mut tree = tree_from_slice(&[
        Some(10u32), Some(5), Some(20), None, None, Some(15), Some(30)
    ]).unwrap();
    let elem = tree.root_mut().unwrap().right_mut().unwrap(); // 20
    assert_eq!(elem.splay_type(), Some(SplayType::Zig));
    elem.splay().map(|r| tree.root = Some(r));
    check_tree_structure(tree.root().unwrap(), &[20, 10, 5, 15, 30]);
}

#[test]
fn splay_zig_zig_left() {
    let mut tree = tree_from_slice(&[
        Some(20u32), Some(15), Some(30), Some(13), Some(16), None, None,
        Some(12), Some(14)
    ]).unwrap();
    let elem = tree.root_mut().unwrap().left_mut().unwrap().left_mut().unwrap(); // 13
    assert_eq!(elem.splay_type(), Some(SplayType::ZigZig));
    elem.splay().map(|r| tree.root = Some(r));
    check_tree_structure(tree.root().unwrap(), &[13, 12, 15, 14, 20, 16, 30]);
}

#[test]
fn splay_zig_zig_right() {
    let mut tree = tree_from_slice(&[
        Some(13), Some(12), Some(15), None, None, Some(14), Some(20), None, None,
        Some(16), Some(30)
    ]).unwrap();
    let elem = tree.root_mut().unwrap().right_mut().unwrap().right_mut().unwrap(); // 20
    assert_eq!(elem.splay_type(), Some(SplayType::ZigZig));
    elem.splay().map(|r| tree.root = Some(r));
    check_tree_structure(tree.root().unwrap(), &[20, 15, 13, 12, 14, 16, 30]);
}

#[test]
fn splay_zig_zag_left() {
    let mut tree = tree_from_slice(&[
        Some(40), Some(30), Some(50), None, None, Some(45), Some(60),
        Some(44), Some(46)
    ]).unwrap();
    let elem = tree.root_mut().unwrap().right_mut().unwrap().left_mut().unwrap(); // 45
    assert_eq!(elem.splay_type(), Some(SplayType::ZigZag));
    elem.splay().map(|r| tree.root = Some(r));
    check_tree_structure(tree.root().unwrap(), &[45, 40, 30, 44, 50, 46, 60]);
}

#[test]
fn splay_zig_zag_right() {
    let mut tree = tree_from_slice(&[
        Some(40), Some(30), Some(50), Some(20), Some(35), None, None, None, None,
        Some(34), Some(36)
    ]).unwrap();
    let elem = tree.root_mut().unwrap().left_mut().unwrap().right_mut().unwrap(); // 35
    assert_eq!(elem.splay_type(), Some(SplayType::ZigZag));
    elem.splay().map(|r| tree.root = Some(r));
    check_tree_structure(tree.root().unwrap(), &[35, 30, 20, 34, 40, 36, 50]);
}
