#![allow(dead_code)]

type Link = Option<Box<Node>>;
struct Node {
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
}

struct BSTree {
    root: Link,
}

impl BSTree {
    pub fn from(v: &[i32]) -> Self {
        todo!()
    }

    fn recursive_make_tree_nodes(v: &[i32]) -> Link {
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_() {}
}
