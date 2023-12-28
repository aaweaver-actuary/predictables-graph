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
}
