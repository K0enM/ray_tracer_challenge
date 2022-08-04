use std::fmt::Debug;

use crate::{ray::Ray, intersection::{Intersections}, tuple::Tuple, sphere::Sphere, util::FuzzyEq, material::Material, matrix::Matrix};

pub trait ShapeFuncs {
    fn intersect(&self, ray: Ray) -> Intersections;
    fn normal_at(&self, object_point: Tuple) -> Tuple;
    fn world_point_to_object_point(&self, world_point: Tuple) -> Tuple;
    fn material(&self) -> Material;
    fn transform(&self) -> Matrix<4>;
}

#[derive(Debug, PartialEq, PartialOrd, Clone, Copy)]
pub enum Shape {
    Sphere(Sphere),
}

impl ShapeFuncs for Shape {
    fn intersect(&self, ray: Ray) -> Intersections {
        match self {
            Self::Sphere(s) => s.intersect(ray)
        }
    }

    fn normal_at(&self, object_point: Tuple) -> Tuple {
        match self {
            Self::Sphere(s) => s.normal_at(object_point)
        }
    }

    fn world_point_to_object_point(&self, world_point: Tuple) -> Tuple {
        match self {
            Self::Sphere(s) => s.world_point_to_object_point(world_point)
        }
    }

    fn material(&self) -> Material {
        match self {
            Self::Sphere(s) => s.material
        }
    }

    fn transform(&self) -> Matrix<4> {
        match self {
            Self::Sphere(s) => s.transform
        }
    }
}

impl From<Sphere> for Shape {
    fn from(s: Sphere) -> Self {
        Self::Sphere(s)
    }
}

impl FuzzyEq<Self> for Shape {
    fn fuzzy_eq(&self, other: Self) -> bool {
        let other = match other {
            Self::Sphere(o) => o
        };

        match self {
            Self::Sphere(s) => s.fuzzy_eq(other)
        }
    }

    fn fuzzy_ne(&self, other: Self) -> bool {
        !self.fuzzy_eq(other)
    }
}