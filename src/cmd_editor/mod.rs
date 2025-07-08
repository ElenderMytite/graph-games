use std::cell::RefCell;
use std::collections::HashMap;
use std::io::{self, Write as _};
use std::rc::Rc;
mod lexer;
mod executer;
mod parser;
use lexer::Token;

use crate::{graph::Graph, pin::Pin};

pub struct CommandLineEditor{
    graph: Rc<RefCell<Graph>>,
    pins: HashMap<String, Pin>,
    current_pin: Option<String>,
    placeholder: usize,
}
impl CommandLineEditor {
    pub fn new() -> Self {
        let graph = Rc::new(RefCell::new(Graph::new()));
        Self {
            pins: HashMap::new(),
            current_pin: None,
            graph,
            placeholder: 0,
        }
    }
    pub fn run(&mut self) {
        loop {
            print!("/");
            io::stdout().flush().unwrap();
            let mut command = String::new();
            io::stdin().read_line(&mut command).expect("Failed to read line");
            let mut command: Vec<Token> = self.tokenize_command(command);
            let executable_command = self.parse_command(&mut command);
            if self.execute_command(executable_command)
            {
                break;
            }
        }
    }
}