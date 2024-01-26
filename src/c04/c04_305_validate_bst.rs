#![allow(dead_code)]

use crate::data_structures::tree::{BSTree, Link};

fn validate_bst_in_order_traversal(tree: &BSTree) -> bool {
    fn check_bst(node: &Link, last: &mut Option<i32>) -> bool {
        match node {
            Some(node) => {
                if !check_bst(node.left(), last) {
                    return false;
                }

                println!("traverse {}", node.val());
                if let Some(ref last_value_printed) = last {
                    if last_value_printed > &node.val() {
                        return false;
                    }
                }
                *last = Some(*node.val());

                if !check_bst(node.right(), last) {
                    return false;
                }

                true // all good
            }
            None => true,
        }
    }

    let mut last_printed: Option<i32> = None;
    check_bst(&tree.root, &mut last_printed)
}

fn validate_bst(tree: &BSTree) -> bool {
    // root node must be greater or equal to left
    // greater than right
    // !! all left nodes must be smaller that root
    // !! all right nodes must be greater that root

    fn check_bst(node: &Link, min: i32, max: i32) -> bool {
        match node {
            Some(node) => {
                let cur_value = *node.val();
                if cur_value <= min || cur_value > max {
                    return false;
                }

                return !check_bst(node.left(), cur_value, max)
                    || !check_bst(node.right(), min, cur_value);
            }
            None => true, // base case
        }
    }

    check_bst(&tree.root, std::i32::MIN, std::i32::MAX)
}

#[test]
fn test_validate_bst() {
    let tree = BSTree::minimal_bstree_from_slice(&[1, 2, 3, 4, 5, 6]);
    assert!(validate_bst(&tree));
}

#[test]
fn test_validate_bst_in_order_traversal() {
    let tree = BSTree::minimal_bstree_from_slice(&[1, 2, 3, 4, 5, 6]);
    assert!(validate_bst_in_order_traversal(&tree));
}
