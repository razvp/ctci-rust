#![allow(dead_code)]
/*
Route Between Nodes

Given a directed graph, design an algorithm to
find out whether there is a route between two nodes.
*/

// We should use bfs

use std::{rc::Rc, cell::RefCell, fmt::Display, collections::{VecDeque, HashSet}, hash::Hash};
type NodeRef = Rc<RefCell<Node>>;

#[derive(Debug,PartialEq, Eq)]
struct Node {
    adjacent: Vec<NodeRef>,
    vertex: String,
}
impl Node {
    pub fn new(s: &str) -> NodeRef {
        Rc::new(RefCell::new(Self {
            adjacent: Vec::new(),
            vertex: s.to_owned(),
        }))
    }

    pub fn add_adjacent(&mut self, n: NodeRef) {
        self.adjacent.push(n); // equivalent to n.clone() ?
    }
}

#[derive(Debug)]
struct Graph {
    vertices: Vec<NodeRef>,
}

impl Graph {
    pub fn new() -> Self {
        Self {vertices: Vec::new()}
    }
    pub fn add_node(&mut self, n: NodeRef) {
        self.vertices.push(n); 
    }
    pub fn get_nodes(&self) -> &Vec<NodeRef> {
        &self.vertices
    }
}

// TODO: doesn't work with cycles
impl Display for Graph {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for n in self.get_nodes() {
            let mut adjacent_nodes = String::new();
            let n = n.as_ref().borrow();
            for nref in n.adjacent.iter() {
                let count = Rc::strong_count(nref);
                adjacent_nodes.push_str(nref.as_ref().borrow().vertex.as_str());
                adjacent_nodes.push_str(format!("[{count}]").as_str());
                adjacent_nodes.push(' ');
            }
            adjacent_nodes.push('\n');
            write!(f, "{} -> {}\n", n.vertex, adjacent_nodes)?;
        }
        Ok(())
    }
}

// fn route_between_nodes(_g: &Graph, n1: &NodeRef, n2: &NodeRef) {
//     // We actually don't need the Graph
//     let mut queue = VecDeque::new();
//     let mut set = HashSet::new();
//     queue.push_back(n1);
//
//     while let Some(node) = queue.pop_front() {
//         set.insert(node);
//     }
// }

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_graph_structure() {
        let mut g = Graph::new();
        let dummy = Node::new("dummy");
        let mut temp = vec![dummy; 6];
        temp[0] = Node::new("a");
        temp[1] = Node::new("b");
        temp[2] = Node::new("c");
        temp[3] = Node::new("d");
        temp[4] = Node::new("e");
        temp[5] = Node::new("f");

        temp[0].as_ref().borrow_mut().add_adjacent(temp[1].clone());
        temp[0].as_ref().borrow_mut().add_adjacent(temp[2].clone());
        temp[0].as_ref().borrow_mut().add_adjacent(temp[3].clone());
        temp[3].as_ref().borrow_mut().add_adjacent(temp[4].clone());
        temp[4].as_ref().borrow_mut().add_adjacent(temp[5].clone());
        // temp[2].as_ref().borrow_mut().add_adjacent(temp[3].clone());

        dbg!(&temp[0]);
        for node in temp {
            g.add_node(node);
        }

        dbg!(&g);
        println!("{g}");

        assert!(false);
    }
}



