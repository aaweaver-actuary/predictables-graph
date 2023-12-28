use crate::graph::vector2d::Vector2D;

#[derive(Debug, Clone)]
pub struct Node {
    pub id: u8,
    pub label: String,
    pub position: Vector2D,
    pub velocity: Vector2D,
    pub mass: f64,
    pub radius: f64,
    pub edge_color: String,
    pub fill: String,
}

impl Node {
    pub fn new(position: Vector2D, mass: f64) -> NodeBuilder {
        NodeBuilder {
            id: 0,
            label: "".to_string(),
            position,
            mass,
            velocity: Vector2D::new(0.0, 0.0),
            radius: 1.0,
            edge_color: "black".to_string(),
            fill: "transparent".to_string(),
        }
    }
}

pub struct NodeBuilder {
    id: u8,
    label: String,
    position: Vector2D,
    velocity: Vector2D,
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

    pub fn label(mut self, label: &str) -> Self {
        self.label = label.to_string();
        self
    }

    pub fn velocity(mut self, velocity: Vector2D) -> Self {
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
mod tests {
    use super::*;

    #[test]
    fn test_node_builder() {
        let position = Vector2D::new(1.0, 2.0);
        let velocity = Vector2D::new(3.0, 4.0);

        // Test that the NodeBuilder works as intended
        let node = Node::new(position.clone(), 5.0)
            .id(1)
            .label("test")
            .velocity(velocity.clone())
            .radius(6.0)
            .edge_color("red")
            .fill("blue")
            .build();

        // Test that the default values are correctly set if no builder methods are called
        let node2 = Node::new(Vector2D::new(1.0, 1.0), Vector2D::new(-2.0, 3.0));

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
        assert_eq!(node2.position, Vector2D::new(1.0, 1.0));
        assert_eq!(node2.velocity, Vector2D::new(-2.0, 3.0));
        assert_eq!(node2.mass, 1.0);
        assert_eq!(node2.radius, 1.0);
        assert_eq!(node2.edge_color, "black".to_string());
        assert_eq!(node2.fill, "transparent".to_string());
    }
}
