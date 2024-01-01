use serde::{Deserialize, Serialize};
use std::ops::{Add, AddAssign, Div, DivAssign, Mul, MulAssign, Neg, Sub, SubAssign};

pub fn approx_equal(a: f64, b: f64, epsilon: f64) -> bool {
    (a - b).abs() < epsilon
}

#[derive(Debug, Serialize, Deserialize, Clone, Copy, Eq, PartialEq, PartialOrd, Default)]
pub struct Vector2D<T> {
    pub x: T,
    pub y: T,
}

impl<
        T: Copy
            + Add<Output = T>
            + Sub<Output = T>
            + Mul<Output = T>
            + Div<Output = T>
            + Neg<Output = T>
            + From<f64>,
    > Vector2D<T>
{
    pub fn from_xy(x: T, y: T) -> Self {
        Vector2D { x, y }
    }

    pub fn new_at_origin() -> Self {
        Vector2D {
            x: T::from(0.0),
            y: T::from(0.0),
        }
    }

    pub fn add(&self, other: &Vector2D<T>) -> Vector2D<T> {
        Vector2D {
            x: self.x + other.x,
            y: self.y + other.y,
        }
    }

    pub fn sub(&self, other: &Vector2D<T>) -> Vector2D<T> {
        Vector2D {
            x: self.x - other.x,
            y: self.y - other.y,
        }
    }

    pub fn dot(&self, other: &Vector2D<T>) -> T {
        self.x * other.x + self.y * other.y
    }

    pub fn scale(&self, scalar: T) -> Vector2D<T> {
        Vector2D {
            x: self.x * scalar,
            y: self.y * scalar,
        }
    }

    pub fn orthogonal(&self) -> Vector2D<T> {
        Vector2D {
            x: -self.y,
            y: self.x,
        }
    }

    pub fn project_on(&self, other: &Vector2D<T>) -> Vector2D<T> {
        let scalar = self.dot(other) / other.dot(other);
        other.scale(scalar)
    }
}
impl Vector2D<f64> {
    pub fn from_rtheta(radius: f64, angle: f64) -> Vector2D<f64> {
        Vector2D {
            x: radius * angle.cos(),
            y: radius * angle.sin(),
        }
    }

    pub fn from_theta(angle: f64) -> Vector2D<f64> {
        Vector2D {
            x: angle.cos(),
            y: angle.sin(),
        }
    }

    pub fn rotate(&self, angle: f64) -> Vector2D<f64> {
        let new_angle = self.angle() + angle;
        let magnitude = self.magnitude();
        Vector2D {
            x: magnitude * new_angle.cos(),
            y: magnitude * new_angle.sin(),
        }
    }

    pub fn rotate_around(&self, angle: f64, other: &Vector2D<f64>) -> Vector2D<f64> {
        let new_angle = self.sub(other).angle() + angle;
        let magnitude = self.sub(other).magnitude();
        Vector2D {
            x: magnitude * new_angle.cos() + other.x,
            y: magnitude * new_angle.sin() + other.y,
        }
    }

    pub fn magnitude(&self) -> f64 {
        (self.x * self.x + self.y * self.y).sqrt()
    }

    pub fn angle(&self) -> f64 {
        self.y.atan2(self.x)
    }

    pub fn distance(&self, other: &Vector2D<f64>) -> f64 {
        (self.sub(other)).magnitude()
    }

    pub fn normalize(&self) -> Vector2D<f64> {
        let magnitude = self.magnitude();
        if magnitude == 0.0 {
            return Vector2D { x: 0.0, y: 0.0 };
        }
        self.scale(1.0 / magnitude)
    }

    pub fn orthonormal(&self) -> Vector2D<f64> {
        self.orthogonal().normalize()
    }

    pub fn linear_interpolation(
        start: &Vector2D<f64>,
        end: &Vector2D<f64>,
        t: f64,
    ) -> Vector2D<f64> {
        start.scale(1.0 - t) + end.scale(t)
    }

    pub fn relative_to(&self, other: &Vector2D<f64>) -> Vector2D<f64> {
        self.sub(other)
    }

    /// Round the vector **componentwise** to the nearest n decimal places
    /// ### Parameters
    /// - `n`: The number of decimal places to round to
    /// ### Returns
    /// A new vector with the rounded components
    /// ### Examples
    /// ```
    /// use crate::math::vector_2d::Vector2D;
    /// let v1 = Vector2D::from_xy(1.234, 2.345);
    /// let v2 = v1.round(2);
    /// println!("{:?}", v2);
    /// ```
    /// #### Output
    /// ```text
    /// Vector2D { x: 1.23, y: 2.35 }
    /// ```
    /// ### Notes
    /// - This method is useful for rounding vectors to a certain number of decimal places
    /// - This method is mainly useful for testing purposes
    /// - This method does not pay close attention to floating point errors, and assumes that the
    ///   `n` parameter is small, say less than 5, so that the floating point errors are not
    ///   significant
    pub fn round(&self, n: usize) -> Vector2D<f64> {
        let order_of_mag: f64 = 10.0_f64.powi(n as i32);
        Vector2D {
            x: order_of_mag * self.x.round() / order_of_mag,
            y: order_of_mag * self.y.round() / order_of_mag,
        }
    }
}

