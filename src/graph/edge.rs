#[derive(Debug, Clone, Copy, PartialEq, PartialOrd)]
pub struct Edge {
    pub node1_idx: usize, // Index of the first node
    pub node2_idx: usize, // Index of the second node
    pub weight: f64,      // Correlation strength
}

impl Edge {
    pub fn new(node1_idx: usize, node2_idx: usize, weight: f64) -> Self {
        Edge {
            node1_idx,
            node2_idx,
            weight,
        }
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
