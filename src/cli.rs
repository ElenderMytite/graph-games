use std::io::stdin;

use crate::graph::Graph;
enum Mode {
    Walk,
    Edit,
}
pub fn console_mainloop(graph: &mut Graph, mut index: usize, jumping: bool) {
    graph.display_adjacent_nodes(index);
    let mut mode = Mode::Walk;
    loop {
        println!("{}", "-".repeat(30));
        let mut input: String = String::from("");
        stdin().read_line(&mut input).unwrap();
        println!("{}", "-".repeat(30));
        let command = input.trim();

        if command == "exit" || command == "quit" {
            println!("Exiting...");
            break;
        }
        if command == "e" || command == "edit" {
            mode = Mode::Edit;
            println!("Switched to Edit mode.");
            continue;
        }

        if command == "w" || command == "walk" {
            println!("Switched to Walk mode.");
            mode = Mode::Walk;
            continue;
        }
        match mode {
            Mode::Walk => {
                let tokens = command.split_whitespace().collect::<Vec<&str>>();
                if tokens.len() == 1 {
                    match tokens[0].parse::<usize>() {
                        Ok(num) => {
                            if !(graph.check_walk(&[index, num], false) == 0 || jumping) {
                                println!(
                                    "Jumping disabled: no edge between {} and {}; ",
                                    index, num
                                );
                                continue;
                            }
                            if num < graph.length {
                                index = num;
                                graph.display_adjacent_nodes(index);
                            } else {
                                println!("Index out of bounds. Please enter a valid index.");
                            }
                        }
                        Err(_) => {
                            println!(
                                "Invalid input. Please enter a valid index or 'exit' or 'quit' to quit."
                            );
                        }
                    }
                } else {
                    graph.display_adjacent_nodes(index);
                    continue;
                }
            }
            Mode::Edit => {
                let tokens = command.split_whitespace().collect::<Vec<&str>>();
                let nodes = tokens[1..]
                    .iter()
                    .map(|arg0: &&str| str::parse::<usize>(*arg0))
                    .collect::<Result<Vec<usize>, _>>();
                if nodes.is_err() {
                    println!("Invalid node indices.");
                    continue;
                }
                match tokens[0] {
                    "d" | "del" | "delete" => {
                        graph.remove_walk(&nodes.unwrap(), false);
                    }
                    "a" | "add" => {
                        graph.add_walk(&nodes.unwrap(), false);
                    }
                    _ => {
                        println!("Unknown command.");
                    }
                }
            }
        }
    }
}
