use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, PartialEq, PartialOrd, Serialize, Deserialize)]
pub struct Edge {
    pub node1_idx: usize, // Index of the first node
    pub node2_idx: usize, // Index of the second node
    pub weight: f64,      // Correlation strength
}

impl Edge {
    /// Create a new EdgeBuilder with default values. The EdgeBuilder can be used to create an Edge
    /// with custom values.
    #[allow(clippy::new_ret_no_self)]
    pub fn new() -> EdgeBuilder {
        EdgeBuilder {
            node1_idx: 0,
            node2_idx: 1,
            weight: 1.0,
        }
    }

    pub fn update_weight(&mut self, weight: f64) {
        self.weight = weight;
    }

    pub fn update_node1_idx(&mut self, node1_idx: usize) {
        self.node1_idx = node1_idx;
    }

    pub fn update_node2_idx(&mut self, node2_idx: usize) {
        self.node2_idx = node2_idx;
    }

    pub fn get_node1_idx(&self) -> usize {
        self.node1_idx
    }

    pub fn get_node2_idx(&self) -> usize {
        self.node2_idx
    }

    pub fn get_weight(&self) -> f64 {
        self.weight
    }

    pub fn set_node1_idx(&mut self, node1_idx: usize) {
        self.node1_idx = node1_idx;
    }

    pub fn set_node2_idx(&mut self, node2_idx: usize) {
        self.node2_idx = node2_idx;
    }

    pub fn set_weight(&mut self, weight: f64) {
        self.weight = weight;
    }

    pub fn has_node(&self, node_idx: usize) -> bool {
        self.node1_idx == node_idx || self.node2_idx == node_idx
    }
}

// Set up EdgeBuilder struct
#[derive(Debug, Clone, Copy, PartialEq, PartialOrd, Serialize, Deserialize)]
pub struct EdgeBuilder {
    pub node1_idx: usize, // Index of the first node
    pub node2_idx: usize, // Index of the second node
    pub weight: f64,      // Correlation strength
}

impl EdgeBuilder {
    pub fn node1_idx(mut self, node1_idx: usize) -> Self {
        self.node1_idx = node1_idx;
        self
    }

    pub fn node2_idx(mut self, node2_idx: usize) -> Self {
        self.node2_idx = node2_idx;
        self
    }

    pub fn weight(mut self, weight: f64) -> Self {
        self.weight = weight;
        self
    }

    pub fn build(self) -> Edge {
        Edge {
            node1_idx: self.node1_idx,
            node2_idx: self.node2_idx,
            weight: self.weight,
        }
    }
}
