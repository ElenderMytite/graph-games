use std::{cell::RefCell, rc::Rc};

use crate::graph::Graph;

#[derive(Clone)]
pub struct Pin {
    pub place: Rc<RefCell<Graph>>,
    pub current: Option<String>,
}
impl Pin{
    pub fn new(place: Rc<RefCell<Graph>>, current: Option<String>) -> Self {
        Self{ place: place, current }
    }
    pub fn print_current(&self) {
        println!("Current node: {}", self.current.clone().unwrap_or("None".to_string()));
    }
    pub fn show_neighbours(&self) {

        if let Some(current) = self.current.clone() {        
            if let Some(node) = self.place.borrow().nodes.get(&current) {
            for (edge_name, neighbour) in node.borrow().edges.iter() {
                println!("Edge: {}, Neighbour: {}", edge_name, neighbour.borrow().name);
            }
            } else{
            println!("Node this visitor's node {} not found in the graph.", current);
            }
        }
    }
    pub fn move_to(&mut self, path: String) {
        if let Some(current) = self.current.clone() {
            if self.place.borrow().nodes.contains_key(&current) {
                self.current = Some(self.place.borrow().nodes[&current].borrow()
                .edges.get(&path)
                .unwrap_or(&(self.place.borrow().nodes[&self.current.clone().unwrap()])).borrow()
                .name.clone());
            } else {
                println!("Path {} doesn't exist for current node: {}.", path, current);
            }
        }
        else {
            println!("No current node set for this visitor.");
        }
    }
}