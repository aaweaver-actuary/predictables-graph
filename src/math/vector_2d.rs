use std::ops::{Add, AddAssign, Div, DivAssign, Mul, MulAssign, Neg, Sub, SubAssign};

pub struct Vector2D {
    pub x: f64,
    pub y: f64,
}

impl Vector2D {
    pub fn from_xy(x: f64, y: f64) -> Vector2D {
        Vector2D { x, y }
    }

    pub fn from_theta(angle: f64) -> Vector2D {
        Vector2D {
            x: angle.cos(),
            y: angle.sin(),
        }
    }

    pub fn from_rtheta(radius: f64, angle: f64) -> Vector2D {
        Vector2D {
            x: radius * angle.cos(),
            y: radius * angle.sin(),
        }
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

    pub fn angle(&self) -> f64 {
        self.y.atan2(self.x)
    }

    pub fn normalize(&self) -> Vector2D {
        let magnitude = self.magnitude();
        if magnitude == 0.0 {
            return Vector2D { x: 0.0, y: 0.0 };
        }
        self.scale(1.0 / magnitude)
    }

    pub fn orthogonal(&self) -> Vector2D {
        Vector2D {
            x: -self.y,
            y: self.x,
        }
    }

    pub fn orthonormal(&self) -> Vector2D {
        self.orthogonal().normalize()
    }

    pub fn project_on(&self, other: &Vector2D) -> Vector2D {
        let scalar = self.dot(other) / other.dot(other);
        other.scale(scalar)
    }

    pub fn rotate(&self, angle: f64) -> Vector2D {
        let new_angle = self.angle() + angle;
        let magnitude = self.magnitude();
        Vector2D {
            x: magnitude * new_angle.cos(),
            y: magnitude * new_angle.sin(),
        }
    }

    pub fn rotate_around(&self, angle: f64, other: &Vector2D) -> Vector2D {
        let new_angle = self.sub(other).angle() + angle;
        let magnitude = self.sub(other).magnitude();
        Vector2D {
            x: magnitude * new_angle.cos() + other.x,
            y: magnitude * new_angle.sin() + other.y,
        }
    }

    pub fn distance(&self, other: &Vector2D) -> f64 {
        (self.sub(other)).magnitude()
    }

    pub fn linear_interpolation(start: &Vector2D, end: &Vector2D, t: f64) -> Vector2D {
        start.scale(1.0 - t).add(&end.scale(t))
    }

    pub fn reflect(&self, normal: &Vector2D) -> Vector2D {
        let normal_normalized = normal.normalize();
        self.sub(&normal_normalized.scale(2.0 * self.dot(&normal_normalized)))
    }
}

impl Add for Vector2D {
    type Output = Vector2D;

    fn add(self, other: Vector2D) -> Vector2D {
        self.add(&other)
    }
}

impl Sub for Vector2D {
    type Output = Vector2D;

    fn sub(self, other: Vector2D) -> Vector2D {
        self.sub(&other)
    }
}

impl Mul<f64> for Vector2D {
    type Output = Vector2D;

    fn mul(self, scalar: f64) -> Vector2D {
        self.scale(scalar)
    }
}

impl Mul<Vector2D> for f64 {
    type Output = Vector2D;

    fn mul(self, vector: Vector2D) -> Vector2D {
        vector.scale(self)
    }
}

impl Div<f64> for Vector2D {
    type Output = Vector2D;

    fn div(self, scalar: f64) -> Vector2D {
        self.scale(1.0 / scalar)
    }
}

impl Neg for Vector2D {
    type Output = Vector2D;

