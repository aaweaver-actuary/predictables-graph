pub struct Edge {
    node1_idx: usize, // Index of the first node
    node2_idx: usize, // Index of the second node
    weight: f64,      // Correlation strength
}

impl Edge {
    pub fn new(node1_idx: usize, node2_idx: usize, weight: f64) -> Self {
        Edge {
            node1_idx,
            node2_idx,
            weight,
        }
    }
}
