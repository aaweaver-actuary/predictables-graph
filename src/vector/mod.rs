pub struct Vector2D {
    pub x: f64,
    pub y: f64,
}

impl Vector2D {
    pub fn new(x: f64, y: f64) -> Vector2D {
        Vector2D { x, y }
    }

    pub fn add(&self, other: &Vector2D) -> Vector2D {
        Vector2D {
            x: self.x + other.x,
            y: self.y + other.y,
        }
    }

    pub fn sub(&self, other: &Vector2D) -> Vector2D {
        Vector2D {
            x: self.x - other.x,
            y: self.y - other.y,
        }
    }

    pub fn dot(&self, other: &Vector2D) -> f64 {
        self.x * other.x + self.y * other.y
    }

    pub fn scale(&self, scalar: f64) -> Vector2D {
        Vector2D {
            x: self.x * scalar,
            y: self.y * scalar,
        }
    }

    pub fn magnitude(&self) -> f64 {
        (self.x * self.x + self.y * self.y).sqrt()
    }

    pub fn normalize(&self) -> Vector2D {
        let magnitude = self.magnitude();
        Vector2D {
            x: self.x / magnitude,
            y: self.y / magnitude,
        }
    }

    pub fn angle(&self) -> f64 {
        self.y.atan2(self.x)
    }
}
