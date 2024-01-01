use derive_builder::Builder;
use serde::{Deserialize, Serialize};
use serde_json::map::Iter;

use crate::math::vector_2d::Vector2D;

/// A node in a force-directed graph.
///
/// ## Attributes
///
/// * `id` - The unique identifier of the node. Note that `PartialEq` and `PartialOrd` are derived for this struct, so nodes with the same `id` are considered equal. The `id` also establishes an ordering, so nodes can be sorted by `id`.
/// * `label` - The label of the node. In many cases this is a text string that will be printed next to the node on the graph.
/// * `position` - The position of the node in 2D space. This is a `Vector2D` struct, which is a wrapper around two `f64` values. `position` is a quantity that changes over time.
/// * `velocity` - The velocity of the node in 2D space. This is a `Vector2D` struct, which is a wrapper around two `f64` values. `velocity` is a quantity that changes over time.
/// * `mass` - The mass of the node. This is a `f64` value. `mass` is a quantity that does not change over time, but given a force acting on the node between t and t + dt, the acceleration of the node is given by `a = F / m`.
/// * `radius` - The radius of the node. This is a `f64` value. `radius` is a quantity that does not change over time, but impacts the way that the node interacts with other nodes.
/// * `edge_color` - The color of the edges that connect this node to other nodes. This is a `String` value. `edge_color` is a quantity that does not change over time. This will often be "black" or "transparent".
/// * `fill` - The color of the node. This is a `String` value. `fill` is a quantity that does not change over time. This will often be "black" or "transparent".
///
/// ## Traits
///
/// * `Debug` - Allows the node to be printed to the console.
/// * `Clone` - Allows the node to be cloned (eg deep copied to another location in memory).
/// * `PartialEq` - Allows the node to be compared for equality with other nodes (eg `node1 == node2`). Here `node1 == node2` is true if and only if `node1.id == node2.id`.
/// * `PartialOrd` - Allows the node to be compared for ordering with other nodes (eg `node1 < node2`). Here `node1 < node2` is true if and only if `node1.id < node2.id`.
/// * `Serialize` - Allows the node to be serialized to a JSON string. This is how nodes are passed between the Rust and JavaScript code.
/// * `Deserialize` - Allows the node to be deserialized from a JSON string. This is how nodes are passed between the Rust and JavaScript code.
/// * `Builder` - Allows the node to be built using the `derive_builder` crate. This is a convenient way to create a node with custom values.
/// * `Default` - Allows the node to be created with default values using the `Default` trait. This is a convenient way to create a node with default values.
///
/// ## Methods
///
/// * `new()` - Create a new `NodeBuilder` with default values. The `NodeBuilder` can then be used to create a `Node` with custom values.
/// * `default()` - Create a new `Node` with default values. This is a convenient way to create a `Node` with default values.
///
/// ## Examples
///
/// ```
/// use crate::graph::node::Node;
/// use crate::math::vector_2d::Vector2D;
///
/// // Create a new node with default values
/// let default_node = Node::default();
///
/// // Create a new node with custom values using Builder syntax
/// let custom_node = Node::new()
///                     .id(1)
///                     .label("Node 1".to_string())
///                     .position(Vector2D::from_xy(1.0, 1.0))
///                     .velocity(Vector2D::from_xy(0.0, 0.0))
///                     .mass(1.0)
///                     .radius(1.0)
///                     .edge_color("black".to_string())
///                     .fill("transparent".to_string())
///                     .build()
///                     .unwrap();
/// ```
///
#[derive(Debug, Clone, PartialEq, PartialOrd, Serialize, Deserialize, Builder, Default)]
pub struct Node {
    pub id: usize,
    pub label: String,
    pub position: Vector2D<f64>,
    pub velocity: Vector2D<f64>,
    pub mass: f64,
    pub radius: f64,
    pub edge_color: String,
    pub fill: String,
}

