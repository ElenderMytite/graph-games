use std::{cell::RefCell, collections::HashMap, rc::Rc};
use crate::node::Node;

#[derive(Clone, Debug)]
pub struct Graph {
    pub nodes: HashMap<String,Rc<RefCell<Node>>>,
}
impl Graph {
    pub fn new() -> Self {
        Self {
            nodes: HashMap::new(),
        }
    }
    /// Adds a node to the graph with a given name.
    pub fn add_node(&mut self, name: String) {
        let node = Rc::new(RefCell::new(Node::new(name.clone(), HashMap::new())));
        self.nodes.insert(name, node);
    }
    /// Unoriented connect of two nodes by their names with an edge.
    pub fn add_edge(&mut self, n1: String, n2: String, name: String) {
        if let (Some(node1), Some(node2)) = (self.nodes.get(&n1), self.nodes.get(&n2)) {
            node1.borrow_mut().edges.insert(name.clone(), Rc::clone(node2));
            node2.borrow_mut().edges.insert(name, Rc::clone(node1));
        }
        else {
            println!("One or both nodes: {}, {} not found in the graph.", n1, n2);
        }
    }
    /// Removes a node from the graph by its name.
    pub fn remove_node(&mut self, name: &str) -> bool {
        if let Some(node) = self.nodes.remove(name) {
            // Remove all edges associated with this node
            for neighbor in node.borrow().edges.values() {
                neighbor.borrow_mut().edges.remove(name);
            }
            true
        } else {
            false
        }
    }
    /// Removes an edge from a node by its name.
    pub fn remove_edge(&mut self, node_name: &str, edge_name: &str) -> bool {
        if let Some(node) = self.nodes.get(node_name) {
            node.borrow_mut().edges.remove(edge_name).is_some() || 
            node.borrow().edges.get(edge_name).expect("Edge should exist")
            .borrow_mut().edges.remove(node_name).is_some()
        } else {
            println!("Node {} not found.", node_name);
            false
        }
    }
}