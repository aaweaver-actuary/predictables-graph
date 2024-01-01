use derive_builder::Builder;
use serde::{Deserialize, Serialize};
use serde_json::map::Iter;

use crate::math::vector_2d::Vector2D;

#[derive(Debug, Clone, PartialEq, PartialOrd, Serialize, Deserialize, Builder, Default)]
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
        EdgeBuilder::default()
    }

    /// Create a new Edge with default values. This is a convenient way to create an Edge with
    /// default values.
    pub fn default() -> Edge {
        Edge {
            node1_idx: 0,
            node2_idx: 1,
            weight: 1.0,
        }
    }

    pub fn has_node(&self, node_idx: usize) -> bool {
        self.node1_idx == node_idx || self.node2_idx == node_idx
    }
}
