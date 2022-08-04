use crate::{material::Material, matrix::Matrix, shape::{ShapeFuncs, Shape}, tuple::Tuple, util::{FuzzyEq, EPSILON}, ray::Ray, intersection::{Intersections, Intersection}};

#[derive(Debug, Clone, Copy, PartialEq, PartialOrd, Default, Builder)]
pub struct Plane {
    #[builder(default)]
    pub transform: Matrix<4>,
    #[builder(default)]
    pub material: Material,
}

impl FuzzyEq<Self> for Plane {
    fn fuzzy_eq(&self, other: Self) -> bool {
        self.transform.fuzzy_eq(other.transform) && self.material.fuzzy_eq(other.material)
    }

    fn fuzzy_ne(&self, other: Self) -> bool {
        !self.fuzzy_eq(other)
    }
}

impl ShapeFuncs for Plane {
    fn intersect(&self, ray: Ray) -> crate::intersection::Intersections {
        if ray.direction.y.abs() < EPSILON {
            return Intersections::new(vec![])
        }

        let t = -ray.origin.y / ray.direction.y;
        Intersections::new(vec![Intersection::new(t, Shape::from(*self))])
    }

    fn normal_at(&self, _object_point: crate::tuple::Tuple) -> crate::tuple::Tuple {
        Tuple::vector(0.0, 1.0, 0.0)
    }

    fn world_point_to_object_point(&self, world_point: Tuple) -> Tuple {
        self.transform.inverse() * world_point
    }

    fn material(&self) -> Material {
        self.material
    }

    fn transform(&self) -> Matrix<4> {
        self.transform
    }
}

#[cfg(test)]
mod tests {
    use crate::{assert_fuzzy_eq, shape::Shape, ray::Ray};

    use super::*;

    #[test]
    fn normal_of_plane_is_const_everywhere() {
        let p: Shape = PlaneBuilder::default().build().unwrap().into();

        let n1 = p.normal_at(p.world_point_to_object_point(Tuple::point(0.0, 0.0, 0.0)));
        let n2 = p.normal_at(p.world_point_to_object_point(Tuple::point(10.0, 0.0, -10.0)));
        let n3 = p.normal_at(p.world_point_to_object_point(Tuple::point(-5.0, 0.0, 150.0)));

        let expected = Tuple::vector(0.0, 1.0, 0.0);
        assert_fuzzy_eq!(expected, n1);
        assert_fuzzy_eq!(expected, n2);
        assert_fuzzy_eq!(expected, n3);
    }

    #[test]
    fn intersect_with_ray_parallel_to_plane() {
        let p: Shape = PlaneBuilder::default().build().unwrap().into();
        let r = Ray::new(Tuple::point(0.0, 10.0, 0.0), Tuple::vector(0.0, 0.0, 1.0));
        let xs = p.intersect(r);
        assert_eq!(0, xs.intersections.len());
    }

    #[test]
    fn intersect_with_coplanar_ray() {
        let p: Shape = PlaneBuilder::default().build().unwrap().into();
        let r = Ray::new(Tuple::point(0.0, 0.0, 0.0), Tuple::vector(0.0, 0.0, 1.0));

        let xs = p.intersect(r);
        assert_eq!(0, xs.intersections.len());
    }

    #[test]
    fn intersect_plane_from_above() {
        let p: Shape = PlaneBuilder::default().build().unwrap().into();
        let r = Ray::new(Tuple::point(0.0, 1.0, 0.0), Tuple::vector(0.0, -1.0, 0.0));
        
        let xs = p.intersect(r);
        assert_fuzzy_eq!(1.0, xs.intersections[0].t);
        assert_fuzzy_eq!(p, xs.intersections[0].object);
    }

    #[test]
    fn intersect_plane_from_below() {
        let p: Shape = PlaneBuilder::default().build().unwrap().into();
        let r = Ray::new(Tuple::point(0.0, -1.0, 0.0), Tuple::vector(0.0, 1.0, 0.0));
        
        let xs = p.intersect(r);
        assert_fuzzy_eq!(1.0, xs.intersections[0].t);
        assert_fuzzy_eq!(p, xs.intersections[0].object);
    }

}
