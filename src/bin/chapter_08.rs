use std::f64::consts::PI;
use std::fs::write;

use ray_tracer_challenge::{
    camera::Camera,
    color::Color,
    light::Light,
    material::Material,
    matrix::Matrix,
    png::ToPNG,
    shape::Shape,
    sphere::{SphereBuilder},
    tuple::Tuple,
    world::World, plane::PlaneBuilder,
};

fn main() {
    let floor_material = Material {
        color: Color::new(1.0, 0.9, 0.9),
        specular: 0.0,
        ..Default::default()
    };

    let floor: Shape = PlaneBuilder::default()
        .material(floor_material)
        .transform(Matrix::translation(0.0, 0.0, 0.0))
        .build()
        .unwrap()
        .into();
        

    let middle_material = Material {
        color: Color::new(0.5, 1.0, 0.1),
        diffuse: 0.7,
        specular: 0.3,
        ..Default::default()
    };

    let middle: Shape = SphereBuilder::default()
        .material(middle_material)
        .transform(Matrix::translation(-0.5, 1.0, 0.5) * Matrix::scaling(0.5, 0.5, 0.5))
        .build()
        .unwrap()
        .into();

    let right_material = Material {
        color: Color::new(0.5, 1.0, 0.1),
        diffuse: 0.7,
        specular: 0.3,
        ..Default::default()
    };

    let right = SphereBuilder::default()
        .material(right_material)
        .transform(Matrix::translation(1.5, 0.5, -0.5) * Matrix::scaling(0.33, 0.33, 0.33))
        .build()
        .unwrap()
        .into();

    let left_material = Material {
        color: Color::new(1.0, 0.8, 0.1),
        diffuse: 0.7,
        specular: 0.3,
        ..Default::default()
    };

    let left = SphereBuilder::default()
        .material(left_material)
        .transform(Matrix::translation(-1.5, 0.33, -0.75) * Matrix::scaling(0.33, 0.33, 0.33))
        .build()
        .unwrap()
        .into();

    let light = Light::point(Tuple::point(-10.0, 10.0, -10.0), Color::white());

    let world = World::new(
        vec![floor, left, middle, right],
        light,
    );
    let mut camera = Camera::new(4096, 4096, PI / 3.0);

    camera.set_transform(Matrix::view_transform(
        Tuple::point(0.0, 1.5, -5.0),
        Tuple::point(0.0, 1.0, 0.0),
        Tuple::vector(0.0, 1.0, 0.0),
    ));
    let canvas = camera.render(&world);

    println!("Writing ./output.png");
    let png = canvas.to_png();
    write("./output.png", png).expect("Could not write ouput.png to disk.");
}
