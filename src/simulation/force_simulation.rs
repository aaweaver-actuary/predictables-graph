use crate::graph::{edge::Edge, node::Node};
use crate::math::vector_2d::Vector2D;

use std::ops::{Add, Mul, Sub};

#[derive(Debug, Clone)]
pub struct ForceSimulation {
    nodes: Vec<Node>,
    edges: Vec<Edge>,
    positions: Vec<Vector2D<f64>>,
    velocities: Vec<Vector2D<f64>>,
    distances: Vec<Vec<f64>>,
    directions: Vec<Vec<f64>>,
    masses: Vec<f64>,
    time_step: f64,
    repulsion_constant: f64,
    attraction_constant: f64,
}

impl ForceSimulation {
    pub fn new(
        nodes: Vec<Node>,
        edges: Vec<Edge>,
        time_step: f64,
        repulsion_constant: f64,
        attraction_constant: f64,
    ) -> Self {
        let n_nodes: usize = nodes.len();
        let n_edges: usize = edges.len();

        let positions: Vec<Vector2D<f64>> = nodes.iter().map(|node| node.position).collect();
        let velocities: Vec<Vector2D<f64>> = nodes.iter().map(|node| node.velocity).collect();

        // Initialize distances and directions matrices with default values
        let n_nodes = nodes.len();
        let distances = vec![vec![0.0; n_nodes]; n_nodes];
        let directions = vec![vec![0.0; n_nodes]; n_nodes];

        let masses: Vec<f64> = nodes.iter().map(|node| node.mass).collect();

        ForceSimulation {
            nodes,
            edges,
            time_step,
            repulsion_constant,
            attraction_constant,
            positions,
            velocities,
            distances,
            directions,
            masses,
        }
    }

