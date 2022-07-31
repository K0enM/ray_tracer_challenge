use std::f64::consts::PI;
use std::fs::write;

use ray_tracer_challenge::{material::Material, color::Color, sphere::Sphere, matrix::Matrix, light::Light, tuple::Tuple, world::World, camera::Camera, png::ToPNG};

fn main() {
  let floor_material = Material {
    color: Color::new(1.0, 0.9, 0.9),
    specular: 0.0,
    ..Default::default()
  };

  let floor_transform = Matrix::scaling(10.0, 0.01, 10.0);
  let floor = Sphere::new(floor_transform, floor_material);

  let left_wall_transform = Matrix::identity()
    .translate(0.0, 0.0, 5.0)
    .rotate_y(-PI / 4.0)
    .rotate_x(PI / 2.0)
    .scale(10.0, 0.01, 10.0);
  let left_wall = Sphere::new(left_wall_transform, floor_material);

  let right_wall_transform = Matrix::identity()
    .translate(0.0, 0.0, 5.0)
    .rotate_y(PI / 4.0)
    .rotate_x(PI / 2.0)
    .scale(10.0, 0.01, 10.0);
  let right_wall = Sphere::new(right_wall_transform, floor_material);

  let middle_material = Material {
    color: Color::new(0.5, 1.0, 0.1),
    diffuse: 0.7,
    specular: 0.3,
    ..Default::default()
  };
  let middle = Sphere::new(Matrix::translation(-0.5, 1.0, 0.5) * Matrix::scaling(0.5, 0.5, 0.5), middle_material);

  let right_material = Material {
    color: Color::new(0.5, 1.0, 0.1),
    diffuse: 0.7,
    specular: 0.3,
    ..Default::default()
  };
  let right = Sphere::new(Matrix::translation(1.5, 0.5, -0.5) * Matrix::scaling(0.33, 0.33, 0.33), right_material);

  let left_material = Material {
    color: Color::new(1.0, 0.8, 0.1),
    diffuse: 0.7,
    specular: 0.3,
    ..Default::default()
  };
  let left = Sphere::new(Matrix::translation(-1.5, 0.33, -0.75) * Matrix::scaling(0.33, 0.33, 0.33), left_material);

  let light = Light::point(Tuple::point(-10.0, 10.0, -10.0), Color::white());

  let world = World::new(vec![floor, left_wall, right_wall, left, middle, right], light);
  let mut camera = Camera::new(4096, 4096, PI / 3.0);

  camera.set_transform(Matrix::view_transform(Tuple::point(0.0, 1.5, -5.0), Tuple::point(0.0, 1.0, 0.0), Tuple::vector(0.0, 1.0, 0.0)));
  let canvas = camera.render(&world);

  println!("Writing ./output.png");
  let png = canvas.to_png();
  write("./output.png", png).expect("Could not write ouput.png to disk.");
}