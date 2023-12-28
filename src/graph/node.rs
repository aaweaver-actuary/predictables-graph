#[derive(Debug, Clone)]
pub struct Node {
    pub position: Vector2D,
    pub velocity: Vector2D,
    pub mass: f64,
}

impl Node {
    pub fn new(position: Vector2D, mass: f64) -> Self {
        Node {
            position,
            velocity: Vector2D::new(0.0, 0.0),
            mass,
        }
    }
}
