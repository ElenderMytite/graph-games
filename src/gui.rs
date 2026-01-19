use crate::graph::{self, Graph};
use bevy::prelude::*;
const XSIZE: f32 = 1200.;
const NODE_RADIUS: f32 = 30.;
const EDGE_THICKNESS: f32 = 7.5;
pub fn start(
    mut commands: Commands,
    graph: Res<Graph>,
    meshes: ResMut<Assets<Mesh>>,
    materials: ResMut<Assets<ColorMaterial>>,
) {
    commands.spawn(Camera2d::default());
    commands.spawn((
        Text::new("Welcome."),
        Node {
            position_type: PositionType::Absolute,
            top: px(12),
            left: px(12),
            ..default()
        },
    ));

    draw_graph(&graph, &mut commands, meshes, materials);
}
pub fn handle_input(mut _graph: ResMut<Graph>) {
    // Handle keyboard input for GUI interactions
}
fn draw_graph(
    graph: &Graph,
    commands: &mut Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    for i in 0..graph.length {
        let default_pos = Vec3::X * XSIZE / (graph.length - 1) as f32 * i as f32 - XSIZE / 2.;
        let node_data: graph::NodeData = graph.data.get(&i).unwrap_or(&Default::default()).clone();
        commands.spawn((
            // fill circle
            Mesh2d(meshes.add(Circle::new(node_data.size.unwrap_or(NODE_RADIUS)))),
            MeshMaterial2d(materials.add(node_data.fill_color.unwrap_or(Color::WHITE))),
            Transform::from_translation(node_data.position.unwrap_or(default_pos) - Vec3::Z * 0.1),
            children![(
                // outline ring
                Mesh2d(meshes.add(Annulus::new(
                    node_data.size.unwrap_or(NODE_RADIUS) - EDGE_THICKNESS,
                    node_data.size.unwrap_or(NODE_RADIUS),
                ))),
                MeshMaterial2d(materials.add(node_data.outline_color.unwrap_or(Color::BLACK))),
                Transform::from_translation(Vec3::ZERO),
            )],
        ));
    }
    // Drawing edges
    for i in 0..graph.length {
        let default_pos_i = Vec3::X * XSIZE / (graph.length - 1) as f32 * i as f32 - XSIZE / 2.;
        let neighbors = graph.get_adjacent_nodes(i);
        let node_data_i: graph::NodeData =
            graph.data.get(&i).unwrap_or(&Default::default()).clone();
        let pos_i = node_data_i.position.unwrap_or(default_pos_i);
        for &neighbor in &neighbors {
            let default_pos_j =
                Vec3::X * XSIZE / (graph.length - 1) as f32 * neighbor as f32 - XSIZE / 2.;
            let node_data_j: graph::NodeData = graph
                .data
                .get(&neighbor)
                .unwrap_or(&Default::default())
                .clone();
            let pos_j = node_data_j.position.unwrap_or(default_pos_j);
            commands.spawn((
                Mesh2d(meshes.add(Segment2d::new(pos_i.truncate(), pos_j.truncate()))),
                MeshMaterial2d(materials.add(ColorMaterial::from(Color::srgb(0.5, 0.5, 0.5)))),
                Transform::from_translation(Vec3::Z * -0.2),
            ));
        }
    }
}
/*
                  EDGE_THICKNESS / 2.,
                   distance
                       - node_data_i.size.unwrap_or(NODE_RADIUS)
                       - node_data_j.size.unwrap_or(NODE_RADIUS),
*/
