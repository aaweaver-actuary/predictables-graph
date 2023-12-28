use crate::graph::{edge::Edge, node::Node};
use crate::math::vector_2d::Vector2D;

use std::ops::{Add, Mul, Sub};

fn factorial(n: u64) -> u64 {
    (1..=n).product()
}

fn combination(n: u64, r: u64) -> u64 {
    factorial(n) / (factorial(r) * factorial(n - r))
}

#[derive(Debug, Clone)]
pub struct ForceSimulation {
    nodes: Vec<Node>,
    edges: Vec<Edge>,
    repulsion_constant: f64,
    attraction_constant: f64,
    distances: Vec<f64>,
    directions: Vec<f64>,
    repulsive_force: Vec<Vector2D<f64>>,
    attractive_force: Vec<Vector2D<f64>>,
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
        let repulsive_force: Vec<Vector2D<f64>> = vec![Vector2D::from_xy(0.0, 0.0); num_nodes];
        let attractive_force: Vec<Vector2D<f64>> = vec![Vector2D::from_xy(0.0, 0.0); num_nodes];
        let total_forces: Vec<Vector2D<f64>> = vec![Vector2D::from_xy(0.0, 0.0); num_nodes];

        let mut fs = ForceSimulation {
            nodes,
            edges,
            repulsion_constant,
            attraction_constant,
            distances,
            directions,
            repulsive_force,
            attractive_force,
            total_forces,
        };

