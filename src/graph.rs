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
    pub fn add_node(&mut self, name: String) {
        let node = Rc::new(RefCell::new(Node::new(name.clone(), HashMap::new())));
        self.nodes.insert(name, node);
    }
    pub fn connect(&mut self, n1: String, n2: String, name: String) {
        if let (Some(node1), Some(node2)) = (self.nodes.get(&n1), self.nodes.get(&n2)) {
            node1.borrow_mut().edges.insert(name.clone(), Rc::clone(node2));
            node2.borrow_mut().edges.insert(name, Rc::clone(node1));
        }
        else {
            println!("One or both nodes: {}, {} not found in the graph.", n1, n2);
        }
    }
}