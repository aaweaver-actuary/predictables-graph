use crate::graph::{edge::Edge, node::Node};
use crate::math::vector_2d::Vector2D;

use std::ops::{Add, Mul, Sub};

pub struct ForceSimulation {
    nodes: Vec<Node>,
    edges: Vec<Edge>,
    repulsion_constant: f64,
    attraction_constant: f64,
    distances: Vec<f64>,
    directions: Vec<f64>,
    total_forces: Vec<Vector2D<f64>>,
}

impl ForceSimulation {
    pub fn new(
        nodes: Vec<Node>,
        edges: Vec<Edge>,
        repulsion_constant: f64,
        attraction_constant: f64,
    ) -> Self {
        let num_nodes: usize = nodes.len();
        let num_elements: usize = num_nodes * (num_nodes - 1) / 2; // Half-matrix size
        let distances: Vec<f64> = vec![0.0; num_elements];
        let directions: Vec<f64> = vec![0.0; num_elements]; // Angle in radians
        let total_forces: Vec<Vector2D<f64>> = vec![Vector2D::from_xy(0.0, 0.0); num_nodes];

        ForceSimulation {
            nodes,
            edges,
            repulsion_constant,
            attraction_constant,
            distances,
            directions,
            total_forces,
        }
    }

    fn index_for_distance_direction(&self, i: usize, j: usize) -> usize {
        // Ensure i < j for upper triangular matrix
        let (i, j): (usize, usize) = if i < j { (i, j) } else { (j, i) };
        i * self.nodes.len() + j - (i * (i + 1) / 2)
    }

    fn precompute_distances_and_directions(&mut self) {
        for i in 0..self.nodes.len() {
            for j in i + 1..self.nodes.len() {
                let index: usize = self.index_for_distance_direction(i, j);
                self.distances[index] = self.nodes[i].position.distance(&self.nodes[j].position);
                self.directions[index] = self.nodes[i]
                    .position
                    .relative_to(&self.nodes[j].position)
                    .angle();
            }
        }
    }

    pub fn update(&mut self, delta_time: f64) {
        self.precompute_distances_and_directions();

        // Reset total forces
        for force in &mut self.total_forces {
            *force = Vector2D::from_xy(0.0, 0.0);
        }

        self.apply_repulsive_forces();
        self.apply_attractive_forces();

        for (node, force) in self.nodes.iter_mut().zip(self.total_forces.iter()) {
            node.velocity += *force / node.mass * delta_time;
            node.position += node.velocity * delta_time;
        }
    }

    fn apply_repulsive_forces(&mut self) {
        for i in 0..self.nodes.len() {
            for j in i + 1..self.nodes.len() {
                let index: usize = self.index_for_distance_direction(i, j);
                let magnitude: f64 = self.distances[index];
                let force_direction: f64 = self.directions[index];

                // Avoid division by very small numbers for stability
                if magnitude > 1e-5 {
                    let force_magnitude = self.repulsion_constant / magnitude.powi(2);
                    let force: Vector2D<f64> =
                        Vector2D::from_rtheta(force_magnitude, force_direction);

                    self.total_forces[i] -= force;
                    self.total_forces[j] += force;
                }
            }
        }
    }

    fn apply_attractive_forces(&mut self) {
        for edge in &self.edges {
            let index = self.index_for_distance_direction(edge.node1_idx, edge.node2_idx);
            let distance = self.distances[index];
            let direction = self.directions[index];

            let force_magnitude = self.attraction_constant * distance * edge.weight;
            let force: Vector2D<f64> = Vector2D::from_rtheta(force_magnitude, direction);

            self.total_forces[edge.node1_idx] += force;
            self.total_forces[edge.node2_idx] -= force;
        }
    }
}

#[cfg(test)]
pub mod test {
    use super::*;

    #[test]
    pub fn test_force_simulation() {
        let node1 = Node::new()
            .id(1)
            .label("Node 1")
            .position(Vector2D::from_xy(0.0, 0.0))
            .build();
        let node2 = Node::new()
            .id(2)
            .label("Node 2")
            .position(Vector2D::from_xy(1.0, 0.0))
            .build();
        let node3 = Node::new()
            .id(3)
            .label("Node 3")
            .position(Vector2D::from_xy(0.0, 1.0))
            .build();
        let nodes: Vec<Node> = vec![node1, node2, node3];

        let edge1 = Edge::new(0, 1, 1.0);
        let edge2 = Edge::new(0, 2, 1.0);
        let edge3 = Edge::new(1, 2, 1.0);
        let edges: Vec<Edge> = vec![edge1, edge2, edge3];

        let mut force_simulation = ForceSimulation::new(nodes, edges, 1.0, 1.0);

        for _ in 0..100 {
            force_simulation.update(1.0);
        }

        let node1_position = force_simulation.nodes[0].position;
        let node2_position = force_simulation.nodes[1].position;
        let node3_position = force_simulation.nodes[2].position;

        assert!(node1_position.distance(&node2_position) < 1e-5);
        assert!(node2_position.distance(&node3_position) < 1e-5);
        assert!(node3_position.distance(&node1_position) < 1e-5);
    }
}