impl<
        T: Copy
            + Add<Output = T>
            + Sub<Output = T>
            + Mul<Output = T>
            + Div<Output = T>
            + Neg<Output = T>
            + From<f64>,
    > Add for Vector2D<T>
{
    type Output = Self;

    fn add(self, other: Self) -> Self::Output {
        Vector2D {
            x: self.x + other.x,
            y: self.y + other.y,
        }
    }
}

impl<
        T: Copy
            + Add<Output = T>
            + Sub<Output = T>
            + Mul<Output = T>
            + Div<Output = T>
            + Neg<Output = T>
            + From<f64>,
    > Sub for Vector2D<T>
{
    type Output = Self;

    fn sub(self, other: Self) -> Self::Output {
        Vector2D {
            x: self.x - other.x,
            y: self.y - other.y,
        }
    }
}

impl<
        'a,
        'b,
        T: Copy + Add<Output = T> + Sub<Output = T> + Mul<Output = T> + Neg<Output = T> + From<f64>,
    > Add<&'b Vector2D<T>> for &'a Vector2D<T>
{
    type Output = Vector2D<T>;

    fn add(self, other: &'b Vector2D<T>) -> Vector2D<T> {
        Vector2D {
            x: self.x + other.x,
            y: self.y + other.y,
        }
    }
}

impl<
        'a,
        'b,
        T: Copy + Add<Output = T> + Sub<Output = T> + Mul<Output = T> + Neg<Output = T> + From<f64>,
    > Sub<&'b Vector2D<T>> for &'a Vector2D<T>
{
    type Output = Vector2D<T>;

    fn sub(self, other: &'b Vector2D<T>) -> Vector2D<T> {
        Vector2D {
            x: self.x - other.x,
            y: self.y - other.y,
        }
    }
}

impl<T> Mul<Vector2D<T>> for Vector2D<T>
where
    T: Mul<Output = T> + Add<Output = T> + Copy,
{
    type Output = T;

    fn mul(self, other: Vector2D<T>) -> T {
        // Dot product
        self.x * other.x + self.y * other.y
    }
}

impl<T> Mul<T> for Vector2D<T>
where
    T: Mul<Output = T> + Copy,
{
    type Output = Vector2D<T>;

    fn mul(self, scalar: T) -> Vector2D<T> {
        // Scalar multiplication
        Vector2D {
            x: self.x * scalar,
            y: self.y * scalar,
        }
    }
}

impl<T> Div<T> for Vector2D<T>
where
    T: Div<Output = T> + Copy,
{
    type Output = Vector2D<T>;

    fn div(self, scalar: T) -> Vector2D<T> {
        Vector2D {
            x: self.x / scalar,
            y: self.y / scalar,
        }
    }
}

impl<'a, T> Div<T> for &'a Vector2D<T>
where
    T: Div<Output = T> + Copy,
{
    type Output = Vector2D<T>;

    fn div(self, scalar: T) -> Vector2D<T> {
        Vector2D {
            x: self.x / scalar,
            y: self.y / scalar,
        }
    }
}

impl<T> Neg for Vector2D<T>
where
    T: Mul<Output = T> + Copy + From<f64>,
{
    type Output = Vector2D<T>;

    fn neg(self) -> Vector2D<T> {
        Vector2D {
            x: self.x * T::from(-1.0),
            y: self.y * T::from(-1.0),
        }
    }
}

impl<'a, T> Neg for &'a Vector2D<T>
where
    T: Mul<Output = T> + Copy + From<f64>,
{
    type Output = Vector2D<T>;

    fn neg(self) -> Vector2D<T> {
        Vector2D {
            x: self.x * T::from(-1.0),
            y: self.y * T::from(-1.0),
        }
    }
}

impl<T: Add<Output = T> + Copy> AddAssign for Vector2D<T> {
    fn add_assign(&mut self, other: Vector2D<T>) {
        self.x = self.x + other.x;
        self.y = self.y + other.y;
    }
}

impl<T: Sub<Output = T> + Copy> SubAssign for Vector2D<T> {
    fn sub_assign(&mut self, other: Vector2D<T>) {
        self.x = self.x - other.x;
        self.y = self.y - other.y;
    }
}

impl<T: Mul<Output = T> + Copy> MulAssign<T> for Vector2D<T> {
    fn mul_assign(&mut self, scalar: T) {
        self.x = self.x * scalar;
        self.y = self.y * scalar;
    }
}

impl<T: Div<Output = T> + Copy> DivAssign<T> for Vector2D<T> {
    fn div_assign(&mut self, scalar: T) {
        self.x = self.x / scalar;
        self.y = self.y / scalar;
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

        let v3 = v1 * 2.0;
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

    #[test]
    fn test_relative_to() {
        let v1 = Vector2D::from_xy(1.0, 2.0);
        let v2 = Vector2D::from_xy(3.0, 4.0);
        let v3 = v1.relative_to(&v2);
        assert_eq!(v3.x, -2.0);
        assert_eq!(v3.y, -2.0);
    }
}
