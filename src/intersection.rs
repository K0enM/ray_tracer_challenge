use crate::sphere::Sphere;

#[derive(Debug, Copy, Clone, PartialEq, PartialOrd)]
pub struct Intersection {
    pub t: f64,
    pub object: Sphere
}

#[derive(Debug, Clone, PartialEq)]
pub struct Intersections {
  pub intersections: Vec<Intersection>
}

impl Intersections {
  pub fn new(mut xs: Vec<Intersection>) -> Self {

    xs.sort_by(|a, b| a.t.partial_cmp(&b.t).unwrap());

    Self {
      intersections:  xs
    }
  }

  pub fn hit(&self) -> Option<Intersection> {
    for i in self.intersections.iter() {
      if i.t > 0.0 {
        return Some(*i)
      }
    }

    None
  }
}

impl Intersection {
  pub fn new(t: f64, object: Sphere) -> Self {
    Self { t, object}
  }
}

#[cfg(test)]
mod tests {
  use crate::{sphere::Sphere, ray::Ray, tuple::Tuple};

use super::*;

  #[test]
  fn intersection_encapsulates_t_and_object() {
    let s = Sphere::default();
    let i = Intersection::new(3.5, s);

    assert_eq!(3.5, i.t);
    assert_eq!(s, i.object)
  }

  #[test]
  fn aggregating_intersections() {
    let s = Sphere::default();
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
    let s = Sphere::default();

    let xs = s.intersect(r);
    assert_eq!(2, xs.intersections.len());
    assert_eq!(s, xs.intersections[0].object);
    assert_eq!(s, xs.intersections[1].object);
  }

  #[test]
  fn hit_when_all_intersections_have_positive_t() {
    let s = Sphere::default();
    let a = Intersection::new(1.0, s);
    let b = Intersection::new(2.0, s);
    let xs = Intersections::new(vec![b, a]);

    let i = xs.hit();
    
    assert!(i.is_some());
    assert_eq!(a, i.unwrap());
  }

  #[test]
  fn hit_when_some_intersections_have_negative_t() {
    let s = Sphere::default();
    let a = Intersection::new(-1.0, s);
    let b = Intersection::new(1.0, s);
    let xs = Intersections::new(vec![b, a]);

    let i = xs.hit();
    
    assert!(i.is_some());
    assert_eq!(b, i.unwrap());
  }

  #[test]
  fn hit_when_all_intersections_have_negative_t() {
    let s = Sphere::default();
    let a = Intersection::new(-2.0, s);
    let b = Intersection::new(-1.0, s);
    let xs = Intersections::new(vec![b, a]);

    let i = xs.hit();
    
    assert!(i.is_none());
  }

  #[test]
  fn hit_is_always_lowest_nognegative_intersection() {
    let s = Sphere::default();
    let a = Intersection::new(5.0, s);
    let b = Intersection::new(7.0, s);
    let c = Intersection::new(-3.0, s);
    let d = Intersection::new(2.0, s);
    let xs = Intersections::new(vec![a, b, c, d]);

    let i = xs.hit();
    
    assert!(i.is_some());
    assert_eq!(d, i.unwrap())
  }
}