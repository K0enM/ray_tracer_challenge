use crate::{
    color::Color,
    intersection::{ComputedIntersection, Intersections},
    light::Light,
    material::Material,
    matrix::Matrix,
    ray::Ray,
    shape::{Shape, ShapeFuncs},
    sphere::{SphereBuilder},
    tuple::Tuple,
};

#[derive(Debug, Clone, PartialEq, PartialOrd, Builder)]
pub struct World {
    #[builder(default)]
    pub objects: Vec<Shape>,
    #[builder(default)]
    pub light_source: Light,
}

impl World {
    pub fn new(objects: Vec<Shape>, light_source: Light) -> Self {
        Self {
            objects,
            light_source,
        }
    }

    pub fn intersect(&self, ray: Ray) -> Intersections {
        let xs = self.objects.iter().flat_map(|o| o.intersect(ray)).collect();

        Intersections::new(xs)
    }

    pub fn shade_hit(&self, comp: ComputedIntersection) -> Color {
        let in_shadow = self.is_shadowed(comp.over_point);

        comp.intersection.object.material().lighting(
            comp.point,
            self.light_source,
            comp.eyev,
            comp.normalv,
            in_shadow,
        )
    }

    pub fn color_at(&self, ray: Ray) -> Color {
        let xs = self.intersect(ray);
        let hit = xs.hit();

        match hit {
            None => Color::black(),
            Some(i) => {
                let comp = i.as_computed(ray);
                self.shade_hit(comp)
            }
        }
    }

    pub fn is_shadowed(&self, point: Tuple) -> bool {
        let v = self.light_source.position - point;
        let distance = v.magnitude();
        let direction = v.normalize();

        let ray = Ray::new(point, direction);
        let xs = self.intersect(ray);
        let hit = xs.hit();
        match hit {
            None => false,
            Some(i) => i.t < distance,
        }
    }
}

impl Default for World {
    fn default() -> Self {
        let light = Light::default();
        let material = Material::new(Color::new(0.8, 1.0, 0.6), 0.1, 0.7, 0.2, 200.0);

        let s1 = SphereBuilder::default()
            .material(material)
            .build()
            .unwrap()
            .into();
        let s2 = SphereBuilder::default()
            .transform(Matrix::scaling(0.5, 0.5, 0.5))
            .build()
            .unwrap()
            .into();

        Self::new(vec![s1, s2], light)
    }
}

#[cfg(test)]
mod tests {
    use crate::{assert_fuzzy_eq, intersection::Intersection, util::FuzzyEq};

    use super::*;

    #[test]
    fn default_world() {
        let light = Light::point(Tuple::point(-10.0, 10.0, -10.0), Color::white());
        let material = Material::new(Color::new(0.8, 1.0, 0.6), 0.1, 0.7, 0.2, 200.0);
        let s1 = SphereBuilder::default()
            .material(material)
            .build()
            .unwrap()
            .into();
        let s2 = SphereBuilder::default()
            .transform(Matrix::scaling(0.5, 0.5, 0.5))
            .build()
            .unwrap()
            .into();

        let w = World::default();

        assert_eq!(light, w.light_source);
        assert!(w.objects.contains(&s1));
        assert!(w.objects.contains(&s2));
    }

    #[test]
    fn intersect_world_with_ray() {
        let w = World::default();
        let r = Ray::new(Tuple::point(0.0, 0.0, -5.0), Tuple::vector(0.0, 0.0, 1.0));

        let xs = w.intersect(r);
        assert_eq!(4, xs.intersections.len());
        xs.intersections[0].t.fuzzy_eq(4.0);
        xs.intersections[1].t.fuzzy_eq(4.5);
        xs.intersections[2].t.fuzzy_eq(5.5);
        xs.intersections[3].t.fuzzy_eq(6.0);
    }

