mod cli;
mod gui;

mod graph;
use bevy::prelude::*;
use graph::*;
use std::collections::HashMap;
use std::env::{self};
fn get_position_on_circle(radius: f32, angle: f32) -> Vec3 {
    Vec3::new(radius * angle.cos(), radius * angle.sin(), 0.)
}
fn main() {
    const LEN: usize = 12;
    let mut graph = Graph::new(
        LEN,
        StoreMethod::EdgesOfNodes,
        HashMap::new(),
    );
    graph.add_walk(&[0,1,2,3,4,5,0], false);
    for i in 0..LEN {
        graph.insert_data(
            i,
            NodeData {
                name: format!("{}", i),
                position: Some(get_position_on_circle(
                    300.,
                    std::f32::consts::PI * 2. / LEN as f32 * i as f32,
                )),
                ..Default::default()
            },
        );
    }
    test_store_method(&mut graph);
    let index = 0;
    let args = env::args();
    let args_vec: Vec<String> = args.collect();

    if args_vec.contains(&"cli".to_string()) {
        if args_vec.contains(&"--jump".to_string()) || args_vec.contains(&"-j".to_string()) {
            cli::console_mainloop(&mut graph, index, true);
        } else {
            cli::console_mainloop(&mut graph, index, false);
        }
    } else if args_vec.contains(&"gui".to_string()) {
        App::new()
            .insert_resource(graph)
            .add_plugins(DefaultPlugins)
            .add_systems(Startup, gui::start)
            .add_systems(Update, gui::handle_input)
            .run();
    } else {
        println!(
            "Run with 'cli' argument to enter console mode or 'gui' argument to enter GUI mode."
        );
    }
}
fn test_store_method(graph: &mut Graph) {
    graph.switch_store_method(graph::StoreMethod::EdgeSet);
    assert_eq!(graph.store_method(), graph::StoreMethod::EdgeSet);
    graph.switch_store_method(graph::StoreMethod::EdgesOfNodes);
    assert_eq!(graph.store_method(), graph::StoreMethod::EdgesOfNodes);
}
