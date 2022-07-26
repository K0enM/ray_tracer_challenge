use std::ops::{Add, Div, Mul, Neg, Sub};

use crate::util::FuzzyEq;

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Tuple {
    pub x: f64,
    pub y: f64,
    pub z: f64,
    pub w: f64,
}

impl Tuple {
    pub fn new(x: f64, y: f64, z: f64, w: f64) -> Self {
        Self { x, y, z, w }
    }

    pub fn point(x: f64, y: f64, z: f64) -> Self {
        Self { x, y, z, w: 1.0 }
    }

    pub fn vector(x: f64, y: f64, z: f64) -> Self {
        Self { x, y, z, w: 0.0 }
    }

    pub fn is_point(self) -> bool {
        if self.w.fuzzy_eq(1.0) {
            return true;
        }

        false
    }

    pub fn is_vector(self) -> bool {
        if self.w.fuzzy_eq(0.0) {
            return true;
        }

        false
    }

    pub fn magnitude(&self) -> f64 {
        let squares = self.x.powi(2) + self.y.powi(2) + self.z.powi(2);
        squares.sqrt()
    }

    pub fn normalize(&self) -> Self {
        let mag = self.magnitude();

        Self {
            x: self.x / mag,
            y: self.y / mag,
            z: self.z / mag,
            w: self.w / mag,
        }
    }

    pub fn dot(&self, other: Tuple) -> f64 {
        self.x * other.x + self.y * other.y + self.z * other.z + self.w * other.w
    }

    pub fn cross(&self, other: Tuple) -> Self {
        Tuple::vector(
            self.y * other.z - self.z * other.y,
            self.z * other.x - self.x * other.z,
            self.x * other.y - self.y * other.x,
        )
    }
}

impl FuzzyEq<Tuple> for Tuple {
    fn fuzzy_eq(&self, other: Self) -> bool {
        self.x.fuzzy_eq(other.x)
            && self.y.fuzzy_eq(other.y)
            && self.z.fuzzy_eq(other.z)
            && self.w.fuzzy_eq(other.w)
    }

    fn fuzzy_ne(&self, other: Self) -> bool {
        !self.fuzzy_eq(other)
    }
}

impl Add<Self> for Tuple {
    type Output = Self;

    fn add(self, rhs: Tuple) -> Self::Output {
        Tuple::new(
            self.x + rhs.x,
            self.y + rhs.y,
            self.z + rhs.z,
            self.w + rhs.w,
        )
    }
}

impl Sub<Self> for Tuple {
    type Output = Self;

    fn sub(self, rhs: Self) -> Self::Output {
        Tuple::new(
            self.x - rhs.x,
            self.y - rhs.y,
            self.z - rhs.z,
            self.w - rhs.w,
        )
    }
}

impl Neg for Tuple {
    type Output = Self;

    fn neg(self) -> Self::Output {
        let zero = Tuple::vector(0.0, 0.0, 0.0);

        zero - self
    }
}

impl Mul<f64> for Tuple {
    type Output = Self;

    fn mul(self, rhs: f64) -> Self::Output {
        Tuple::new(self.x * rhs, self.y * rhs, self.z * rhs, self.w * rhs)
    }
}

impl Div<f64> for Tuple {
    type Output = Self;

