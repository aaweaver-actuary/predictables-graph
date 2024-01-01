use crate::graph::node::Node;
use crate::math::vector_2d::Vector2D;
use derive_builder::Builder;
use serde::{Deserialize, Serialize};

/// A list of nodes. This is a wrapper around a `Vec<Node>`, with additional methods.
#[derive(Debug, Clone, PartialEq, PartialOrd, Serialize, Deserialize, Builder, Default)]
#[builder(setter(into))]
pub struct NodeList {
    pub nodes: Vec<Node>,
}

impl NodeList {
    /// Create a new NodeListBuilder with default values. The NodeListBuilder can then
    /// be used to create a NodeList with custom values.
    #[allow(clippy::new_ret_no_self)]
    pub fn new() -> NodeListBuilder {
        NodeListBuilder::default()
    }

    pub fn add_node(&mut self, node: Node) {
        self.nodes.push(node);
    }

    pub fn add_nodes(&mut self, nodes: Vec<Node>) {
        self.nodes.extend(nodes);
    }

    pub fn len(&self) -> usize {
        self.nodes.len()
    }
}

impl Iterator for NodeList {
    type Item = Node;
    fn next(&mut self) -> Option<Self::Item> {
        let mut next = self.nodes.iter();

        Some(next.next()).map(|x| x.unwrap().clone())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_node_list_builder() {
        let node_list = NodeList::new()
            .nodes(vec![Node::default()])
            .build()
            .unwrap();
        assert_eq!(node_list.nodes.len(), 1);

        let node1 = Node::new()
            .id(1)
            .position(Vector2D::from_xy(1.0, 1.0))
            .mass(1.0)
            .velocity(Vector2D::from_xy(0.0, 0.0))
            .radius(1.0)
            .label("".to_string())
            .build();

        let node2 = Node::new()
            .id(2)
            .position(Vector2D::from_xy(2.0, 2.0))
            .build();

        let node_list2 = NodeList::new()
            .nodes(vec![node1.unwrap(), node2.unwrap()])
            .build()
            .unwrap();

        assert_eq!(node_list2.nodes.len(), 2);
    }

    #[test]
    fn test_node_list_iterator() {
        let node1 = Node::new()
            .id(1)
            .position(Vector2D::from_xy(1.0, 1.0))
            .mass(1.0)
            .velocity(Vector2D::from_xy(0.0, 0.0))
            .radius(1.0)
            .label("".to_string())
            .build()
            .unwrap();

        let node2 = Node::new()
            .id(2)
            .position(Vector2D::from_xy(2.0, 2.0))
            .build()
            .unwrap();

        let mut node_list = NodeList::new().nodes(vec![node1, node2]).build().unwrap();

        let mut iter = node_list.clone().nodes.into_iter();

        assert_eq!(iter.next(), Some(node_list.nodes[0].clone()));
        assert_eq!(iter.next(), Some(node_list.nodes[1].clone()));
        assert_eq!(iter.next(), None);
    }

    #[test]
    fn test_node_list_empty() {
        let node_list = NodeList::new().nodes(vec![]).build().unwrap();
        assert_eq!(node_list.nodes.len(), 0);
    }

    #[test]
    fn test_node_list_single_node() {
        let node = Node::new()
            .id(1)
            .position(Vector2D::from_xy(1.0, 1.0))
            .mass(1.0)
            .velocity(Vector2D::from_xy(0.0, 0.0))
            .radius(1.0)
            .label("".to_string())
            .build()
            .unwrap();

        let node_list = NodeList::new().nodes(vec![node]).build().unwrap();
        assert_eq!(node_list.nodes.len(), 1);
    }

    #[test]
    fn test_node_list_multiple_nodes() {
        let node1 = Node::new()
            .id(1)
            .position(Vector2D::from_xy(1.0, 1.0))
            .mass(1.0)
            .velocity(Vector2D::from_xy(0.0, 0.0))
            .radius(1.0)
            .label("".to_string())
            .build()
            .unwrap();

        let node2 = Node::new()
            .id(2)
            .position(Vector2D::from_xy(2.0, 2.0))
            .build()
            .unwrap();

        let node_list = NodeList::new().nodes(vec![node1, node2]).build().unwrap();

        assert_eq!(node_list.nodes.len(), 2);
    }
}
