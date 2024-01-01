use crate::graph::edge::Edge;
use crate::graph::edge_list::EdgeList;
use crate::graph::node::Node;
use crate::graph::node_list::NodeList;
use crate::math::vector_2d::Vector2D;

use derive_builder::Builder;
use rand::Rng;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, PartialOrd, Serialize, Deserialize, Builder, Default)]
pub struct Graph {
    pub nodes: NodeList,
    pub edges: EdgeList,
}

impl Graph {
    /// Create a new GraphBuilder with default values. The GraphBuilder can then
    /// be used to create a Graph with custom values.
    #[allow(clippy::new_ret_no_self)]
    pub fn new() -> GraphBuilder {
        GraphBuilder::default()
    }

    pub fn default() -> Graph {
        Graph {
            nodes: NodeList::default(),
            edges: EdgeList::default(),
        }
    }

    pub fn n_nodes(&self) -> usize {
        self.nodes.len()
    }

    pub fn n_edges(&self) -> usize {
        self.edges.len()
    }

    /// Create a fully connected graph with `n` nodes.
    ///
    /// # Arguments
    ///
    /// * `n` - The number of nodes in the graph.
    ///
    /// # Examples
    ///
    /// ```
    /// use graph::graph::Graph;
    ///
    /// let graph = Graph::fully_connected(10);
    /// ```
    pub fn fully_connected(&self, n: usize) -> Graph {
        // Set up random number generator for random position initialization
        let mut rng = rand::thread_rng();

        let mut nodes = NodeList::new();
        let mut edges = EdgeList::new();

        for i in 0..n {
            let node = Node::new()
                .id(i)
                .position(Vector2D {
                    x: rng.gen_range(-1.0..1.0),
                    y: rng.gen_range(-1.0..1.0),
                }) // random position - between -1 and 1
                .build()
                .unwrap();
            nodes.nodes(vec![node]);

            for j in 0..i {
                if i != j {
                    let edge = Edge::new()
                        .node1_idx(i)
                        .node2_idx(j)
                        .weight(1.0)
                        .build()
                        .unwrap();
                    edges.edges(vec![edge]);
                }
            }
        }

        Graph {
            nodes: nodes.build().unwrap(),
            edges: edges.build().unwrap(),
        }
    }

    pub fn add_node(&mut self, node: Node) {
        self.nodes.nodes.push(node);
    }

    pub fn add_edge(&mut self, edge: Edge) {
        self.edges.edges.push(edge);
    }

    pub fn get_node(&self, node_idx: usize) -> Option<&Node> {
        self.nodes.nodes.get(node_idx)
    }

    pub fn get_edge(&self, edge_idx: usize) -> Option<&Edge> {
        self.edges.edges.get(edge_idx)
    }
}
