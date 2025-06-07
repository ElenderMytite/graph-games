use std::cell::RefCell;
use std::collections::HashMap;
use std::io::{self, Write as _};
use std::rc::Rc;
use crate::{graph::Graph, visitor::Visitor};

pub struct CommandLineEditor{
    graph: Rc<RefCell<Graph>>,
    visitors: HashMap<String, Visitor>,
    current_visitor: String,
}
impl CommandLineEditor {
    pub fn new() -> Self {
        let graph = Rc::new(RefCell::new(Graph::new()));
        graph.borrow_mut().add_node("#default".to_string());
        Self {
            visitors: HashMap::from([("#default".to_string(), Visitor::new(Rc::clone(&graph), "#default".to_string()))]),
            current_visitor: String::from("#default".to_string()),
            graph,
        }
    }
    
    pub fn pin(&mut self, name: String, visitor: Visitor) {
        self.visitors.insert(name, visitor);
    }
    
    pub fn go_to_pin(&mut self, name: String) {
        if self.visitors.contains_key(&name) {
            self.current_visitor = name;
        } else {
            println!("Visitor {} not found.", name);
        }
    }
    
    pub fn current_pin(&self) -> Visitor {
        self.visitors.get(&self.current_visitor).unwrap().clone()
    }
    pub fn run(&mut self) {
        loop {
            print!("> ");
            io::stdout().flush().unwrap();
            let mut command = String::new();
            io::stdin().read_line(&mut command).expect("Failed to read line");
            let mut command: Vec<&str> = command.trim().split(" ").collect();
            command.reverse();
            if self.execute_command(self.parse_command(&mut command))
            {
                break;
            }

        }
    }
    fn execute_command(&mut self, commands: Vec<Command>) -> bool {
        for command in commands{
        match command {
            Command::Exit => return true,
            Command::Node(name) => self.graph.borrow_mut().add_node(name),
            Command::Edge(n1, n2, edge_name) => self.graph.borrow_mut().connect(n1, n2, edge_name),
            Command::Pin(visitor_name, visitor) => self.pin(visitor_name, visitor),
            Command::Switch(visitor_name) => self.go_to_pin(visitor_name),
            Command::Current => {
                let visitor = self.current_pin();
                visitor.print_current();
            },
            Command::ListNeighbours => {
                let visitor = self.current_pin();
                visitor.show_neighbours();
            },
            Command::None => (),
            Command::Move(visitor_name, edge) => {
                let mut visitor = self.current_pin();
                visitor.move_to(edge);
                self.pin(visitor_name, visitor);
            },
        }}
        false
    }
    fn parse_command(&self, command: &mut Vec<&str>) -> Vec<Command> {
        let mut commands = vec![];
        commands.push(match command.pop().unwrap_or("").trim() {
            "ex" | "exit" => Command::Exit,
            "nd" | "node" => {
                let name = self.parse_node_name(command);
                Command::Node(name.trim().to_string())
            },
            "eg" | "edge" => {
                let n1 = self.parse_node_name(command);
                let n2 = self.parse_node_name(command);
                let edge_name = command.pop().unwrap_or("placeholder").trim();
                Command::Edge(n1.trim().to_string(), n2.trim().to_string(), edge_name.trim().to_string())
            },
            "pn" | "pin" => {
                let visitor_name = command.pop().unwrap_or("visitor").trim();
                let node_name = self.parse_node_name(command); 
                let visitor = Visitor::new(self.current_pin().place, node_name.trim().to_string());
                Command::Pin(visitor_name.trim().to_string(), visitor)
            },
            "sw" | "swich"  => {
                let visitor_name = command.pop().unwrap_or("visitor").trim();
                Command::Switch(visitor_name.trim().to_string())
            },
            "cr" | "current" => {
                Command::Current
            },
            "ls" | "listneighbours" => {
                Command::ListNeighbours
            },
            "la" | "listall" =>{
                self.graph.borrow().nodes.iter().for_each(|(name, node)| {
                    println!("Node: {}", name);
                    for (edge_name, neighbour) in node.borrow().edges.iter() {
                        println!("  Edge: {}, Neighbour: {}", edge_name, neighbour.borrow().name);
                    }
                });
                Command::None
            }
            "lv" | "listvisitors" => {
                self.visitors.iter().for_each(|(name, visitor)| {
                    println!("Visitor: {}, Current Node: {}", name, visitor.current);
                });
                Command::None
            },
            "mv" | "move" => {
                let edge = command.pop().unwrap_or("way").trim();
                let visitor_name = command.pop().unwrap_or("visitor").trim();
                Command::Move(visitor_name.trim().to_string(), edge.trim().to_string())
            },
            "" => Command::None,
            _ => {println!("Unknown command: {:?}", command); Command::None},
        });
        commands
    }
    fn parse_node_name(&self, command: &mut Vec<&str>) -> String {
        if let Some(name) = command.pop() {
            if name.starts_with("@") {
                return self.visitors.get(&name[1..])
                    .map_or_else(|| {
                        println!("Visitor {} not found.", name);
                        String::new()
                    }, |v| v.current.clone());
            }
            name.to_string()
        } else {
            String::new()
        }
    }
}
enum Command {
    Exit,
    Node(String),
    Edge(String, String, String),
    Pin(String, Visitor),
    Switch(String),
    Move(String, String),
    Current,
    ListNeighbours,
    None,
}