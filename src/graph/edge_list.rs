use derive_builder::Builder;
use serde::{Deserialize, Serialize};

use crate::graph::edge::Edge;
use crate::math::vector_2d::Vector2D;

/// A list of edges. This is a wrapper around a `Vec<Edge>`, with additional methods.
#[derive(Debug, Clone, PartialEq, PartialOrd, Builder, Default, Serialize, Deserialize)]
#[builder(setter(into))]
pub struct EdgeList {
    pub edges: Vec<Edge>,
}

impl EdgeList {
    /// Create a new EdgeListBuilder with default values. The EdgeListBuilder can then
    /// be used to create an EdgeList with custom values.
    #[allow(clippy::new_ret_no_self)]
    pub fn new() -> EdgeListBuilder {
        EdgeListBuilder::default()
    }

    pub fn add_edge(&mut self, edge: Edge) {
        self.edges.push(edge);
    }

    pub fn add_edges(&mut self, edges: Vec<Edge>) {
        self.edges.extend(edges);
    }

    pub fn len(&self) -> usize {
        self.edges.len()
    }
}

impl Iterator for EdgeList {
    type Item = Edge;
    fn next(&mut self) -> Option<Self::Item> {
        let mut next = self.edges.iter();

        Some(next.next()).map(|x| x.unwrap().clone())
    }
}
