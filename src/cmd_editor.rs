use std::cell::RefCell;
use std::collections::HashMap;
use std::io::{self, Write as _};
use std::rc::Rc;
use crate::{graph::Graph, visitor::Visitor};

pub struct CommandLineEditor{
    graph: Rc<RefCell<Graph>>,
    visitors: HashMap<String, Visitor>,
    current_visitor: Option<String>,
    placeholder: usize,
}
impl CommandLineEditor {
    pub fn new() -> Self {
        let graph = Rc::new(RefCell::new(Graph::new()));
        Self {
            visitors: HashMap::new(),
            current_visitor: None,
            graph,
            placeholder: 0,
        }
    }
    
    pub fn pin(&mut self, name: String, visitor: Visitor) {
        self.visitors.insert(name, visitor);
    }
    pub fn next_free(&mut self) -> String
    {
        self.placeholder += 1;
        format!("{}", self.placeholder)
    }
    
    pub fn go_to_pin(&mut self, name: String) {
        if self.visitors.contains_key(&name) {
                self.current_visitor = Some(name);
        } else {
                println!("Visitor {} not found.", name);
        }
    }

    pub fn current_pin(&self) -> Result<Visitor,&str> {
        match self.current_visitor {
            Some(ref name) => {
                Ok(self.visitors[name].clone())
            },
            None => Err("No current visitor set."),
            
        }
    }
    pub fn run(&mut self) {
        loop {
            print!("> ");
            io::stdout().flush().unwrap();
            let mut command = String::new();
            io::stdin().read_line(&mut command).expect("Failed to read line");
            let mut command: Vec<String> = command.trim().split(" ").map(|s| s.to_string()).collect();
            command.reverse();
            let exe_command = self.parse_command(&mut command);
            if self.execute_command(exe_command)
            {
                break;
            }

        }
    }
    fn execute_command(&mut self, commands: Vec<Command>) -> bool {
        for command in commands{
        match command {
            Command::Exit => return true,
            Command::CreateNode(name) => self.graph.borrow_mut().add_node(name),
            Command::CreateEdge(n1, n2, edge_name) => self.graph.borrow_mut().connect(n1, n2, edge_name),
            Command::CreatePin(visitor_name, visitor) => self.pin(visitor_name, visitor),
            Command::RemoveNode(name) => {
                if self.graph.borrow_mut().nodes.remove(&name).is_some() {
                    println!("Node {} removed.", name);
                } else {
                    println!("Node {} not found.", name);
                }
            },
            Command::RemovePin(name) => {
                if self.visitors.remove(&name).is_some() {
                    println!("Visitor {} removed.", name);
                } else {
                    println!("Visitor {} not found.", name);
                }
            },
            Command::Switch(visitor_name) => self.go_to_pin(visitor_name),
            Command::Current => {
                match self.current_pin() {
                    Ok(visitor) => visitor.print_current(),
                    Err(e) => println!("Error from current pin: {}", e),
                }
            },
            Command::ListNeighbours => {
                match self.current_pin()
                {
                        Ok(visitor) => visitor.show_neighbours(),
                        Err(e) => println!("Error from list neighbours: {}", e),
                }
            },
            Command::None => (),
            Command::Move(visitor_name, edge) => {
                if let Some(visitor) = self.visitors.get_mut(&visitor_name) {
                    visitor.move_to(edge);
                }

            },
        }}
        false
    }
    fn parse_command(&mut self, command: &mut Vec<String>) -> Vec<Command> {
        let mut commands = vec![];
        commands.push(match command.pop().unwrap_or(String::from("")).trim() {
            "ex" | "exit" => Command::Exit,
            "nd" | "node" => {
                let name = self.parse_node_name(command);
                Command::CreateNode(name.trim().to_string())
            },
            "nr" | "noderemove" => {
                let name = command.pop().unwrap_or_default();
                Command::RemoveNode(name.trim().to_string())
            },
            "eg" | "edge" => {
                let n1 = self.parse_node_name(command);
                let n2 = self.parse_node_name(command);
                let edge_name: String = command.pop().unwrap_or_else(|| -> String {String::from("edge") + self.next_free().as_str()}).trim().to_string();
                Command::CreateEdge(n1.trim().to_string(), n2.trim().to_string(), edge_name.trim().to_string())
            },
            "pn" | "pin" => {
                let visitor_name = command.pop().unwrap_or_else(||-> String {String::from("visitor") + self.next_free().as_str()}).trim().to_string();
                let node_name = self.parse_node_name(command); 
                let visitor = Visitor::new(Rc::clone(&self.graph), Some(node_name.trim().to_string()));
                Command::CreatePin(visitor_name.to_string(), visitor)
            },
            "pr" | "pinremove" => {
                let name = command.pop().unwrap_or_default();
                Command::RemovePin(name.trim().to_string())
            },
            "sw" | "swich"  => {
                let visitor_name = command.pop().unwrap_or_else(||-> String {String::from("visitor") + self.next_free().as_str()}).trim().to_string();
                Command::Switch(visitor_name.trim().to_string())
            },
            "cr" | "current" => {
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
                self.visitors.iter().for_each(|(name, visitor)| {
                    println!("Visitor: {}, Current Node: {}", name, visitor.current.clone().unwrap_or("None".to_string()));
                });
                Command::None
            },
            "mv" | "move" => {
                let visitor_name = command.pop()
                .unwrap_or_else(||-> String {String::from("visitor") + self.next_free().as_str()})
                .trim()
                .to_string();
                let edge = command.pop()
                .unwrap_or_else(||-> String {String::from("edge") + self.next_free().as_str()})
                .trim().
                to_string();
                Command::Move(visitor_name.trim().to_string(), edge.trim().to_string())
            },
            "" => Command::None,
            _ => {println!("Unknown command: {:?}", command); Command::None},
        });
        commands
    }
    fn parse_node_name(&self, command: &mut Vec<String>) -> String {
        if let Some(name) = command.pop() {
            if name.starts_with("@") {
                return self.visitors.get(&name[1..])
                    .map_or_else(|| {
                        println!("Visitor {} not found.", name);
                        String::new()
                    }, |v| v.current.clone().unwrap());
            }
            name.to_string()
        } else {
            String::new()
        }
    }
}
enum Command {
    Exit,
    CreateNode(String),
    CreateEdge(String, String, String),
    CreatePin(String, Visitor),
    RemoveNode(String),
    RemovePin(String),
    Switch(String),
    Move(String, String),
    Current,
    ListNeighbours,
    None,
}