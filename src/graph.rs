use std::collections::{HashMap, HashSet};
use bevy::{color::Color, math::Vec3, prelude::Resource};
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum StoreMethod {
    EdgeSet,
    EdgesOfNodes,
}
enum Edges {
    EdgesOfNodes(HashMap<usize, HashSet<usize>>),
    EdgeSet(HashSet<(usize, usize)>),
}
#[derive(Resource)]
pub struct Graph {
    pub data: HashMap<usize, NodeData>,
    pub length: usize,
    edges: Edges,
}
impl Graph {
    pub fn new(length: usize, store_method: StoreMethod, names: HashMap<usize, NodeData>) -> Self {
        let edges = match store_method {
            StoreMethod::EdgeSet => Edges::EdgeSet(HashSet::new()),
            StoreMethod::EdgesOfNodes => Edges::EdgesOfNodes(HashMap::new()),
        };
        Graph {
            length,
            edges,
            data: names,
        }
    }
    pub fn insert_data(&mut self, index: usize, data: NodeData) {
        if let Some(node_data) = self.data.get_mut(&index) {
            node_data.name = data.name;
            if data.size.is_some() {
                node_data.size = data.size;
            }
            if data.fill_color.is_some() {
                node_data.fill_color = data.fill_color;
            }
            if data.outline_color.is_some() {
                node_data.outline_color = data.outline_color;
            }
            if data.position.is_some() {
                node_data.position = data.position;
            }
        }
        else {
            self.data.insert(index, data);
        }
    }
    /// Checks if the given walk is valid in the graph. Returns amount of invalid edges found.
    pub fn check_walk(&self, walk: &[usize], cycle: bool) -> usize {
        if walk.is_empty() {
            return 0;
        }
        if cycle {
            let mut extended_walk = walk.to_vec();
            extended_walk.push(walk[0]);
            return self.check_walk(&extended_walk, false);
        }
        match &self.edges {
            Edges::EdgesOfNodes(adj_list) => {
                let mut invalid_edges = 0;
                for window in walk.windows(2) {
                    if let Some(neighbors) = adj_list.get(&window[0]) {
                        if !neighbors.contains(&window[1]) {
                            invalid_edges += 1;
                        }
                    } else {
                        invalid_edges += 1;
                    }
                }
                invalid_edges
            }
            Edges::EdgeSet(edge_set) => {
                let mut invalid_edges = 0;
                for window in walk.windows(2) {
                    if !edge_set.contains(&(window[0], window[1])) {
                        invalid_edges += 1;
                    }
                }
                invalid_edges
            }
        }
    }
    /// Adds the edges from the given walk to the graph. If cycle is true, adds an edge from the last to the first vertex.
    pub fn add_walk(&mut self, walk: &[usize], cycle: bool) {
        if walk.is_empty() {
            return;
        }
        if cycle {
            let mut extended_walk = walk.to_vec();
            extended_walk.push(walk[0]);
            return self.add_walk(&extended_walk, false);
        }
        match &mut self.edges {
            Edges::EdgesOfNodes(adj_list) => {
                for window in walk.windows(2) {
                    if window[0] >= self.length || window[1] >= self.length {
                        println!(
                            "Vertex index out of bounds; skipping edge ({}, {})",
                            window[0], window[1]
                        );
                        continue;
                    }
                    adj_list
                        .entry(window[0])
                        .or_insert_with(HashSet::new)
                        .insert(window[1]);
                }
            }
            Edges::EdgeSet(edge_set) => {
                for window in walk.windows(2) {
                    if window[0] >= self.length || window[1] >= self.length {
                        println!(
                            "Vertex index out of bounds; skipping edge ({}, {})",
                            window[0], window[1]
                        );
                        continue;
                    }
                    edge_set.insert((window[0], window[1]));
                }
            }
        }
    }
    pub fn remove_walk(&mut self, walk: &[usize], cycle: bool) {
        if walk.is_empty() {
            return;
        }
        if cycle {
            let mut extended_walk = walk.to_vec();
            extended_walk.push(walk[0]);
            return self.remove_walk(&extended_walk, false);
        }
        match &mut self.edges {
            Edges::EdgesOfNodes(adj_list) => {
                for window in walk.windows(2) {
                    if let Some(neighbors) = adj_list.get_mut(&window[0]) {
                        neighbors.remove(&window[1]);
                    }
                }
            }
            Edges::EdgeSet(edge_set) => {
                for window in walk.windows(2) {
                    edge_set.remove(&(window[0], window[1]));
                }
            }
        }
    }
}
impl Graph {
    pub fn switch_store_method(&mut self, method: StoreMethod) {
        match (&self.edges, method) {
            (Edges::EdgesOfNodes(adj_list), StoreMethod::EdgeSet) => {
                let mut edge_set = HashSet::new();
                for (node, neighbors) in adj_list {
                    for &neighbor in neighbors {
                        edge_set.insert((*node, neighbor));
                    }
                }
                self.edges = Edges::EdgeSet(edge_set);
            }
            (Edges::EdgeSet(edge_set), StoreMethod::EdgesOfNodes) => {
                let mut adj_list: HashMap<usize, HashSet<usize>> = HashMap::new();
                for &(src, dst) in edge_set {
                    adj_list.entry(src).or_insert_with(HashSet::new).insert(dst);
                }
                self.edges = Edges::EdgesOfNodes(adj_list);
            }
            _ => {}
        }
    }
    pub fn store_method(&self) -> StoreMethod {
        match &self.edges {
            Edges::EdgesOfNodes(_) => StoreMethod::EdgesOfNodes,
            Edges::EdgeSet(_) => StoreMethod::EdgeSet,
        }
    }
}
impl Graph {
    pub fn get_adjacent_nodes(&self, index: usize) -> Vec<usize> {
        match &self.edges {
            Edges::EdgesOfNodes(adj_list) => {
                if let Some(neighbors) = adj_list.get(&index) {
                    neighbors.iter().cloned().collect()
                } else {
                    Vec::new()
                }
            }
            Edges::EdgeSet(edge_set) => edge_set
                .iter()
                .filter_map(|&(src, dst)| if src == index { Some(dst) } else { None })
                .collect(),
        }
    }
    pub fn display_adjacent_nodes(&self, index: usize) {
        match &self.edges {
            Edges::EdgesOfNodes(adj_list) => {
                if let Some(neighbors) = adj_list.get(&index) {
                    println!(
                        "Adjacent nodes for {}:",
                        self.data.get(&index).unwrap_or(&NodeData::from(index.to_string())).name
                    );
                    for neighbor in neighbors {
                        println!(
                            "{}: {}",
                            self.data.get(neighbor).unwrap_or(&NodeData::from("".to_string())).name,
                            neighbor
                        );
                    }
                } else {
                    println!("No adjacent nodes for {}", index);
                }
            }
            Edges::EdgeSet(edge_set) => {
                let neighbors: Vec<usize> = edge_set
                    .iter()
                    .filter_map(|&(src, dst)| if src == index { Some(dst) } else { None })
                    .collect();
                if !neighbors.is_empty() {
                    for neighbor in neighbors {
                        println!(
                            "{:?}",
                            self.data.get(&neighbor).unwrap_or(&NodeData::from(neighbor.to_string()))
                        );
                    }
                } else {
                    println!("No adjacent nodes for {}", index);
                }
            }
        }
    }
}
#[cfg(test)]
mod tests {
        use super::*;
        use std::collections::HashMap;
        use std::iter::FromIterator;
        #[test]
    fn test_graph_operations() {
        let mut graph = Graph::new(
            5,
            StoreMethod::EdgesOfNodes,
            HashMap::from_iter([
                (0, NodeData::from("A".to_string())),
                (1, NodeData::from("B".to_string())),
                (2, NodeData::from("C".to_string())),
                (3, NodeData::from("D".to_string())),
                (4, NodeData::from("E".to_string())),
            ]),
        );
        graph.add_walk(&[0, 1, 2], false);
        assert_eq!(graph.check_walk(&[0, 1, 2], false), 0);
        assert_eq!(graph.check_walk(&[0, 2], false), 1);
        graph.switch_store_method(StoreMethod::EdgeSet);
        assert_eq!(graph.store_method(), StoreMethod::EdgeSet);
        assert_eq!(graph.check_walk(&[0, 1, 2], false), 0);
        assert_eq!(graph.check_walk(&[0, 2], false), 1);
    }
}
#[derive(Debug, Clone, PartialEq, Default)]
pub struct NodeData
{
    pub name: String,
    pub size: Option<f32>,
    pub fill_color: Option<Color>,
    pub outline_color: Option<Color>,
    pub position: Option<Vec3>,
}
impl From<String> for NodeData {
    fn from(name: String) -> Self {
        NodeData {
            name,
            size: None,
            fill_color: None,
            position: None,
            outline_color: None,
        }
    }
}