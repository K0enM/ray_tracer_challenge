#![allow(incomplete_features)]
#![feature(generic_const_exprs)]
#[macro_use]
extern crate derive_builder;

pub mod camera;
pub mod canvas;
pub mod color;
pub mod intersection;
pub mod light;
pub mod material;
pub mod matrix;
pub mod plane;
pub mod png;
pub mod ppm;
pub mod ray;
pub mod rgb;
pub mod shape;
pub mod sphere;
pub mod tuple;
pub mod two_dimensional;
pub mod util;
pub mod world;
