use rand::Rng;
use wasm_bindgen::prelude::*;

use crate::graph::edge::Edge;
use crate::graph::node::Node;
use crate::math::vector_2d::Vector2D;
use crate::simulation::force_simulation::ForceSimulation;

mod graph;
mod math;
mod simulation;

fn get_nodes() -> vec![Node] {
    // Set up random number generator
    let mut rng = rand::thread_rng();

    // Create 10 nodes with random positions, but start them with no velocity
    let mut nodes = vec![];
    for i in 0..10 {
        nodes.push(
            Node::new()
                .id(i + 1)
                .label(&format!("Node {}", i + 1))
                .position(Vector2D::new(
                    rng.gen_range(-1.0, 1.0),
                    rng.gen_range(-1.0, 1.0),
                )) // random position - between -1 and 1
                .velocity(Vector2D::new(0.0, 0.0)) // start with no velocity
                .build(),
        );
    }
    nodes
}

fn get_edges(nodes: &Vec<Node>) -> vec![Edge] {
    // Set up random number generator
    let mut rng = rand::thread_rng();

    // Put an edge between each pair of nodes (eg fully connected graph) with a random weight
    let mut edges = vec![];
    for i in 0..nodes.len() {
        for j in i + 1..nodes.len() {
            edges.push(
                Edge::new()
                    .node1_idx(i)
                    .node2_idx(j)
                    .weight(rng.gen_range(0.5, 1.5)) // random weight between 0.5 and 1.5
                    .build(),
            );
        }
    }
    edges
}

#[wasm_bindgen]
pub fn build_simulation() -> ForceSimulation {
    let nodes = test_nodes();
    let edges = test_edges(&nodes);
    // Your simulation logic here
    ForceSimulation::new(nodes, edges, 1, 1, 1)
}

#[wasm_bindgen]
pub fn run_simulation_step() {
    // Your simulation logic here
}