impl Node {
    #[allow(clippy::new_ret_no_self)]
    pub fn new() -> NodeBuilder {
        NodeBuilder {
            id: Some(0),
            label: Some("".to_string()),
            position: Some(Vector2D::from_xy(1.0, 1.0)),
            velocity: Some(Vector2D::from_xy(0.0, 0.0)),
            mass: Some(1.0),
            radius: Some(1.0),
            edge_color: Some("black".to_string()),
            fill: Some("transparent".to_string()),
        }
    }

    pub fn default() -> Node {
        Node {
            id: 0,
            label: "".to_string(),
            position: Vector2D::from_xy(1.0, 1.0),
            velocity: Vector2D::from_xy(0.0, 0.0),
            mass: 1.0,
            radius: 1.0,
            edge_color: "black".to_string(),
            fill: "transparent".to_string(),
        }
    }
}

#[cfg(test)]
pub mod tests {
    use super::*;
    use crate::math::vector_2d::Vector2D;

    #[test]
    pub fn test_node_builder() {
        let position = Vector2D::from_xy(1.0, 2.0);
        let velocity = Vector2D::from_xy(3.0, 4.0);

        // Test that the NodeBuilder works as intended
        let node = Node::new()
            .id(1)
            .position(position)
            .mass(5.0)
            .label("test".to_owned())
            .velocity(velocity)
            .radius(6.0)
            .edge_color("red".to_owned())
            .fill("blue".to_owned())
            .build();

        match node {
            Ok(node) => {
                assert_eq!(node.id, 1);
                assert_eq!(node.label, "test".to_string());
                assert_eq!(node.position, position);
                assert_eq!(node.velocity, velocity);
                assert_eq!(node.mass, 5.0);
                assert_eq!(node.radius, 6.0);
                assert_eq!(node.edge_color, "red".to_string());
                assert_eq!(node.fill, "blue".to_string());
            }
            Err(e) => panic!("Error building node: {}", e),
        }

        // Test that the default values are correctly set if no builder methods are called
        let node2 = Node::default();

        assert_eq!(node2.id, 0);
        assert_eq!(node2.label, "".to_string());
        assert_eq!(node2.position, Vector2D::from_xy(1.0, 1.0));
        assert_eq!(node2.velocity, Vector2D::from_xy(0.0, 0.0));
        assert_eq!(node2.mass, 1.0);
        assert_eq!(node2.radius, 1.0);
        assert_eq!(node2.edge_color, "black".to_string());
        assert_eq!(node2.fill, "transparent".to_string());
    }

    #[test]
    pub fn test_new() {
        let node = Node::new().build().unwrap();
        assert_eq!(node.id, 0);
        assert_eq!(node.label, "".to_string());
        assert_eq!(node.position, Vector2D::from_xy(1.0, 1.0));
        assert_eq!(node.velocity, Vector2D::from_xy(0.0, 0.0));
        assert_eq!(node.mass, 1.0);
        assert_eq!(node.radius, 1.0);
        assert_eq!(node.edge_color, "black".to_string());
        assert_eq!(node.fill, "transparent".to_string());
    }

    #[test]
    pub fn test_derived_traits() {
        let node1 = Node::new().id(1).build().unwrap();
        let node2 = Node::new().id(1).build().unwrap();
        let node3 = Node::new().id(2).build().unwrap();

        // Test Clone
        assert_eq!(node1, node1.clone());

        // Test PartialEq and PartialOrd
        assert_eq!(node1, node2);
        assert!(node1 < node3);

        // Test Debug
        println!("{:?}", node1);

        // Test Serialize and Deserialize
        let serialized = serde_json::to_string(&node1).unwrap();
        let deserialized: Node = serde_json::from_str(&serialized).unwrap();
        assert_eq!(node1, deserialized);
    }

    #[test]
    pub fn test_large_values() {
        let large_value = 1e100;
        let node = Node::new()
            .mass(large_value)
            .radius(large_value)
            .build()
            .unwrap();
        assert_eq!(node.mass, large_value);
        assert_eq!(node.radius, large_value);
    }
}
