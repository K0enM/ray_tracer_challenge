use std::ops::{Add, Mul, Sub};

use crate::{rgb::ToRgbA32, util::FuzzyEq};

#[derive(Debug, Clone, Copy, PartialEq, PartialOrd)]
pub struct Color {
    red: f64,
    green: f64,
    blue: f64,
}

impl Color {
    pub fn new(red: f64, green: f64, blue: f64) -> Self {
        Self { red, green, blue }
    }

    pub fn clamp(&self, lower_bound: f64, upper_bound: f64) -> Self {
        Color::new(
            self.red.max(lower_bound).min(upper_bound),
            self.green.max(lower_bound).min(upper_bound),
            self.blue.max(lower_bound).min(upper_bound),
        )
    }

    pub fn white() -> Self {
        Self {
            red: 1.0,
            green: 1.0,
            blue: 1.0,
        }
    }

    pub fn black() -> Self {
        Self {
            red: 0.0,
            green: 0.0,
            blue: 0.0,
        }
    }

    pub fn red() -> Self {
        Self {
            red: 1.0,
            green: 0.0,
            blue: 0.0,
        }
    }

    pub fn green() -> Self {
        Self {
            red: 0.0,
            green: 1.0,
            blue: 0.0,
        }
    }

    pub fn blue() -> Self {
        Self {
            red: 0.0,
            green: 0.0,
            blue: 1.0,
        }
    }
}

impl FuzzyEq<Self> for Color {
    fn fuzzy_eq(&self, other: Self) -> bool {
        self.red.fuzzy_eq(other.red)
            && self.green.fuzzy_eq(other.green)
            && self.blue.fuzzy_eq(other.blue)
    }

    fn fuzzy_ne(&self, other: Self) -> bool {
        !self.fuzzy_eq(other)
    }
}

impl ToRgbA32 for Color {
    fn to_rgba32(&self) -> Vec<u8> {
        let clamped = self.clamp(0.0, 1.0);
        let mut data: Vec<u8> = Vec::new();

        let r = (clamped.red * 255.0).round() as u8;
        let g = (clamped.green * 255.0).round() as u8;
        let b = (clamped.blue * 255.0).round() as u8;
        let a: u8 = 255;

        data.push(r);
        data.push(g);
        data.push(b);
        data.push(a);

        data
    }
}

impl Add<Self> for Color {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        Color::new(
            self.red + rhs.red,
            self.green + rhs.green,
            self.blue + rhs.blue,
        )
    }
}

impl Sub<Self> for Color {
    type Output = Self;

    fn sub(self, rhs: Self) -> Self::Output {
        Color::new(
            self.red - rhs.red,
            self.green - rhs.green,
            self.blue - rhs.blue,
        )
    }
}

impl Mul<f64> for Color {
    type Output = Self;

    fn mul(self, rhs: f64) -> Self::Output {
        Color::new(self.red * rhs, self.green * rhs, self.blue * rhs)
    }
}

impl Mul<Self> for Color {
    type Output = Self;

    fn mul(self, rhs: Self) -> Self::Output {
        Color::new(
            self.red * rhs.red,
            self.green * rhs.green,
            self.blue * rhs.blue,
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn colors_are_rgb_tuples() {
        let a = Color::new(-0.5, 0.4, 1.7);

        let expected = Color::new(-0.5, 0.4, 1.7);

        assert!(a.fuzzy_eq(expected));
    }

    #[test]
    fn adding_colors() {
        let a = Color::new(0.9, 0.6, 0.75);
        let b = Color::new(0.7, 0.1, 0.25);

        let expected = Color::new(1.6, 0.7, 1.0);
        let actual = a + b;

        assert!(actual.fuzzy_eq(expected));
    }

    #[test]
    fn subtracting_colors() {
        let a = Color::new(0.9, 0.6, 0.75);
        let b = Color::new(0.7, 0.1, 0.25);

        let expected = Color::new(0.2, 0.5, 0.5);
        let actual = a - b;

        assert!(actual.fuzzy_eq(expected));
    }

    #[test]
    fn multiply_color_by_scalar() {
        let a = Color::new(0.9, 0.6, 0.75);
        let scalar = 2.0;

        let expected = Color::new(1.8, 1.2, 1.5);
        let actual = a * scalar;

        assert!(actual.fuzzy_eq(expected));
    }

    #[test]
    fn multiply_two_colors() {
        let a = Color::new(1.0, 0.2, 0.4);
        let b = Color::new(0.9, 1.0, 0.1);

        let expected = Color::new(0.9, 0.2, 0.04);
        let actual = a * b;

        assert!(actual.fuzzy_eq(expected));
    }

    #[test]
    fn clamping_colors() {
        let c = Color::new(2.3, -6.7, 0.8);

        let expected = Color::new(1.0, 0.0, 0.8);
        let actual = c.clamp(0.0, 1.0);

        assert!(actual.fuzzy_eq(expected));
    }

    #[test]
    fn color_to_rgba32() {
        let c = Color::new(1.5, 0.0, 0.0);
        let expected: Vec<u8> = vec![255_u8, 0_u8, 0_u8, 255_u8];
        let actual = c.to_rgba32();

        assert_eq!(actual, expected);
    }
}