    fn div(self, rhs: f64) -> Self::Output {
        Tuple::new(self.x / rhs, self.y / rhs, self.z / rhs, self.w / rhs)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn tuple_with_w_1_is_point() {
        let tuple = Tuple::new(4.3, -4.2, 3.1, 1.0);
        assert!(tuple.is_point());
        assert!(!tuple.is_vector());
    }

    #[test]
    fn tuple_with_w_0_is_vector() {
        let tuple = Tuple::new(4.3, -4.2, 3.1, 0.0);
        assert!(!tuple.is_point());
        assert!(tuple.is_vector());
    }

    #[test]
    fn point_creates_tuple_with_w_1() {
        let point = Tuple::point(4.0, -4.0, 3.0);
        let expected = Tuple::new(4.0, -4.0, 3.0, 1.0);

        assert!(point.fuzzy_eq(expected))
    }

    #[test]
    fn vector_creates_tuple_with_w_0() {
        let vector = Tuple::vector(4.0, -4.0, 3.0);
        let expected = Tuple::new(4.0, -4.0, 3.0, 0.0);

        assert!(vector.fuzzy_eq(expected))
    }

    #[test]
    fn add_two_tuples() {
        let a = Tuple::new(3.0, -2.0, 5.0, 1.0);
        let b = Tuple::new(-2.0, 3.0, 1.0, 0.0);

        let expected = Tuple::new(1.0, 1.0, 6.0, 1.0);
        let actual = a + b;
        assert!(actual.fuzzy_eq(expected))
    }

    #[test]
    fn subtract_two_points() {
        let a = Tuple::point(3.0, 2.0, 1.0);
        let b = Tuple::point(5.0, 6.0, 7.0);

        let expected = Tuple::vector(-2.0, -4.0, -6.0);
        let actual = a - b;

        assert!(actual.fuzzy_eq(expected));
        assert!(actual.is_vector());
    }

    #[test]
    fn subtract_vector_from_point() {
        let p = Tuple::point(3.0, 2.0, 1.0);
        let v = Tuple::vector(5.0, 6.0, 7.0);

        let expected = Tuple::point(-2.0, -4.0, -6.0);
        let actual = p - v;

        assert!(actual.fuzzy_eq(expected));
        assert!(actual.is_point());
    }

    #[test]
    fn subtract_two_vectors() {
        let a = Tuple::vector(3.0, 2.0, 1.0);
        let b = Tuple::vector(5.0, 6.0, 7.0);

        let expected = Tuple::vector(-2.0, -4.0, -6.0);
        let actual = a - b;

        assert!(actual.fuzzy_eq(expected));
        assert!(actual.is_vector());
    }

    #[test]
    fn negate_tuple() {
        let a = Tuple::new(1.0, -2.0, 3.0, -4.0);

        let expected = Tuple::new(-1.0, 2.0, -3.0, 4.0);
        let actual = -a;

        assert!(actual.fuzzy_eq(expected));
    }

    #[test]
    fn multiply_tuple_by_scalar() {
        let a = Tuple::new(1.0, -2.0, 3.0, -4.0);

        let expected = Tuple::new(3.5, -7.0, 10.5, -14.0);
        let actual = a * 3.5;

        assert!(actual.fuzzy_eq(expected))
    }

    #[test]
    fn multiply_tuple_by_fraction() {
        let a = Tuple::new(1.0, -2.0, 3.0, -4.0);

        let expected = Tuple::new(0.5, -1.0, 1.5, -2.0);
        let actual = a * 0.5;

        assert!(actual.fuzzy_eq(expected))
    }

    #[test]
    fn divide_tuple_by_scalar() {
        let a = Tuple::new(1.0, -2.0, 3.0, -4.0);

        let expected = Tuple::new(0.5, -1.0, 1.5, -2.0);
        let actual = a / 2.0;

        assert!(actual.fuzzy_eq(expected))
    }

    #[test]
    fn compute_the_magnitude_of_vector_1_0_0() {
        let a = Tuple::vector(1.0, 0.0, 0.0);

        let expected = 1.0;
        let actual = a.magnitude();

        assert!(actual.fuzzy_eq(expected))
    }

    #[test]
    fn compute_the_magnitude_of_vector_0_1_0() {
        let a = Tuple::vector(1.0, 0.0, 0.0);

        let expected = 1.0;
        let actual = a.magnitude();

        assert!(actual.fuzzy_eq(expected))
    }

    #[test]
    fn compute_the_magnitude_of_vector_0_0_1() {
        let a = Tuple::vector(0.0, 0.0, 1.0);

        let expected = 1.0;
        let actual = a.magnitude();

        assert!(actual.fuzzy_eq(expected))
    }

    #[test]
    fn compute_the_magnitude_of_vector_1_2_3() {
        let a = Tuple::vector(1.0, 2.0, 3.0);

        let expected = 14.0_f64.sqrt();
        let actual = a.magnitude();

        assert!(actual.fuzzy_eq(expected))
    }

    #[test]
    fn compute_the_magnitude_of_negative_vector_1_2_3() {
        let a = Tuple::vector(1.0, 2.0, 3.0);
        let a = -a;

        let expected = 14.0_f64.sqrt();
        let actual = a.magnitude();

        assert!(actual.fuzzy_eq(expected))
    }

    #[test]
    fn normalize_vector_4_0_0_gives_vector_1_0_0() {
        let a = Tuple::vector(4.0, 0.0, 0.0);

        let expected = Tuple::vector(1.0, 0.0, 0.0);
        let actual = a.normalize();

        assert!(actual.fuzzy_eq(expected))
    }

    #[test]
    fn normalize_vector_1_2_3() {
        let a = Tuple::vector(1.0, 2.0, 3.0);

        // Real values -> 1.0/sqrt(14), 2.0/sqrt(14), 3.0/sqrt(14)
        let expected = Tuple::vector(0.26726, 0.53452, 0.80178);
        let actual = a.normalize();

        assert!(actual.fuzzy_eq(expected))
    }

    #[test]
    fn calculate_magnitude_of_normalized_vector() {
        let a = Tuple::vector(1.0, 2.0, 3.0);

        let expected = 1.0;
        let actual = a.normalize().magnitude();

        assert!(actual.fuzzy_eq(expected))
    }

    #[test]
    fn calculate_dot_product_of_two_tuples() {
        let a = Tuple::vector(1.0, 2.0, 3.0);
        let b = Tuple::vector(2.0, 3.0, 4.0);

        let expected = 20.0;
        let actual = a.dot(b);

        assert!(actual.fuzzy_eq(expected));
    }

    #[test]
    fn calculate_cross_product_of_two_vectors() {
        let a = Tuple::vector(1.0, 2.0, 3.0);
        let b = Tuple::vector(2.0, 3.0, 4.0);

        let expected = Tuple::vector(-1.0, 2.0, -1.0);
        let actual = a.cross(b);

        assert!(actual.fuzzy_eq(expected));

        let expected = Tuple::vector(1.0, -2.0, 1.0);
        let actual = b.cross(a);

        assert!(actual.fuzzy_eq(expected));
    }
}
