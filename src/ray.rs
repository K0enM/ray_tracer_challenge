use crate::{tuple::Tuple, util::FuzzyEq, matrix::Matrix};

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Ray {
    pub origin: Tuple,
    pub direction: Tuple,
}

impl FuzzyEq<Self> for Ray {
    fn fuzzy_eq(&self, other: Self) -> bool {
        if self.origin.fuzzy_eq(other.origin) && self.direction.fuzzy_eq(other.direction) {
            return true;
        }

        false
    }

    fn fuzzy_ne(&self, other: Self) -> bool {
        !self.fuzzy_eq(other)
    }
}

impl Ray {
    pub fn new(origin: Tuple, direction: Tuple) -> Self {
        if !origin.is_point() || !direction.is_vector() {
            panic!("Given origin or vector are not of the correct tuple type")
        }
        Ray { origin, direction }
    }

    pub fn position(&self, t: f64) -> Tuple {
        self.origin + self.direction * t
    }

    pub fn transform(&self, m: Matrix<4>) -> Self {
        Self { origin: m * self.origin, direction: m * self.direction }
    }
}

#[cfg(test)]
mod tests {
    use crate::assert_fuzzy_eq;

    use super::*;

    #[test]
    fn create_and_query_ray() {
        let origin = Tuple::point(1.0, 2.0, 3.0);
        let direction = Tuple::vector(4.0, 5.0, 6.0);

        let ray = Ray::new(origin, direction);
        assert_fuzzy_eq!(origin, ray.origin);
        assert_fuzzy_eq!(direction, ray.direction);
    }

    #[test]
    fn compute_point_from_a_distance() {
        let r = Ray::new(Tuple::point(2.0, 3.0, 4.0), Tuple::vector(1.0, 0.0, 0.0));
        assert_fuzzy_eq!(r.position(0.0), Tuple::point(2.0, 3.0, 4.0));
        assert_fuzzy_eq!(r.position(1.0), Tuple::point(3.0, 3.0, 4.0));
        assert_fuzzy_eq!(r.position(-1.0), Tuple::point(1.0, 3.0, 4.0));
        assert_fuzzy_eq!(r.position(2.5), Tuple::point(4.5, 3.0, 4.0));
    }

    #[test]
    fn translating_a_ray() {
        let r = Ray::new(Tuple::point(1.0, 2.0, 3.0), Tuple::vector(0.0, 1.0, 0.0));
        let m = Matrix::translation(3.0, 4.0, 5.0);

        let r2 = r.transform(m);
        assert_fuzzy_eq!(Tuple::point(4.0, 6.0, 8.0), r2.origin);
        assert_fuzzy_eq!(Tuple::vector(0.0, 1.0, 0.0), r2.direction);
    }

    #[test]
    fn scaling_a_ray() {
        let r = Ray::new(Tuple::point(1.0, 2.0, 3.0), Tuple::vector(0.0, 1.0, 0.0));
        let m = Matrix::scaling(2.0, 3.0, 4.0);

        let r2 = r.transform(m);
        assert_fuzzy_eq!(Tuple::point(2.0, 6.0, 12.0), r2.origin);
        assert_fuzzy_eq!(Tuple::vector(0.0, 3.0, 0.0), r2.direction);
    }
}
