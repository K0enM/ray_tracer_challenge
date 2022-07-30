use crate::{ray::Ray, tuple::Tuple, util::FuzzyEq, intersection::{Intersection, Intersections}, matrix::Matrix};

#[derive(Debug, Copy, Clone, PartialEq, PartialOrd)]
pub struct Sphere {
    pub transform: Matrix<4>
}

impl Sphere {
    pub fn new(t: Matrix<4>) -> Self {
        Self {
            transform: t
        }
    }

    pub fn intersect(&self, ray: Ray) -> Intersections {
        let object_space_ray = ray.transform(self.transform.inverse());
        let sphere_to_ray = object_space_ray.origin - Tuple::point(0.0, 0.0, 0.0);

        let a = object_space_ray.direction.dot(object_space_ray.direction);
        let b = 2.0 * object_space_ray.direction.dot(sphere_to_ray);
        let c = sphere_to_ray.dot(sphere_to_ray) - 1.0;
        let discriminant = b.powi(2) - 4.0 * a * c;

        if discriminant < 0.0 {
          return Intersections::new(vec![])
        }

        let t1 = Intersection::new((-b - discriminant.sqrt()) / (2.0 * a), *self);
        let t2 = Intersection::new((-b + discriminant.sqrt()) / (2.0 * a), *self);

        Intersections::new(vec![t1, t2])
    }

    pub fn set_transform(&mut self, t: Matrix<4>) {
        self.transform = t
    }
}

impl Default for Sphere {
    fn default() -> Self {
        Self::new(Matrix::identity())
    }
}

impl FuzzyEq<Self> for Sphere {
    fn fuzzy_eq(&self, other: Self) -> bool {
        if self.transform.fuzzy_eq(other.transform) {
            return true;
        }

        false
    }

    fn fuzzy_ne(&self, other: Self) -> bool {
        !self.fuzzy_eq(other)
    }
}

#[cfg(test)]
mod tests {
    use crate::assert_fuzzy_eq;

    use super::*;

    #[test]
    fn ray_intersects_sphere_at_two_points() {
        let r = Ray::new(Tuple::point(0.0, 0.0, -5.0), Tuple::vector(0.0, 0.0, 1.0));
        let s = Sphere::default();

        let xs = s.intersect(r);
        assert_eq!(2, xs.intersections.len());
        assert_eq!(4.0, xs.intersections[0].t);
        assert_eq!(6.0, xs.intersections[1].t);
    }

    #[test]
    fn ray_intersects_sphere_at_tangent() {
        let r = Ray::new(Tuple::point(0.0, 1.0, -5.0), Tuple::vector(0.0, 0.0, 1.0));
        let s = Sphere::default();

        let xs = s.intersect(r);
        assert_eq!(2, xs.intersections.len());
        assert_fuzzy_eq!(5.0, xs.intersections[0].t);
        assert_fuzzy_eq!(5.0, xs.intersections[1].t);
    }

    #[test]
    fn ray_misses_sphere() {
        let r = Ray::new(Tuple::point(0.0, 2.0, -5.0), Tuple::vector(0.0, 0.0, 1.0));
        let s = Sphere::default();

        let xs = s.intersect(r);
        assert_eq!(0, xs.intersections.len())
    }

    #[test]
    fn ray_originates_inside_sphere() {
        let r = Ray::new(Tuple::point(0.0, 0.0, 0.0), Tuple::vector(0.0, 0.0, 1.0));
        let s = Sphere::default();

        let xs = s.intersect(r);
        assert_eq!(2, xs.intersections.len());
        assert_fuzzy_eq!(-1.0, xs.intersections[0].t);
        assert_fuzzy_eq!(1.0, xs.intersections[1].t);
    }

    #[test]
    fn sphere_is_behind_ray() {
        let r = Ray::new(Tuple::point(0.0, 0.0, 5.0), Tuple::vector(0.0, 0.0, 1.0));
        let s = Sphere::default();

        let xs = s.intersect(r);
        assert_eq!(2, xs.intersections.len());
        assert_eq!(-6.0, xs.intersections[0].t);
        assert_eq!(-4.0, xs.intersections[1].t);
    }

    #[test]
    fn sphere_default_transformation() {
        let s  = Sphere::default();
        assert_fuzzy_eq!(Matrix::identity(), s.transform);
    }

    #[test]
    fn changing_sphere_transformation() {
        let t = Matrix::translation(2.0, 3.0, 4.0);
        let s = Sphere::new(t);
        assert_fuzzy_eq!(t, s.transform);
    }
}