    fn neg(self) -> Vector2D {
        self.scale(-1.0)
    }
}

impl AddAssign for Vector2D {
    fn add_assign(&mut self, other: Vector2D) {
        *self = self.add(&other);
    }
}

impl SubAssign for Vector2D {
    fn sub_assign(&mut self, other: Vector2D) {
        *self = self.sub(&other);
    }
}

impl MulAssign<f64> for Vector2D {
    fn mul_assign(&mut self, scalar: f64) {
        *self = self.scale(scalar);
    }
}

impl DivAssign<f64> for Vector2D {
    fn div_assign(&mut self, scalar: f64) {
        *self = self.scale(1.0 / scalar);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_add() {
        let v1 = Vector2D::from_xy(1.0, 2.0);
        let v2 = Vector2D::from_xy(3.0, 4.0);

        // test the add method
        let v3 = v1.add(&v2);
        assert_eq!(v3.x, 4.0);
        assert_eq!(v3.y, 6.0);

        // test the + operator
        let v4 = v1 + v2;
        assert_eq!(v4.x, 4.0);
        assert_eq!(v4.y, 6.0);

        // test the += operator
        let mut v5 = v1;
        v5 += v2;
        assert_eq!(v5.x, 4.0);
        assert_eq!(v5.y, 6.0);
    }

    #[test]
    fn test_sub() {
        let v1 = Vector2D::from_xy(1.0, 2.0);
        let v2 = Vector2D::from_xy(3.0, 4.0);
        let v3 = v1.sub(&v2);
        assert_eq!(v3.x, -2.0);
        assert_eq!(v3.y, -2.0);

        let v4 = v1 - v2;
        assert_eq!(v4.x, -2.0);
        assert_eq!(v4.y, -2.0);

        let mut v5 = v1;
        v5 -= v2;
        assert_eq!(v5.x, -2.0);
        assert_eq!(v5.y, -2.0);
    }

    #[test]
    fn test_dot() {
        let v1 = Vector2D::from_xy(1.0, 2.0);
        let v2 = Vector2D::from_xy(3.0, 4.0);
        let dot = v1.dot(&v2);
        assert_eq!(dot, 11.0);

        let dot = v1 * v2;
        assert_eq!(dot, 11.0);
    }

    #[test]
    fn test_scale() {
        let v1 = Vector2D::from_xy(1.0, 2.0);
        let v2 = v1.scale(2.0);
        assert_eq!(v2.x, 2.0);
        assert_eq!(v2.y, 4.0);

        let v3 = 2.0 * v1;
        assert_eq!(v3.x, 2.0);
        assert_eq!(v3.y, 4.0);

        let mut v4 = v1;
        v4 *= 2.0;
        assert_eq!(v4.x, 2.0);
        assert_eq!(v4.y, 4.0);
    }

    #[test]
    fn test_magnitude() {
        let v1 = Vector2D::from_xy(3.0, 4.0);
        let magnitude = v1.magnitude();
        assert_eq!(magnitude, 5.0);
    }

    #[test]
    fn test_angle() {
        let v1 = Vector2D::from_xy(1.0, 1.0);
        let angle = v1.angle();
        assert_eq!(angle, std::f64::consts::FRAC_PI_4);
    }

    #[test]
    fn test_normalize() {
        let v1 = Vector2D::from_xy(3.0, 4.0);
        let v2 = v1.normalize();
        assert_eq!(v2.x * v2.x + v2.y * v2.y, 1.0);
    }

    #[test]
    fn test_orthogonal() {
        let v1 = Vector2D::from_xy(1.0, 2.0);
        let v2 = v1.orthogonal();
        assert_eq!(v1.dot(&v2), 0.0);
    }

    #[test]
    fn test_orthonormal() {
        let v1 = Vector2D::from_xy(1.0, 2.0);
        let v2 = v1.orthonormal();
        assert_eq!(v2.magnitude(), 1.0);
    }

    #[test]
    fn test_project_on() {
        let v1 = Vector2D::from_xy(1.0, 2.0);
        let v2 = Vector2D::from_xy(3.0, 4.0);
        let v3 = v1.project_on(&v2);
        assert_eq!(v3.x, 1.32);
        assert_eq!(v3.y, 1.76);
    }

    #[test]
    fn test_rotate() {
        let v1 = Vector2D::from_xy(1.0, 0.0);
        let v2 = v1.rotate(std::f64::consts::FRAC_PI_2);
        assert_eq!(v2.x, 0.0);
        assert_eq!(v2.y, 1.0);
    }

    #[test]
    fn test_rotate_around() {
        let v1 = Vector2D::from_xy(1.0, 0.0);
        let v2 = Vector2D::from_xy(0.0, 0.0);
        let v3 = v1.rotate_around(std::f64::consts::FRAC_PI_2, &v2);
        assert_eq!(v3.x, 0.0);
        assert_eq!(v3.y, 1.0);
    }

    #[test]
    fn test_distance() {
        let v1 = Vector2D::from_xy(1.0, 0.0);
        let v2 = Vector2D::from_xy(0.0, 0.0);
        let distance = v1.distance(&v2);
        assert_eq!(distance, 1.0);
    }

    #[test]
    fn test_linear_interpolation() {
        let v1 = Vector2D::from_xy(1.0, 0.0);
        let v2 = Vector2D::from_xy(0.0, 0.0);
        let v3 = Vector2D::linear_interpolation(&v1, &v2, 0.5);
        assert_eq!(v3.x, 0.5);
        assert_eq!(v3.y, 0.0);
    }

    #[test]
    fn test_reflect() {
        let v1 = Vector2D::from_xy(1.0, 0.0);
        let v2 = Vector2D::from_xy(0.0, 1.0);
        let v3 = v1.reflect(&v2);
        assert_eq!(v3.x, -1.0);
        assert_eq!(v3.y, 0.0);
    }

    #[test]
    fn test_from_theta() {
        let v1 = Vector2D::from_theta(std::f64::consts::FRAC_PI_2);
        assert_eq!(v1.x, 0.0);
        assert_eq!(v1.y, 1.0);
    }

    #[test]
    fn test_from_rtheta() {
        let v1 = Vector2D::from_rtheta(2.0, std::f64::consts::FRAC_PI_2);
        assert_eq!(v1.x, 0.0);
        assert_eq!(v1.y, 2.0);
    }
}
