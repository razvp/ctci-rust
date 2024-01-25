#![allow(dead_code)]
/*
Route Between Nodes

Given a directed graph, design an algorithm to
find out whether there is a route between two nodes.
*/

// We should use bfs

use std::{
    cell::RefCell,
    collections::{HashSet, VecDeque},
    fmt::Display,
    hash::Hash,
    ops::Deref,
    rc::Rc,
};
#[derive(Debug, Clone, PartialEq, Eq)]
struct NodeRef(Rc<RefCell<Node>>);
impl Deref for NodeRef {
    type Target = Rc<RefCell<Node>>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl Hash for NodeRef {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        let borrow = &self.0.deref().borrow().vertex;
        borrow.hash(state)
    }
}

impl NodeRef {
    fn add_adjacent(&self, other: &Self) {
        self.as_ref().borrow_mut().add_adjacent(other.clone());
    }
}

#[derive(Debug, Hash, PartialEq, Eq)]
struct Node {
    adjacent: Vec<NodeRef>,
    vertex: String,
}
impl Node {
    pub fn new(s: &str) -> NodeRef {
        NodeRef(Rc::new(RefCell::new(Node {
            adjacent: Vec::new(),
            vertex: s.to_owned(),
        })))
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
        Self {
            vertices: Vec::new(),
        }
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
            let n = n.0.as_ref().borrow();
            for nref in n.adjacent.iter() {
                let count = Rc::strong_count(nref);
                adjacent_nodes.push_str(nref.0.as_ref().borrow().vertex.as_str());
                adjacent_nodes.push_str(format!("[{count}]").as_str());
                adjacent_nodes.push(' ');
            }
            adjacent_nodes.push('\n');
            write!(f, "{} -> {}\n", n.vertex, adjacent_nodes)?;
        }
        Ok(())
    }
}

fn route_between_nodes(_g: &Graph, n1: &NodeRef, n2: &NodeRef) -> bool {
    // We actually don't need the Graph
    let mut queue = VecDeque::new();
    let mut visited = HashSet::new();
    queue.push_back(n1.to_owned());

    while let Some(node) = queue.pop_front() {
        visited.insert(node.clone());
        // visited.insert(node.clone());
        if node == *n2 {
            return true;
        } else {
            for adj in node.0.as_ref().borrow().adjacent.iter() {
                if !visited.contains(&adj) {
                    let adj = adj.to_owned();
                    queue.push_back(adj);
                }
            }
        }
    }

    false
}

#[cfg(test)]
mod tests {

    use super::*;
    #[test]
    fn test_graph_structure_and_route_between_nodes() {
        let mut g = Graph::new();
        let dummy = Node::new("dummy");
        let mut temp = vec![dummy; 6];
        temp[0] = Node::new("a");
        temp[1] = Node::new("b");
        temp[2] = Node::new("c");
        temp[3] = Node::new("d");
        temp[4] = Node::new("e");
        temp[5] = Node::new("f");

        temp[0].add_adjacent(&temp[1]);
        temp[0].add_adjacent(&temp[2]);
        temp[0].add_adjacent(&temp[3]);
        temp[3].add_adjacent(&temp[4]);
        temp[4].add_adjacent(&temp[5]);

        for node in temp {
            g.add_node(node);
        }
        println!("{g}");

        let start = &g.get_nodes()[3];
        let end = &g.get_nodes()[5];
        assert_eq!(route_between_nodes(&g, &start, &end), true);
    }
}
