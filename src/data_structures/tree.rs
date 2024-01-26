#![allow(dead_code)]

pub type Link = Option<Box<Node>>;
#[derive(Debug)]
pub struct Node {
    val: i32,
    left: Link,
    right: Link,
}

impl Node {
    fn new(val: i32) -> Box<Self> {
        Box::new(Self {
            val,
            left: None,
            right: None,
        })
    }
    pub fn val(&self) -> &i32 {
        &self.val
    }
    pub fn left(&self) -> &Link {
        &self.left
    }
    pub fn right(&self) -> &Link {
        &self.right
    }
}

/// ```
/// // For now you can only create a minimal_bstree
/// // Create a tree:
/// use ctci_rust::data_structures::tree::BSTree;
/// let tree = BSTree::minimal_bstree_from_slice(&[1,2,3,4,5]);
/// ```
#[derive(Debug)]
pub struct BSTree {
    pub root: Link,
}

impl BSTree {
    pub fn minimal_bstree_from_slice(v: &[i32]) -> Self {
        // make sure vector is sorted
        let mut v = v.to_owned();
        v.sort();
        Self {
            root: Self::recursive_make_tree_nodes(&v),
        }
    }

    fn recursive_make_tree_nodes(v: &[i32]) -> Link {
        if v.len() == 0 {
            return None;
        }
        if v.len() == 1 {
            let root = Node::new(v[0]);
            return Some(root);
        }

        if v.len() == 2 {
            let mut root = Node::new(v[0]);
            let right = Node::new(v[1]);
            root.right = Some(right);
            return Some(root);
        }

        let mid_index = v.len() / 2;
        dbg!(&v, &mid_index);

        let mut root = Node::new(v[mid_index]);
        let left = Self::recursive_make_tree_nodes(&v[0..mid_index]);
        let right = Self::recursive_make_tree_nodes(&v[(mid_index+1)..]);
        root.left = left;
        root.right = right;

        Some(root)
    }

    // pub fn print(&self) {
    //     fn _recursive_print(root: &Link) {
    //         if let Some(node) = root {
    //             print!("{}", node.val);
    //             _recursive_print(&node.left);
    //             _recursive_print(&node.right);
    //         } else {
    //             print!("*");
    //         }
    //         print!("\n");
    //     }
    //
    //     _recursive_print(&self.root);
    // }
}