    #[test]
    fn shading_an_intersection() {
        let w = World::default();
        let r = Ray::new(Tuple::point(0.0, 0.0, -5.0), Tuple::vector(0.0, 0.0, 1.0));
        let s = w.objects[0];
        let i = Intersection::new(4.0, s);
        let comp = i.as_computed(r);

        let c = w.shade_hit(comp);
        assert_fuzzy_eq!(Color::new(0.38066, 0.47583, 0.2855), c);
    }

    #[test]
    fn shading_an_intersection_from_inside() {
        let w = World {
            light_source: Light::point(Tuple::point(0.0, 0.25, 0.0), Color::white()),
            ..Default::default()
        };
        let r = Ray::new(Tuple::point(0.0, 0.0, 0.0), Tuple::vector(0.0, 0.0, 1.0));
        let s = w.objects[1];
        let i = Intersection::new(0.5, s);

        let comp = i.as_computed(r);
        let c = w.shade_hit(comp);

        assert_fuzzy_eq!(Color::new(0.90498, 0.90498, 0.90498), c);
    }

    #[test]
    fn shade_hit_is_given_intersection_in_shadow() {
        let w = World {
            light_source: Light::point(Tuple::point(0.0, 0.0, -10.0), Color::white()),
            objects: vec![
                SphereBuilder::default().build().unwrap().into(),
                SphereBuilder::default()
                    .transform(Matrix::translation(0.0, 0.0, 10.0))
                    .build()
                    .unwrap()
                    .into(),
            ],
        };

        let r = Ray::new(Tuple::point(0.0, 0.0, 5.0), Tuple::vector(0.0, 0.0, 1.0));
        let i = Intersection::new(4.0, w.objects[1]);
        let comp = i.as_computed(r);
        let c = w.shade_hit(comp);
        assert_fuzzy_eq!(Color::new(0.1, 0.1, 0.1), c);
    }

    #[test]
    fn color_when_ray_misses() {
        let w = World::default();
        let r = Ray::new(Tuple::point(0.0, 0.0, -5.0), Tuple::vector(0.0, 1.0, 0.0));
        let c = w.color_at(r);
        assert_fuzzy_eq!(Color::black(), c);
    }

    #[test]
    fn color_when_ray_hits() {
        let w = World::default();
        let r = Ray::new(Tuple::point(0.0, 0.0, -5.0), Tuple::vector(0.0, 0.0, 1.0));
        let c = w.color_at(r);
        assert_fuzzy_eq!(Color::new(0.38066, 0.47583, 0.2855), c);
    }

    #[test]
    fn color_with_intersection_behind_ray() {
        let mat = Material {
            ambient: 1.0,
            ..Default::default()
        };
        let shapes = vec![
            SphereBuilder::default()
                .material(mat)
                .build()
                .unwrap()
                .into(),
            SphereBuilder::default()
                .material(mat)
                .build()
                .unwrap()
                .into(),
        ];
        let w = WorldBuilder::default().objects(shapes).build().unwrap();

        let inner = w.objects[1];

        let r = Ray::new(Tuple::point(0.0, 0.0, 0.75), Tuple::vector(0.0, 0.0, -1.0));
        let c = w.color_at(r);

        assert_fuzzy_eq!(inner.material().color, c);
    }

    #[test]
    fn no_shadow_when_nothing_is_colinear_with_point_and_light() {
        let w = World::default();
        let p = Tuple::point(0.0, 10.0, 0.0);

        assert!(!w.is_shadowed(p));
    }

    #[test]
    fn shadow_when_object_is_between_point_and_light() {
        let w = World::default();
        let p = Tuple::point(10.0, -10.0, 10.0);

        assert!(w.is_shadowed(p));
    }

    #[test]
    fn no_shadow_when_object_is_behind_light() {
        let w = World::default();
        let p = Tuple::point(-20.0, 20.0, -20.0);

        assert!(!w.is_shadowed(p));
    }

    #[test]
    fn no_shadown_when_object_is_behind_point() {
        let w = World::default();
        let p = Tuple::point(-2.0, 2.0, -2.0);

        assert!(!w.is_shadowed(p));
    }
}
