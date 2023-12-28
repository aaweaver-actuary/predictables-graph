use crate::math::vector_2d::Vector2D;

#[derive(Debug, Clone)]
pub struct Node {
    pub id: u8,
    pub label: String,
    pub position: Vector2D<f64>,
    pub velocity: Vector2D<f64>,
    pub mass: f64,
    pub radius: f64,
    pub edge_color: String,
    pub fill: String,
}

impl Node {
    /// Create a new NodeBuilder with default values. The NodeBuilder can be used to create a Node
    /// with custom values.
    ///
    /// # Examples
    ///
    /// ```
    /// use crate::graph::node::Node;
    ///
    ///
    /// let node = Node::new()
    ///     .id(1)
    ///     .label("test")
    ///     .build(); // This creates a Node with default values except for id and label
    /// ```
    #[allow(clippy::new_ret_no_self)]
    pub fn new() -> NodeBuilder {
        NodeBuilder {
            id: 0,
            label: "".to_string(),
            position: Vector2D::from_xy(1.0, 1.0),
            mass: 1.0,
            velocity: Vector2D::from_xy(0.0, 0.0),
            radius: 1.0,
            edge_color: "black".to_string(),
            fill: "transparent".to_string(),
        }
    }
}

impl Default for Node {
    fn default() -> Node {
        Node::new().build()
    }
}

pub struct NodeBuilder {
    id: u8,
    label: String,
    position: Vector2D<f64>,
    velocity: Vector2D<f64>,
    mass: f64,
    radius: f64,
    edge_color: String,
    fill: String,
}

impl NodeBuilder {
    pub fn id(mut self, id: u8) -> Self {
        self.id = id;
        self
    }

    pub fn mass(mut self, mass: f64) -> Self {
        self.mass = mass;
        self
    }

    pub fn label(mut self, label: &str) -> Self {
        self.label = label.to_string();
        self
    }

    pub fn position(mut self, position: Vector2D<f64>) -> Self {
        self.position = position;
        self
    }

    pub fn velocity(mut self, velocity: Vector2D<f64>) -> Self {
        self.velocity = velocity;
        self
    }

    pub fn radius(mut self, radius: f64) -> Self {
        self.radius = radius;
        self
    }

    pub fn edge_color(mut self, color: &str) -> Self {
        self.edge_color = color.to_string();
        self
    }

    pub fn fill(mut self, fill: &str) -> Self {
        self.fill = fill.to_string();
        self
    }

    pub fn build(self) -> Node {
        Node {
            id: self.id,
            label: self.label,
            position: self.position,
            velocity: self.velocity,
            mass: self.mass,
            radius: self.radius,
            edge_color: self.edge_color,
            fill: self.fill,
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
            .label("test")
            .velocity(velocity)
            .radius(6.0)
            .edge_color("red")
            .fill("blue")
            .build();

        // Test that the default values are correctly set if no builder methods are called
        let node2 = Node::new().build();

        // NodeBuilder checks
        assert_eq!(node.id, 1);
        assert_eq!(node.label, "test".to_string());
        assert_eq!(node.position, position);
        assert_eq!(node.velocity, velocity);
        assert_eq!(node.mass, 5.0);
        assert_eq!(node.radius, 6.0);
        assert_eq!(node.edge_color, "red".to_string());
        assert_eq!(node.fill, "blue".to_string());

        // Default values check
        assert_eq!(node2.id, 0);
        assert_eq!(node2.label, "".to_string());
        assert_eq!(node2.position, Vector2D::from_xy(1.0, 1.0));
        assert_eq!(node2.velocity, Vector2D::from_xy(-2.0, 3.0));
        assert_eq!(node2.mass, 1.0);
        assert_eq!(node2.radius, 1.0);
        assert_eq!(node2.edge_color, "black".to_string());
        assert_eq!(node2.fill, "transparent".to_string());
    }
}