        fs.update_node_mass();
        fs
    }

    fn repulsive_force_n1_exerts_on_n2(
        &self,
        distance: f64,
        n1_mass: f64,
        n2_mass: f64,
        direction: f64,
    ) -> Vector2D<f64> {
        let magnitude = self.repulsion_constant * n1_mass * n2_mass / distance.powi(2);
        Vector2D::from_rtheta(magnitude, direction)
    }

    fn attractive_force_n1_exerts_on_n2(
        &self,
        distance: f64,
        weight: f64,
        direction: f64,
    ) -> Vector2D<f64> {
        let magnitude = self.attraction_constant * weight / distance.powi(2);
        Vector2D::from_rtheta(magnitude, direction)
    }

    fn total_force_n1_exerts_on_n2(&self, n1: &Node, n2: &Node, weight: f64) -> Vector2D<f64> {
        let distance = n1.position.distance(&n2.position).max(1e-5); // Avoid division by zero
        let direction = n1.position.relative_to(&n2.position).angle();
        let repulsive_force =
            self.repulsive_force_n1_exerts_on_n2(distance, n1.mass, n2.mass, direction);
        let attractive_force = self.attractive_force_n1_exerts_on_n2(distance, weight, direction);
        repulsive_force + attractive_force
    }

    // fn repulsive_force_n1_exerts_on_n2(&self, n1: &Node, n2: &Node) -> Vector2D<f64> {
    // 	let distance = n1.position.distance(&n2.position);
    // 	let n1_mass = n1.mass;
    // 	let n2_mass = n2.mass;
    // 	let direction = n1.position.relative_to(&n2.position).angle();

    // 	let magnitude = self.repulsion_constant * n1_mass * n2_mass / distance.powi(2);
    // 	Vector2D::from_rtheta(magnitude, direction)
    // }

    // fn attractive_force_n1_exerts_on_n2(&self, n1: &Node, n2: &Node, weight: f64) -> Vector2D<f64> {
    // 	let distance = n1.position.distance(&n2.position);
    // 	let direction = n1.position.relative_to(&n2.position).angle();

    // 	let magnitude = self.attraction_constant * weight / distance.powi(2);
    // 	Vector2D::from_rtheta(magnitude, direction)
    // }

    // fn total_force_n1_exerts_on_n2(&self, n1: &Node, n2: &Node, weight: f64) -> Vector2D<f64> {
    // 	let repulsive_force = self.repulsive_force_n1_exerts_on_n2(n1, n2);
    // 	let attractive_force = self.attractive_force_n1_exerts_on_n2(n1, n2, weight);
    // 	repulsive_force + attractive_force
    // }

    fn acceleration_from_force_n1_exerts_on_n2(
        &self,
        n1: &Node,
        n2: &Node,
        weight: f64,
    ) -> Vector2D<f64> {
        let total_force = self.total_force_n1_exerts_on_n2(n1, n2, weight);
        total_force / n2.mass
    }

    fn chg_in_velocity_from_force_n1_exerts_on_n2(
        &self,
        n1: &Node,
        n2: &Node,
        weight: f64,
        delta_time: f64,
    ) -> Vector2D<f64> {
        let acceleration = self.acceleration_from_force_n1_exerts_on_n2(n1, n2, weight);
        n2.velocity + acceleration * delta_time
    }

    fn chg_in_position_from_force_n1_exerts_on_n2(
        &self,
        n1: &Node,
        n2: &Node,
        weight: f64,
        delta_time: f64,
    ) -> Vector2D<f64> {
        let velocity = self.chg_in_velocity_from_force_n1_exerts_on_n2(n1, n2, weight, delta_time);
        n2.position + velocity * delta_time
    }

    pub fn get_edges(&self) -> &Vec<Edge> {
        &self.edges
    }

    pub fn get_nodes(&self) -> &Vec<Node> {
        &self.nodes
    }

    /// Update the nodes of the force simulation. Mass should be reset to be equal to the sum of
    /// the weights of the edges connected to the node.
    fn update_node_mass(&mut self) {
        for node_idx in 0..self.nodes.len() {
            let mass = self.get_node_mass(node_idx);
            self.nodes[node_idx].update_mass(mass);
        }
    }

    fn get_node_mass(&self, node_idx: usize) -> f64 {
        let mut total_mass: f64 = 0.0;
        for edge in &self.edges {
            if edge.node1_idx == node_idx || edge.node2_idx == node_idx {
                total_mass += edge.weight;
            }
        }
        total_mass
    }

    fn precompute_distances_and_directions(&mut self) {
        let mut edge_idx: usize = 0;
        for i in 0..(self.nodes.len() - 2) {
            for j in i + 1..(self.nodes.len() - i) {
                self.distances[edge_idx] = self.nodes[i].position.distance(&self.nodes[j].position);
                self.directions[edge_idx] = self.nodes[i]
                    .position
                    .relative_to(&self.nodes[j].position)
                    .angle();

                edge_idx += 1;
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
        let mut edge_idx: usize = 0;
        for i in 0..(self.nodes.len() - 2) {
            for j in i..(self.nodes.len() - i) {
                let magnitude: f64 = self.distances[edge_idx];
                let force_direction: f64 = self.directions[edge_idx];

                // Avoid division by very small numbers for stability
                if magnitude > 1e-5 {
                    let force_magnitude = self.repulsion_constant / magnitude.powi(2);
                    let force: Vector2D<f64> =
                        Vector2D::from_rtheta(force_magnitude, force_direction);

                    self.total_forces[i] -= force;
                    self.total_forces[j] += force;
                }

                edge_idx += 1;
            }
        }
    }

    fn apply_attractive_forces(&mut self) {
        let mut edge_idx: usize = 0;
        for edge in &self.edges {
            let distance = self.distances[edge_idx];
            let direction = self.directions[edge_idx];

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

        println!("node1={:?}", nodes[0]);
        println!("node2={:?}", nodes[1]);
        println!("node3={:?}", nodes[2]);
        println!("nodes={:?}", nodes);
        println!("=====================================\n\n");

        let edge1 = Edge::new(0, 1, 1.0);
        let edge2 = Edge::new(0, 2, 2.0);
        let edge3 = Edge::new(1, 2, 3.0);
        let edges: Vec<Edge> = vec![edge1, edge2, edge3];

        let n1_mass = 1.0 + 2.0;
        let n2_mass = 1.0 + 3.0;
        let n3_mass = 2.0 + 3.0;

        println!("edge1={:?}", edges[0]);
        println!("edge2={:?}", edges[1]);
        println!("edge3={:?}", edges[2]);
        println!("edges={:?}", edges);
        println!("=====================================\n\n");

        let mut force_simulation = ForceSimulation::new(nodes.clone(), edges, 1.0, 1.0);

        assert_eq!(force_simulation.get_node_mass(0), n1_mass);
        assert_eq!(force_simulation.get_node_mass(1), n2_mass);
        assert_eq!(force_simulation.get_node_mass(2), n3_mass);

        println!("node1={:?}", nodes[0]);
        println!("node2={:?}", nodes[1]);
        println!("node3={:?}", nodes[2]);
        println!("nodes={:?}", nodes);
        println!("=====================================\n\n");

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
