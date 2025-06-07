use std::{cell::RefCell, rc::Rc};

use crate::graph::Graph;

#[derive(Clone)]
pub struct Visitor {
    pub place: Rc<RefCell<Graph>>,
    pub current: String,
}
impl Visitor{
    pub fn new(place: Rc<RefCell<Graph>>, current: String) -> Self {
        Self{ place: place, current }
    }
    pub fn print_current(&self) {
        println!("Current node: {}", self.current);
    }
    pub fn show_neighbours(&self) {
        if let Some(node) = self.place.borrow().nodes.get(&self.current) {
            for (edge_name, neighbour) in node.borrow().edges.iter() {
                println!("Edge: {}, Neighbour: {}", edge_name, neighbour.borrow().name);
            }
        } else {
            println!("Node this visitor's node {} not found in the graph.", self.current);
        }
    }
    pub fn move_to(&mut self, path: String) {
        if self.place.borrow().nodes.contains_key(&self.current) {
            self.current = self.place.borrow().nodes[&self.current].borrow().edges.get(&path).unwrap().borrow().name.clone();
        } else {
            println!("Path {} doesn't exist for current node: {}.", path, self.current);
        }
    }
}