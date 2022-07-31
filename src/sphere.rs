use crate::{
    intersection::{Intersection, Intersections},
    material::Material,
    matrix::Matrix,
    ray::Ray,
    tuple::Tuple,
    util::FuzzyEq,
};

#[derive(Debug, Copy, Clone, PartialEq, PartialOrd)]
pub struct Sphere {
    pub transform: Matrix<4>,
    pub material: Material,
}

impl Sphere {
    pub fn new(t: Matrix<4>, m: Material) -> Self {
        Self {
            transform: t,
            material: m,
        }
    }

    pub fn with_transform(t: Matrix<4>) -> Self {
        Self {
            transform: t,
            material: Material::default(),
        }
    }

    pub fn with_material(m: Material) -> Self {
        Self { transform: Matrix::identity(), material: m }
    }

    pub fn intersect(&self, ray: Ray) -> Intersections {
        let object_space_ray = ray.transform(self.transform.inverse());
        let sphere_to_ray = object_space_ray.origin - Tuple::point(0.0, 0.0, 0.0);

        let a = object_space_ray.direction.dot(object_space_ray.direction);
        let b = 2.0 * object_space_ray.direction.dot(sphere_to_ray);
        let c = sphere_to_ray.dot(sphere_to_ray) - 1.0;
        let discriminant = b.powi(2) - 4.0 * a * c;

        if discriminant < 0.0 {
            return Intersections::new(vec![]);
        }

        let t1 = Intersection::new((-b - discriminant.sqrt()) / (2.0 * a), *self);
        let t2 = Intersection::new((-b + discriminant.sqrt()) / (2.0 * a), *self);

        Intersections::new(vec![t1, t2])
    }

    pub fn set_transform(&mut self, t: Matrix<4>) {
        self.transform = t
    }

    pub fn set_material(&mut self, m: Material) {
        self.material = m
    }

    pub fn normal_at(&self, world_point: Tuple) -> Tuple {
        let object_point = self.transform.inverse() * world_point;
        let object_normal = object_point - Tuple::point(0.0, 0.0, 0.0);
        let mut world_normal = self.transform.inverse().tranpose() * object_normal;

        world_normal.w = 0.0;
        world_normal.normalize()
    }
}

impl Default for Sphere {
    fn default() -> Self {
        Self::new(Matrix::identity(), Material::default())
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
    use std::f64::consts::PI;

    use crate::{assert_fuzzy_eq, color::Color};

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
        let s = Sphere::default();
        assert_fuzzy_eq!(Matrix::identity(), s.transform);
    }

    #[test]
    fn changing_sphere_transformation() {
        let t = Matrix::translation(2.0, 3.0, 4.0);
        let mut s = Sphere::default();
        s.set_transform(t);
        assert_fuzzy_eq!(t, s.transform);
    }

    #[test]
    fn normal_on_a_sphere_at_a_point_on_the_x_axis() {
        let s = Sphere::default();
        let actual = s.normal_at(Tuple::point(1.0, 0.0, 0.0));
        let expected = Tuple::vector(1.0, 0.0, 0.0);

        assert_fuzzy_eq!(expected, actual)
    }

    #[test]
    fn normal_on_a_sphere_at_a_point_on_the_y_axis() {
        let s = Sphere::default();
        let actual = s.normal_at(Tuple::point(0.0, 1.0, 0.0));
        let expected = Tuple::vector(0.0, 1.0, 0.0);

        assert_fuzzy_eq!(expected, actual);
    }

    #[test]
    fn normal_on_a_sphere_at_a_nonaxial_point() {
        let s = Sphere::default();
        let actual = s.normal_at(Tuple::point(
            (3.0_f64.sqrt()) / 3.0,
            (3.0_f64.sqrt()) / 3.0,
            (3.0_f64.sqrt()) / 3.0,
        ));
        let expected = Tuple::vector(
            (3.0_f64.sqrt()) / 3.0,
            (3.0_f64.sqrt()) / 3.0,
            (3.0_f64.sqrt()) / 3.0,
        );

        assert_fuzzy_eq!(expected, actual);
    }

    #[test]
    fn normal_is_normalized_vector() {
        let s = Sphere::default();
        let actual = s.normal_at(Tuple::point(
            (3.0_f64.sqrt()) / 3.0,
            (3.0_f64.sqrt()) / 3.0,
            (3.0_f64.sqrt()) / 3.0,
        ));
        let expected = actual.normalize();

        assert_fuzzy_eq!(expected, actual);
    }

    #[test]
    #[allow(clippy::approx_constant)]
    fn computing_the_normal_on_a_translated_sphere() {
        let s = Sphere::new(Matrix::translation(0.0, 1.0, 0.0), Material::default());
        let p = Tuple::point(0.0, 1.70711, -0.70711);
        let n = s.normal_at(p);

        let expected_result = Tuple::vector(0.0, 0.70711, -0.70711);

        assert_fuzzy_eq!(n, expected_result);
    }

    #[test]
    fn computing_the_normal_on_a_scaled_and_rotated_sphere() {
        let s = Sphere::new(
            Matrix::scaling(1.0, 0.5, 1.0) * Matrix::rotation_z(PI / 5.0),
            Material::default(),
        );
        let sqrt2_over_2 = (2.0_f64).sqrt() / 2.0;
        let p = Tuple::point(0.0, sqrt2_over_2, -sqrt2_over_2);
        let n = s.normal_at(p);

        let expected_result = Tuple::vector(0.0, 0.97014, -0.24254);

        assert_fuzzy_eq!(n, expected_result);
    }

    #[test]
    fn sphere_has_default_material() {
        let s = Sphere::default();
        assert_fuzzy_eq!(Material::default(), s.material);
    }

    #[test]
    fn sphere_may_be_assigned_material() {
        let mut s = Sphere::default();
        let m = Material::new(Color::black(), 1.0, 2.0, 3.0, 4.0);
        s.set_material(m);

        assert_fuzzy_eq!(m, s.material);
    }
}
