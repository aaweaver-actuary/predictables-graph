use serde::{Deserialize, Serialize};
use std::ops::{Add, AddAssign, Div, DivAssign, Mul, MulAssign, Neg, Sub, SubAssign};

/// Returns true if the absolute difference between `a` and `b` is less than `epsilon`.
/// This is used in the tests below to see angles of 2pi and 0 as equal.
///
/// # Parameters
///
/// * `a` - The first value to compare.
/// * `b` - The second value to compare.
/// * `epsilon` - The maximum difference between `a` and `b` for them to be considered equal.
///
/// # Returns
///
/// * `true` if the absolute difference between `a` and `b` is less than `epsilon`.
/// * `false` otherwise.
///
/// ## Examples
///
/// ```
/// use math::vector_2d::approx_equal;
///
/// let a = 0.0;
/// let b = 2.0 * std::f64::cons::PI;
/// let epsilon = 0.0001;
/// assert_eq!(approx_equal(a, b, epsilon), true);
/// ```
///
fn approx_equal(a: f64, b: f64, epsilon: f64) -> bool {
    (a - b).abs() < epsilon
}

/// A 2D vector. This is a simple struct that represents a 2D vector.
///
/// # Attributes
///
/// * `x` - The x component of the vector.
/// * `y` - The y component of the vector.
///
/// # Methods
///
/// * `from_xy` - Creates a new vector from the x and y components.
/// * `from_theta` - Creates a new unit vector from an angle in radians.
/// * `from_rtheta` - Creates a new vector from a radius and an angle in radians.
/// * `add` - Adds two vectors together.
/// * `sub` - Subtracts one vector from another.
/// * `dot` - Calculates the dot product of two vectors.
/// * `scale` - Scales a vector by a scalar.
/// * `magnitude` - Calculates the magnitude of a vector.
/// * `angle` - Calculates the angle of a vector.
/// * `normalize` - Normalizes a vector.
/// * `orthogonal` - Calculates a vector that is orthogonal to the original.
/// * `orthonormal` - Calculates a vector that is orthogonal and normalized.
/// * `project_on` - Projects a vector onto another vector.
/// * `rotate` - Rotates a vector by an angle in radians.
/// * `rotate_around` - Rotates a vector around another vector by an angle in radians.
/// * `distance` - Calculates the distance between two vectors.
/// * `linear_interpolation` - Calculates a linear interpolation between two vectors.
///
/// # Operators
///
/// * `+` - Adds two vectors together.
/// * `-` - Subtracts one vector from another.
/// * `*` - Calculates the dot product of two vectors or scales a vector by a scalar, depending on the types of the operands.
/// * `/` - Scales a vector by a scalar.
/// * `-` - Negates a vector.
/// * `+=` - Adds two vectors together and assigns the result to the first vector.
/// * `-=` - Subtracts one vector from another and assigns the result to the first vector.
/// * `*=` - Calculates the dot product of two vectors or scales a vector by a scalar, depending on the types of the operands, and assigns the result to the first vector.
/// * `/=` - Scales a vector by a scalar and assigns the result to the first vector.
///
/// # Examples
///
/// ```rust
/// use math::vector_2d::Vector2D;
///
/// let v1 = Vector2D::from_xy(1.0, 2.0);
/// let v2 = Vector2D::from_xy(3.0, 4.0);
/// let v3 = v1 + v2;
///
/// println!("v3.x = {}", v3.x);
/// println!("v3.y = {}", v3.y);
/// ```
///
#[derive(Debug, Serialize, Deserialize, Clone, Copy, PartialEq, PartialOrd)]
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
        &start.scale(1.0 - t) + &end.scale(t)
    }
}

impl Add for Vector2D {
    type Output = Vector2D;

    fn add(self, other: Vector2D) -> Vector2D {
        Vector2D {
            x: self.x + other.x,
            y: self.y + other.y,
        }
    }
}

impl<'a, 'b> Add<&'b Vector2D> for &'a Vector2D {
    type Output = Vector2D;

    fn add(self, other: &'b Vector2D) -> Vector2D {
        Vector2D {
            x: self.x + other.x,
            y: self.y + other.y,
        }
    }
}