    fn acceleration_from_force_n1_exerts_on_n2(
        &self,
        n1: &Node,
        n2: &Node,
        weight: f64,
    ) -> Vector2D<f64> {
        let total_force = self.total_force_n1_exerts_on_n2(n1, n2, weight);
        total_force / n2.mass
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

    ///
    fn chg_in_position_from_force_n1_exerts_on_n2(
        &self,
        n1: &Node,
        n2: &Node,
        weight: f64,
        delta_time: f64,
    ) -> Vector2D<f64> {
        // inputs for basic kinematics equation
        let v0 = n2.velocity;
        let p0 = n2.position;
        let a = self.acceleration_from_force_n1_exerts_on_n2(n1, n2, weight);

        // basic kinematics equation
        let pf = p0 + (v0 * delta_time) + (a * delta_time.powi(2) / 2.0);

        // delta_p = pf - p0
        pf - p0
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

    fn get_edge_connecting_nodes(&self, node1_idx: usize, node2_idx: usize) -> Option<&Edge> {
        self.edges
            .iter()
            .find(|&edge| edge.has_node(node1_idx) && edge.has_node(node2_idx))
    }

    pub fn get_edges(&self) -> &Vec<Edge> {
        &self.edges
    }

    /// Returns a vector of edges that are connected to the node with the provided index.
    fn get_edges_by_node_idx(&self, node_idx: usize) -> Vec<&Edge> {
        let mut edges: Vec<&Edge> = Vec::new();
        for edge in &self.edges {
            if (edge.has_node(node_idx)) {
                edges.push(edge);
            }
        }
        edges
    }

    fn get_n_nodes(&self) -> usize {
        self.nodes.len()
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

    pub fn get_nodes(&self) -> &Vec<Node> {
        &self.nodes
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

    fn total_force_n1_exerts_on_n2(&self, n1: &Node, n2: &Node, weight: f64) -> Vector2D<f64> {
        let distance = n1.position.distance(&n2.position).max(1e-5); // Avoid division by zero
        let direction = n1.position.relative_to(&n2.position).angle();
        let repulsive_force =
            self.repulsive_force_n1_exerts_on_n2(distance, n1.mass, n2.mass, direction);
        let attractive_force = self.attractive_force_n1_exerts_on_n2(distance, weight, direction);
        attractive_force - repulsive_force
    }

    /// Updates the distances cache based on the current positions of the nodes.
    pub fn update_distances(&mut self) {
        for i in 0..self.nodes.len() {
            self.distances[i][i] = 0.0; // Distance to itself is always 0
            for j in (i + 1)..self.nodes.len() {
                let distance = self.nodes[i].position.distance(&self.nodes[j].position);
                // Since the distance is symmetrical, assign it to both [i][j] and [j][i]
                self.distances[i][j] = distance;
                self.distances[j][i] = distance;
            }
        }
    }

    /// Updates the directions cache based on the current positions of the nodes.
    pub fn update_directions(&mut self) {
        for i in 0..self.nodes.len() {
            self.directions[i][i] = 0.0; // Angle to itself is always 0
            for j in (i + 1)..self.nodes.len() {
                let angle = self.nodes[i]
                    .position
                    .relative_to(&self.nodes[j].position)
                    .angle();

                // Since the angle is anti-symmetrical, calculate for j > i and infer for j < i
                self.directions[i][j] = angle;
                // Normalize the angle to be within the range [0, 2π]
                self.directions[j][i] =
                    (angle + std::f64::consts::PI) % (2.0 * std::f64::consts::PI);
            }
        }
    }

    /// Updates the positions cache based on the current positions of the nodes.
    pub fn update_positions(&mut self) {
        for i in 0..self.nodes.len() {
            self.positions[i] = self.nodes[i].position;
        }
    }

    /// Updates the velocities cache based on the current velocities of the nodes.
    pub fn update_velocities(&mut self) {
        for i in 0..self.nodes.len() {
            self.velocities[i] = self.nodes[i].velocity;
        }
    }

    /// Updates the masses cache based on the current masses of the nodes.
    pub fn update_masses(&mut self) {
        for i in 0..self.nodes.len() {
            self.masses[i] = self.get_node_mass(i);
        }
    }

    /// Performs a single simulation step.
    pub fn step(&mut self) {
        self.update_distances();
        self.update_directions();
        self.apply_forces();
        self.update_positions_and_velocities();
    }

    /// Calculates all pairwise forces between nodes.
    fn calculate_forces(&mut self) {
        // Initialize a matrix of vectors to store the total forces that each node exerts on each
        // other node. The matrix is anti-symmetrical, so the force that node i exerts on node j is
        // the negative of the force that node j exerts on node i.
        let total_forces: Vec<Vec<Vector2D<f64>>> =
            vec![vec![Vector2D::from_xy(0.0, 0.0); self.get_n_nodes()]; self.get_n_nodes()];

        // Loop over all pairs i, j of nodes
        for i in 0..self.get_n_nodes() {
            for j in (i + 1)..self.get_n_nodes() {
                let distance = self.distances[i][j];
                let direction = self.directions[i][j];
                let n1_mass = self.masses[i];
                let n2_mass = self.masses[j];
                let weight = self.get_edge_connecting_nodes(i, j).unwrap().weight;

                // Calculate the total force that node i exerts on node j
                total_forces[i][j] =
                    self.total_force_n1_exerts_on_n2(&self.nodes[i], &self.nodes[j], weight);

                // Force is anti-symmetrical, so the force that node j exerts on node i is the
                // negative of the force that node i exerts on node j
                total_forces[j][i] = -total_forces[i][j];
            }
        }

        // Return the total forces
        total_forces
    }

    /// Applies forces between all pairs of nodes to get the change in position and velocity. The
    /// change in position and velocity is returned as a tuple of two vectors of 2D vectors. The
    /// first vector contains the change in position vectors for each node, and the second vector
    /// contains the change in velocity vectors for each node.
    ///
    /// Note that we have made a simplifying assumption that the change in position and velocity
    /// vectors for each node is independent of the change in position and velocity vectors for
    /// other nodes. This is not true in general, but it is a reasonable approximation for small
    /// time steps.
    ///
    fn apply_forces(&mut self) -> (Vec<Vec<Vector2D<f64>>>, Vec<Vec<Vector2D<f64>>>) {
        let force: Vec<Vec<Vector2D<f64>>> = self.calculate_forces();
        let delta_time = self.time_step;

        // Allocate memory for the total change in position and velocity vectors & init to 0
        let delta_p: vec![vec![Vector2D::from_xy(0.0, 0.0); self.get_n_nodes()]];
        let delta_v: vec![vec![Vector2D::from_xy(0.0, 0.0); self.get_n_nodes()]];

        // Loop over all pairs i, j of nodes, adding the change in position and velocity vectors
        // for each pair to get the total change in position and velocity vectors
        for i in 0..(self.get_n_nodes() - 1) {
            for j in 0..(self.get_n_nodes() - 1) {
                let weight = self.get_edge_connecting_nodes(i, j).unwrap().weight;

                // Calculate the change in position of node j due to the force that node i exerts
                // on node j
                delta_p[i] += self.chg_in_position_from_force_n1_exerts_on_n2(
                    &self.nodes[i],
                    &self.nodes[j],
                    weight,
                    delta_time,
                );

                // Calculate the change in velocity of node j due to the force that node i exerts
                // on node j
                delta_v[i] += self.chg_in_velocity_from_force_n1_exerts_on_n2(
                    &self.nodes[i],
                    &self.nodes[j],
                    weight,
                    delta_time,
                );
            }
        }

        // Return the total change in position and velocity vectors
        (delta_p, delta_v)
    }

    /// Updates positions and velocities of all nodes based on the forces.
    fn update_positions_and_velocities(&mut self) {
        let (delta_p, delta_v) = self.apply_forces();

        // Loop over all nodes, updating their positions and velocities
        for i in 0..self.get_n_nodes() {
            self.nodes[i].position += delta_p[i];
            self.nodes[i].velocity += delta_v[i];
        }
    }
}

#[cfg(test)]
pub mod test {
    use super::*;

    fn setup() -> (Vec<Node>, Vec<Edge>) {
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
        let edge2 = Edge::new(0, 2, 2.0);
        let edge3 = Edge::new(1, 2, 3.0);
        let edges: Vec<Edge> = vec![edge1, edge2, edge3];

        (nodes, edges)
    }

    fn get_force_simulation() -> ForceSimulation {
        let (nodes, edges) = setup();
        ForceSimulation::new(nodes, edges, 1.0, 1.0, 1.0)
    }

    #[test]
    pub fn test_get_node_mass() {
        let mut force_simulation = get_force_simulation();

        assert_eq!(force_simulation.get_node_mass(0), 1.0 + 2.0);
        assert_eq!(force_simulation.get_node_mass(1), 1.0 + 3.0);
        assert_eq!(force_simulation.get_node_mass(2), 2.0 + 3.0);
    }

    #[test]
    pub fn test_repulsive_force_n1_exerts_on_n2() {
        let mut force_simulation = get_force_simulation();

        let distance = 1.0;
        let n1_mass = 1.0;
        let n2_mass = 1.0;
        let direction = 0.0;

        let expected = Vector2D::from_xy(1.0, 0.0);
        let actual =
            force_simulation.repulsive_force_n1_exerts_on_n2(distance, n1_mass, n2_mass, direction);
        assert_eq!(actual, expected);
    }

    #[test]
    pub fn test_attractive_force_n1_exerts_on_n2() {
        let mut force_simulation = get_force_simulation();

        let distance = 1.0;
        let weight = 1.0;
        let direction = 0.0;

        let expected = Vector2D::from_xy(1.0, 0.0);
        let actual = force_simulation.attractive_force_n1_exerts_on_n2(distance, weight, direction);
        assert_eq!(actual, expected);
    }

    #[test]
    pub fn test_total_force_n1_exerts_on_n2() {
        let (nodes, edges) = setup();
        let weight = edges[1].weight;
        let mut fs = get_force_simulation();

        let mut n1 = &nodes[0];
        let mut n2 = &nodes[2];
        let repulsive_force: Vector2D<f64> = fs
            .repulsive_force_n1_exerts_on_n2(
                n1.position.distance(&n2.position),
                n1.mass,
                n2.mass,
                n1.position.relative_to(&n2.position).angle(),
            )
            .round(5);

        println!("repulsive_force: {:?}", repulsive_force);

        let attractive_force = fs
            .attractive_force_n1_exerts_on_n2(
                n1.position.distance(&n2.position),
                weight,
                n1.position.relative_to(&n2.position).angle(),
            )
            .round(5);

        println!("attractive_force: {:?}", attractive_force);

        let expected = attractive_force - repulsive_force;

        println!("expected: {:?}", expected);

        let actual = fs.total_force_n1_exerts_on_n2(n1, n2, weight).round(5);
        assert_eq!(actual, expected);
    }

    #[test]
    pub fn test_acceleration_from_force_n1_exerts_on_n2() {
        let (nodes, edges) = setup();
        let weight = edges[1].weight;
        let mut fs = get_force_simulation();

        let mut n1 = &nodes[0];
        let mut n2 = &nodes[2];
        let force = fs.total_force_n1_exerts_on_n2(n1, n2, weight);

        println!("net force: {:?}", force);

        let expected = force / n2.mass;

        println!("expected: {:?}", expected);

        let actual = fs.acceleration_from_force_n1_exerts_on_n2(n1, n2, weight);
        assert_eq!(actual.round(5), expected.round(5));
    }

    #[test]
    pub fn test_chg_in_velocity_from_force_n1_exerts_on_n2() {
        let time_step = 0.25;
        let (nodes, edges) = setup();
        let weight = edges[1].weight;
        let mut fs = get_force_simulation();

        let mut n1 = &nodes[0];
        let mut n2 = &nodes[2];
        let acceleration = fs.acceleration_from_force_n1_exerts_on_n2(n1, n2, weight);

        println!("acceleration: {:?}", acceleration);

        let v0 = n2.velocity;
        let delta_v = acceleration * time_step;

        println!("expected delta v: {:?}", delta_v);

        let actual = fs.chg_in_velocity_from_force_n1_exerts_on_n2(n1, n2, weight, time_step);
        assert_eq!(actual.round(5), delta_v.round(5));
    }

    #[test]
    pub fn test_chg_in_position_from_force_n1_exerts_on_n2() {
        let time_step = 5.0;
        let (nodes, edges) = setup();
        let weight = edges[1].weight;
        let mut fs = get_force_simulation();

        let mut n1 = &nodes[0];
        let mut n2 = &nodes[2];
        let acceleration = fs.acceleration_from_force_n1_exerts_on_n2(n1, n2, weight);

        println!("acceleration: {:?}", acceleration.round(3));

        let p0 = n2.position;
        let v0 = n2.velocity;

        println!("p0: {:?}", p0.round(3));
        println!("v0: {:?}", v0.round(3));

        let pf = p0 + v0 * time_step + acceleration * time_step.powi(2) / 2.0;

        println!("expected pf: {:?}", pf.round(3));

        let delta_p = pf - p0;

        println!("expected delta p: {:?}", delta_p.round(3));

        let actual = fs.chg_in_position_from_force_n1_exerts_on_n2(n1, n2, weight, time_step);
        assert_eq!(actual.round(5), delta_p.round(5));
    }
}
