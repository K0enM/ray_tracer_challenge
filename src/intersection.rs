use crate::{ray::Ray, sphere::{Sphere, SphereBuilder}, tuple::Tuple, util::EPSILON, shape::{Shape, ShapeFuncs}};

#[derive(Debug, PartialEq, PartialOrd, Clone, Copy)]
pub struct Intersection {
    pub t: f64,
    pub object: Shape,
}

#[derive(Debug, Clone, PartialEq, PartialOrd, Copy)]
pub struct ComputedIntersection {
    pub intersection: Intersection,
    pub point: Tuple,
    pub over_point: Tuple,
    pub eyev: Tuple,
    pub normalv: Tuple,
    pub inside: bool,
}

#[derive(Debug, Clone, PartialEq)]
pub struct Intersections {
    pub intersections: Vec<Intersection>,
}

impl Intersections {
    pub fn new(mut xs: Vec<Intersection>) -> Self {
        xs.sort_by(|a, b| a.t.partial_cmp(&b.t).unwrap());

        Self { intersections: xs }
    }

    pub fn hit(&self) -> Option<Intersection> {
        for i in self.intersections.iter() {
            if i.t > 0.0 {
                return Some(*i);
            }
        }

        None
    }
}

impl IntoIterator for Intersections {
    type Item = Intersection;
    type IntoIter = std::vec::IntoIter<Self::Item>;

    fn into_iter(self) -> Self::IntoIter {
        self.intersections.into_iter()
    }
}

impl Intersection {
    pub fn new(t: f64, object: Shape) -> Self {
        Self { t, object }
    }

    pub fn as_computed(&self, ray: Ray) -> ComputedIntersection {
        let point = ray.position(self.t);
        let eyev = -ray.direction;
        let mut normalv = self.object.normal_at(point);

        let mut inside = false;

        if (normalv.dot(eyev)) < 0.0 {
            inside = true;
            normalv = -normalv;
        }

        let over_point = point + normalv * EPSILON;

        ComputedIntersection {
            intersection: *self,
            point,
            over_point,
            eyev,
            normalv,
            inside,
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::{
        assert_fuzzy_eq,
        matrix::Matrix,
        ray::Ray,
        sphere::Sphere,
        tuple::Tuple,
        util::{FuzzyEq, EPSILON},
    };

    use super::*;

    #[test]
    fn intersection_encapsulates_t_and_object() {
        let s = Shape::from(Sphere::default());
        let i = Intersection::new(3.5, s);

        assert_eq!(3.5, i.t);
        assert_eq!(s, i.object)
    }

    #[test]
    fn aggregating_intersections() {
        let s = Shape::from(Sphere::default());
        let a = Intersection::new(1.0, s);
        let b = Intersection::new(2.0, s);

        let xs = Intersections::new(vec![a, b]);
        assert_eq!(2, xs.intersections.len());
        assert_eq!(1.0, xs.intersections[0].t);
        assert_eq!(2.0, xs.intersections[1].t);
    }

    #[test]
    fn intersect_sets_the_object_on_the_intersection() {
        let r = Ray::new(Tuple::point(0.0, 0.0, -5.0), Tuple::vector(0.0, 0.0, 1.0));
        let s = Shape::from(Sphere::default());

        let xs = s.intersect(r);
        assert_eq!(2, xs.intersections.len());
        assert_eq!(s, xs.intersections[0].object);
        assert_eq!(s, xs.intersections[1].object);
    }

    #[test]
    fn hit_when_all_intersections_have_positive_t() {
        let s = Shape::from(Sphere::default());
        let a = Intersection::new(1.0, s);
        let b = Intersection::new(2.0, s);
        let xs = Intersections::new(vec![b, a]);

        let i = xs.hit();

        assert!(i.is_some());
        assert_eq!(a, i.unwrap());
    }

    #[test]
    fn hit_when_some_intersections_have_negative_t() {
        let s = Shape::from(Sphere::default());
        let a = Intersection::new(-1.0, s);
        let b = Intersection::new(1.0, s);
        let xs = Intersections::new(vec![b, a]);

        let i = xs.hit();

        assert!(i.is_some());
        assert_eq!(b, i.unwrap());
    }

    #[test]
    fn hit_when_all_intersections_have_negative_t() {
        let s = Shape::from(Sphere::default());
        let a = Intersection::new(-2.0, s);
        let b = Intersection::new(-1.0, s);
        let xs = Intersections::new(vec![b, a]);

        let i = xs.hit();

        assert!(i.is_none());
    }

    #[test]
    fn hit_is_always_lowest_nognegative_intersection() {
        let s = Shape::from(Sphere::default());
        let a = Intersection::new(5.0, s);
        let b = Intersection::new(7.0, s);
        let c = Intersection::new(-3.0, s);
        let d = Intersection::new(2.0, s);
        let xs = Intersections::new(vec![a, b, c, d]);

        let i = xs.hit();

        assert!(i.is_some());
        assert_eq!(d, i.unwrap())
    }

    #[test]
    fn precomputing_state_of_intersection() {
        let r = Ray::new(Tuple::point(0.0, 0.0, -5.0), Tuple::vector(0.0, 0.0, 1.0));
        let s = Shape::from(Sphere::default());
        let i = Intersection::new(4.0, s);
        let comp = i.as_computed(r);

        assert!(comp.intersection.t.fuzzy_eq(i.t));
        assert_fuzzy_eq!(i.object, comp.intersection.object);
        assert_fuzzy_eq!(Tuple::point(0.0, 0.0, -1.0), comp.point);
        assert_fuzzy_eq!(Tuple::vector(0.0, 0.0, -1.0), comp.eyev);
        assert_fuzzy_eq!(Tuple::vector(0.0, 0.0, -1.0), comp.normalv);
    }

    #[test]
    fn hit_when_intersection_occurs_on_outside() {
        let r = Ray::new(Tuple::point(0.0, 0.0, -5.0), Tuple::vector(0.0, 0.0, 1.0));
        let s = Shape::from(Sphere::default());
        let i = Intersection::new(4.0, s);
        let comp = i.as_computed(r);

        assert!(!comp.inside);
    }

    #[test]
    fn hit_when_intersection_is_inside() {
        let r = Ray::new(Tuple::point(0.0, 0.0, 0.0), Tuple::vector(0.0, 0.0, 1.0));
        let s = Shape::from(Sphere::default());
        let i = Intersection::new(1.0, s);
        let comp = i.as_computed(r);

        assert!(comp.inside);
        assert_fuzzy_eq!(Tuple::point(0.0, 0.0, 1.0), comp.point);
        assert_fuzzy_eq!(Tuple::vector(0.0, 0.0, -1.0), comp.eyev);
        assert_fuzzy_eq!(Tuple::vector(0.0, 0.0, -1.0), comp.normalv);
    }

    #[test]
    fn hit_should_offset_point() {
        let r = Ray::new(Tuple::point(0.0, 0.0, -5.0), Tuple::vector(0.0, 0.0, 1.0));
        let s = Shape::from(SphereBuilder::default().transform(Matrix::translation(0.0, 0.0, 1.0)).build().unwrap());
        let i = Intersection::new(5.0, s);
        let comp = i.as_computed(r);

        assert!(comp.over_point.z < -EPSILON / 2.0);
        assert!(comp.point.z > comp.over_point.z);
    }
}
