use crate::graph::{edge::Edge, node::Node};

pub struct ForceSimulation {
    nodes: Vec<Node>,
    edges: Vec<Edge>,
    repulsion_constant: f64,
    attraction_constant: f64,
    distances: Vec<f64>,         // Flattened half-matrix for distances
    directions: Vec<Vector2D>,   // Flattened half-matrix for directions
    total_forces: Vec<Vector2D>, // Permanent buffer for total forces
}

impl ForceSimulation {
    pub fn new(
        nodes: Vec<Node>,
        edges: Vec<Edge>,
        repulsion_constant: f64,
        attraction_constant: f64,
    ) -> Self {
        let num_nodes = nodes.len();
        let num_elements = num_nodes * (num_nodes - 1) / 2; // Half-matrix size
        let distances = vec![0.0; num_elements];
        let directions = vec![Vector2D::new(0.0, 0.0); num_elements];
        let total_forces = vec![Vector2D::new(0.0, 0.0); num_nodes];

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
        let (i, j) = if i < j { (i, j) } else { (j, i) };
        i * self.nodes.len() + j - (i * (i + 1) / 2)
    }

    fn precompute_distances_and_directions(&mut self) {
        for i in 0..self.nodes.len() {
            for j in i + 1..self.nodes.len() {
                let index = self.index_for_distance_direction(i, j);
                self.distances[index] = self.nodes[i].position.distance_to(&self.nodes[j].position);
                self.directions[index] =
                    self.nodes[i].position.direction_to(&self.nodes[j].position);
            }
        }
    }

    pub fn update(&mut self, delta_time: f64) {
        self.precompute_distances_and_directions();

        // Reset total forces
        for force in &mut self.total_forces {
            *force = Vector2D::new(0.0, 0.0);
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
                let index = self.index_for_distance_direction(i, j);
                let distance = self.distances[index];
                let direction = self.directions[index];

                // Avoid division by very small numbers for stability
                if distance > 1e-5 {
                    let force_magnitude = self.repulsion_constant / distance.powi(2);
                    let force = direction * force_magnitude;

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
            let force = direction * force_magnitude;

            self.total_forces[edge.node1_idx] += force;
            self.total_forces[edge.node2_idx] -= force;
        }
    }
}
