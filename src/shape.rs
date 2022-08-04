use std::fmt::Debug;

use crate::{
    intersection::Intersections, material::Material, matrix::Matrix, plane::Plane, ray::Ray,
    sphere::Sphere, tuple::Tuple, util::FuzzyEq,
};

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
    Plane(Plane),
}

impl ShapeFuncs for Shape {
    fn intersect(&self, ray: Ray) -> Intersections {
        match self {
            Self::Sphere(s) => s.intersect(ray),
            Self::Plane(p) => p.intersect(ray),
        }
    }

    fn normal_at(&self, object_point: Tuple) -> Tuple {
        match self {
            Self::Sphere(s) => s.normal_at(object_point),
            Self::Plane(p) => p.normal_at(object_point),
        }
    }

    fn world_point_to_object_point(&self, world_point: Tuple) -> Tuple {
        match self {
            Self::Sphere(s) => s.world_point_to_object_point(world_point),
            Self::Plane(p) => p.world_point_to_object_point(world_point),
        }
    }

    fn material(&self) -> Material {
        match self {
            Self::Sphere(s) => s.material,
            Self::Plane(p) => p.material,
        }
    }

    fn transform(&self) -> Matrix<4> {
        match self {
            Self::Sphere(s) => s.transform,
            Self::Plane(p) => p.transform,
        }
    }
}

impl FuzzyEq<Self> for Shape {
    fn fuzzy_eq(&self, other: Self) -> bool {
        self.material().fuzzy_eq(other.material()) && self.transform().fuzzy_eq(other.transform())
    }

    fn fuzzy_ne(&self, other: Self) -> bool {
        !self.fuzzy_eq(other)
    }
}

impl From<Sphere> for Shape {
    fn from(s: Sphere) -> Self {
        Self::Sphere(s)
    }
}

impl From<Plane> for Shape {
    fn from(p: Plane) -> Self {
        Self::Plane(p)
    }
}
