mod cli;
mod graph;
use graph::*;
use std::collections::HashMap;
use std::iter::FromIterator;
use std::env::{self};
use bevy::prelude::*;
fn main() {
    let mut graph = Graph::new(
        5,
        StoreMethod::EdgesOfNodes,
        HashMap::from_iter([
            (0, "A".to_string()),
            (1, "B".to_string()),
            (2, "C".to_string()),
            (3, "D".to_string()),
            (4, "E".to_string()),
        ]),
    );
    let index = 0;
    let args = env::args(); 
    let args_vec: Vec<String> = args.collect();
    assert_store_method(&mut graph);
    if args_vec.contains(&"cli".to_string())
    {
        if args_vec.contains(&"--jump".to_string()) || args_vec.contains(&"-j".to_string()) {
            cli::console_mainloop(&mut graph, index, true);
        }
        else {            
            cli::console_mainloop(&mut graph, index, false);
        }
    }
    else if args_vec.contains(&"gui".to_string())
    {
        App::new()
            .insert_resource(graph)
            .add_plugins(DefaultPlugins)
            .add_systems(Startup, gui::start)
            .add_systems(Update, gui::handle_input)
            .run();
    }
    else {
        println!("Run with 'cli' argument to enter console mode or 'gui' argument to enter GUI mode.");
    }

}
fn assert_store_method(graph: &mut Graph, ) {
    graph.switch_store_type(graph::StoreMethod::EdgeSet);
    assert_eq!(graph.store_method(), graph::StoreMethod::EdgeSet);
    graph.switch_store_type(graph::StoreMethod::EdgesOfNodes);
    assert_eq!(graph.store_method(), graph::StoreMethod::EdgesOfNodes);
}
mod gui {
    use bevy::prelude::*;
    use crate::graph::Graph;
    pub fn start(mut commands: Commands, _graph: Res<Graph>) {
        commands.spawn(Camera2d::default());
        // spawn circle
        commands.spawn(Sprite {
            color: Color::srgb(0.5, 0.5, 1.0),
            custom_size: Some(Vec2::new(30.0, 30.0)),
            ..Default::default()
        });
        // Additional GUI setup can be done here
    }
    pub fn handle_input(mut _graph: ResMut<Graph>) {
        // Handle keyboard input for GUI interactions
    }
}
