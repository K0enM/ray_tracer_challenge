pub const EPSILON: f64 = 0.00001;

pub trait FuzzyEq<T: Clone> {
    fn fuzzy_eq(&self, other: T) -> bool;

    fn fuzzy_ne(&self, other: T) -> bool;
}

impl FuzzyEq<f64> for f64 {
    fn fuzzy_eq(&self, other: f64) -> bool {
        (*self - other).abs() < EPSILON
    }

    fn fuzzy_ne(&self, other: f64) -> bool {
        !self.fuzzy_eq(other)
    }
}