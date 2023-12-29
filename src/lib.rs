#![allow(unused_imports)]
#![allow(unused)]
#![allow(dead_code)]

use rand::Rng;
use wasm_bindgen::prelude::*;

use crate::graph::edge::Edge;
use crate::graph::node::Node;
use crate::math::vector_2d::Vector2D;
use crate::simulation::force_simulation::ForceSimulation;

mod graph;
mod math;
mod simulation;

fn get_nodes() -> Vec<Node> {
    // Set up random number generator
    let mut rng = rand::thread_rng();

    // Create 10 nodes with random positions, but start them with no velocity
    let mut nodes = vec![];
    for i in 0..10 {
        nodes.push(
            Node::new()
                .id(i + 1)
                .label(&format!("Node {}", i + 1))
                .position(Vector2D {
                    x: rng.gen_range(-1.0..1.0),
                    y: rng.gen_range(-1.0..1.0),
                }) // random position - between -1 and 1
                .velocity(Vector2D::new_at_origin()) // start with no velocity
                .build(),
        );
    }
    nodes
}

fn get_edges(nodes: &Vec<Node>) -> Vec<Edge> {
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
                    .weight(rng.gen_range(0.5..1.5)) // random weight between 0.5 and 1.5
                    .build(),
            );
        }
    }
    edges
}

#[wasm_bindgen]
pub fn build_simulation() -> String {
    let nodes = get_nodes();
    let edges = get_edges(&nodes);
    let sim = ForceSimulation::new(nodes, edges, 1.0, 1.0, 1.0);

    // let serial_nodes = serde_json::to_string(&nodes).unwrap();
    // let serial_edges = serde_json::to_string(&edges).unwrap();
    let serial_sim = serde_json::to_string(&sim).unwrap();

    serial_sim
}

#[wasm_bindgen]
pub fn run_simulation_step(sim: &str) -> String {
    let mut sim: ForceSimulation = serde_json::from_str(sim).unwrap();
    sim.step();

    serde_json::to_string(&sim).unwrap()
}