impl Sub<Vector2D> for Vector2D {
    type Output = Vector2D;

    fn sub(self, other: Vector2D) -> Vector2D {
        Vector2D {
            x: self.x - other.x,
            y: self.y - other.y,
        }
    }
}

impl<'a, 'b> Sub<&'b Vector2D> for &'a Vector2D {
    type Output = Vector2D;

    fn sub(self, other: &'b Vector2D) -> Vector2D {
        Vector2D {
            x: self.x - other.x,
            y: self.y - other.y,
        }
    }
}

impl Mul<Vector2D> for Vector2D {
    type Output = f64;

    fn mul(self, other: Vector2D) -> f64 {
        self.x * other.x + self.y * other.y
    }
}

impl<'a> Mul<f64> for &'a Vector2D {
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

impl<'a> Mul<&'a Vector2D> for f64 {
    type Output = Vector2D;

    fn mul(self, vector: &'a Vector2D) -> Vector2D {
        vector.scale(self)
    }
}

impl Div<f64> for Vector2D {
    type Output = Vector2D;

    fn div(self, scalar: f64) -> Vector2D {
        self.scale(1.0 / scalar)
    }
}

impl<'a> Div<f64> for &'a Vector2D {
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

impl<'a> Neg for &'a Vector2D {
    type Output = Vector2D;

    fn neg(self) -> Vector2D {
        self.scale(-1.0)
    }
}

impl AddAssign for Vector2D {
    fn add_assign(&mut self, other: Vector2D) {
        *self = self.add(other);
    }
}

impl SubAssign for Vector2D {
    fn sub_assign(&mut self, other: Vector2D) {
        *self = self.sub(other);
    }
}

impl SubAssign<&Vector2D> for Vector2D {
    fn sub_assign(&mut self, other: &Vector2D) {
        *self = &*self - other;
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
    use crate::math::vector_2d::approx_equal;
    pub const PI: f64 = std::f64::consts::PI;

    #[test]
    fn test_add() {
        let v1 = Vector2D::from_xy(1.0, 2.0);
        let v2 = Vector2D::from_xy(3.0, 4.0);

        // test the add method
        let v3 = v1.add(v2);
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
        let v3 = v1.sub(v2);
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
        assert_eq!(angle, PI / 4.0);
    }

    #[test]
    fn test_normalize() {
        let v1 = Vector2D::from_xy(3.0, 4.0);
        let v2 = v1.normalize();
        assert!(approx_equal(v2.x * v2.x + v2.y * v2.y, 1.0, 1e-4));
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
        assert!(approx_equal(v2.magnitude(), 1.0, 1e-4));
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
    fn test_rotation() {
        let epsilon = 1e-10; // Define an appropriate tolerance level
        let v1 = Vector2D::from_xy(1.0, 0.0);

        // Rotating by 2*pi radians (full circle)
        let rotated = v1.rotate(2.0 * PI);

        // Check if the rotated vector is approximately equal to the original
        assert!(approx_equal(rotated.x, v1.x, epsilon));
        assert!(approx_equal(rotated.y, v1.y, epsilon));
    }

    #[test]
    fn test_rotate_around() {
        let v1 = Vector2D::from_xy(1.0, 0.0);
        let v2 = Vector2D::from_xy(0.0, 0.0);
        let v3 = v1.rotate_around(PI / 2.0, &v2);

        assert!(approx_equal(v3.x, 0.0, 1e-10));
        assert!(approx_equal(v3.y, 1.0, 1e-10));
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
    fn test_from_theta() {
        let v1 = Vector2D::from_theta(PI / 2.0);
        assert!(approx_equal(v1.x, 0.0, 1e-4));
        assert!(approx_equal(v1.y, 1.0, 1e-4));
    }

    #[test]
    fn test_from_rtheta() {
        let v1 = Vector2D::from_rtheta(2.0, PI / 2.0);
        assert!(approx_equal(v1.x, 0.0, 1e-4));
        assert!(approx_equal(v1.y, 2.0, 1e-4));
    }
}
