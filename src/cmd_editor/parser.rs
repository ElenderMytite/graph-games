use std::rc::Rc;

use crate::pin::Pin;

use super::{executer::Command, lexer::Token, CommandLineEditor};

impl CommandLineEditor {
    pub fn parse_command(&mut self, command: &mut Vec<Token>) -> Vec<Command> {
        let mut commands = vec![];
        commands.push(
            match command.pop().unwrap_or(Token::Alphanumeric(String::from(""))) {
                Token::Alphanumeric(cmd) => match cmd.as_str() 
                    {
                        "ex" | "exit" | "esc" | "escape" => Command::Exit,
                        "cn" | "node" => {
                            let name = self.parse_node_name(command);
                            Command::CreateNode(name.trim().to_string())
                        },
                        "rn" | "removenode" => {
                            let name = command.pop().unwrap_or_default();
                            Command::RemoveNode(name.trim().to_string())
                        },
                        "ce" | "edge" | "createedge" => {
                            let n1 = self.parse_node_name(command);
                            let n2 = self.parse_node_name(command);
                            let edge_name: String = command.pop().unwrap().to_string();
                            Command::CreateEdge(n1.trim().to_string(), n2.trim().to_string(), edge_name.trim().to_string())
                        },
                        "re" | "removeedge" => {
                            let node_name = self.parse_node_name(command);
                            let edge_name: String = command.pop().unwrap().to_string();
                            Command::RemoveEdge(node_name.trim().to_string(), edge_name.trim().to_string())
                        },
                        "cp" | "pin" | "createpin" => {
                            let visitor_name = command.pop().unwrap().to_string();
                            let node_name = self.parse_node_name(command); 
                            let visitor = Pin::new(Rc::clone(&self.graph), Some(node_name.trim().to_string()));
                            Command::CreatePin(visitor_name.to_string(), visitor)
                        },
                        "rp" | "removepin" => {
                            let name = command.pop().unwrap_or_default();
                            Command::RemovePin(name.trim().to_string())
                        },
                        "sw" | "swich"  => {
                            let visitor_name = command.pop().unwrap().to_string();
                            Command::Switch(visitor_name.trim().to_string())
                        },
                        "sc" | "current" => {
                            Command::Current
                        },
                        "ls" | "listneighbours" => {
                            Command::ListNeighbours
                        },
                        "la" | "listall" =>{
                            self.graph.borrow().nodes.iter().for_each(|(name,_)| {
                                println!("Node: {}", name);
                            });
                            Command::None
                        }
                        "lv" | "listvisitors" => {
                            self.pins.iter().for_each(|(name, visitor)| {
                                println!("Visitor: {}, Current Node: {}", name, visitor.current.clone().unwrap_or("None".to_string()));
                            });
                            Command::None
                        },
                        "mv" | "move" => {
                            let visitor_name = command.pop().unwrap().to_string();
                            let edge = command.pop().unwrap().to_string();
                            Command::Move(visitor_name.trim().to_string(), edge.trim().to_string())
                        },
                        "" => Command::None,
                        _ => {println!("Unknown command: {:?}", command); Command::None},
                },
                Token::Adress(addr) => todo!("Handle address commands: {:?}", addr),
            }
            );
        commands
    }
    fn parse_node_name(&mut self, command: &mut Vec<Token>) -> String {
        if let Some(name) = command.pop() {
            match name {
                Token::Alphanumeric(name) => name,
                Token::Adress(addr) => match self.pins.get(&addr) {
                    Some(pin) => pin.current.clone().unwrap_or_else(|| addr),
                    None => addr,
                },
            }
        } else {
            String::from("node") + self.next_free().as_str()
        }
    }
}