use crate::pin::Pin;

use super::CommandLineEditor;
impl CommandLineEditor {
    pub fn pin(&mut self, name: String, visitor: Pin) {
        self.pins.insert(name, visitor);
    }
    pub fn next_free(&mut self) -> String
    {
        self.placeholder += 1;
        format!("{}", self.placeholder)
    }
    
    pub fn go_to_pin(&mut self, name: String) {
        if self.pins.contains_key(&name) {
                self.current_pin = Some(name);
        } else {
                println!("Visitor {} not found.", name);
        }
    }

    pub fn current_pin(&self) -> Result<Pin,&str> {
        match self.current_pin {
            Some(ref name) => {
                Ok(self.pins[name].clone())
            },
            None => Err("No current visitor set."),
            
        }
    }
    pub fn execute_command(&mut self, commands: Vec<Command>) -> bool {
        for command in commands {
            match command {
                Command::Exit => return true,
                
                Command::CreateNode(name) => self.graph.borrow_mut().add_node(name),
                
                Command::CreateEdge(n1, n2, edge_name) => {
                    self.graph.borrow_mut().add_edge(n1, n2, edge_name)
                }
                Command::CreatePin(visitor_name, visitor) => self.pin(visitor_name, visitor),
                Command::RemoveNode(name) => {
                    if self.graph.borrow_mut().remove_node(&name) {
                        println!("Node {} removed.", name);
                    } else {
                        println!("Node {} not found.", name);
                    }
                }
                Command::RemoveEdge(node_name, edge_name) => {
                    if self.graph.borrow_mut().remove_edge(&node_name, &edge_name) {
                        println!("Edge {} removed from node {}.", edge_name, node_name);
                    } else {
                        println!("Edge {} not found in node {}.", edge_name, node_name);
                    }
                }
                Command::RemovePin(name) => {
                    if self.pins.remove(&name).is_some() {
                        println!("Visitor {} removed.", name);
                    } else {
                        println!("Visitor {} not found.", name);
                    }
                }
                Command::Switch(visitor_name) => self.go_to_pin(visitor_name),
                Command::Current => match self.current_pin() {
                    Ok(visitor) => visitor.print_current(),
                    Err(e) => println!("Error from current pin: {}", e),
                },
                Command::ListNeighbours => match self.current_pin() {
                    Ok(visitor) => visitor.show_neighbours(),
                    Err(e) => println!("Error from list neighbours: {}", e),
                },
                Command::None => (),
                Command::Move(visitor_name, edge) => {
                    if let Some(visitor) = self.pins.get_mut(&visitor_name) {
                        visitor.move_to(edge);
                    }
                }
            }
        }
        false
    }
}
pub enum Command {
    Exit,
    CreateNode(String),
    CreateEdge(String, String, String),
    CreatePin(String, Pin),
    RemoveNode(String),
    RemoveEdge(String, String),
    RemovePin(String),
    Switch(String),
    Move(String, String),
    Current,
    ListNeighbours,
    None,
}
