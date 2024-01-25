#![allow(unused)]
use std::cmp::max;
use std::f32::MIN;

use crate::data_structures::tree::BSTree;
use crate::data_structures::tree::Link;

// a tree is balanced if all subtrees are balanced
fn check_balanced(tree: &BSTree) -> bool {
    fn get_height(node: &Link) -> i32 {
        match node {
            Some(node) => max(get_height(&node.left()), get_height(&node.right())) + 1,
            None => -1,
        }
    }

    match &tree.root {
        Some(node) => {
            let left_height = get_height(&node.left());
            let right_height = get_height(&node.right());

            let diff = left_height - right_height;
            dbg!(&left_height, &right_height, diff);
            diff.abs() <= 1
        }
        None => true, // ? empty tree is balanced
    }
}

// faster solution that returns earlier without computing
// other heights if one subtree isn't balanced

fn check_balanced_2(tree: &BSTree) -> bool {
    fn check_height(link: &Link) -> i32 {
        match link {
            Some(node) => {
                let left_height = check_height(node.left());
                if left_height == std::i32::MIN {
                    return std::i32::MIN;
                };
                let right_height = check_height(node.right());
                if right_height == std::i32::MIN {
                    return std::i32::MIN;
                };

                if left_height.abs_diff(right_height) > 1 {
                    return std::i32::MIN;
                } else {
                    return max(left_height, right_height) + 1;
                }
            }
            None => -1,
        }
    }

    match &tree.root {
        Some(node) => {
            let left_height = check_height(node.left());
            let right_height = check_height(node.right());
            if left_height == std::i32::MIN || right_height == std::i32::MIN {
                return false;
            }

            left_height.abs_diff(right_height) <= 1
        }
        None => true,
    }
}

#[test]
fn test_check_balanced() {
    let tree = BSTree::minimal_bstree_from_slice(&[1, 2, 3, 4, 5, 6, 7, 8]);
    assert!(check_balanced(&tree));
}

#[test]
fn test_check_balanced_2() {
    let tree = BSTree::minimal_bstree_from_slice(&[1, 2, 3, 4, 5, 6, 7, 8]);
    assert!(check_balanced_2(&tree));
}
