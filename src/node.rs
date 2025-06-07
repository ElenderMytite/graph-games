use std::{cell::RefCell, collections::HashMap, rc::Rc};

#[derive(Debug, Clone)]
pub struct Node {
    pub name: String,
    pub edges: HashMap<String,Rc<RefCell<Node>>>,
}

impl Node {
    pub fn new(name: String, edges: HashMap<String,Rc<RefCell<Node>>>) -> Self {
        Self { name, edges }
    }
    
}