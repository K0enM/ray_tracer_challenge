use std::sync::Mutex;

use crate::{canvas::Canvas, matrix::Matrix, ray::Ray, tuple::Tuple, world::World};
#[allow(unused_imports)]
use indicatif::{ProgressBar, ProgressStyle};
use itertools::Itertools;
use rayon::prelude::*;

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Camera {
    pub hsize: usize,
    pub vsize: usize,
    pub fov: f64,
    pub transform: Matrix<4>,
    half_width: f64,
    half_height: f64,
    pixel_size: f64,
}

impl Camera {
    pub fn new(hsize: usize, vsize: usize, fov: f64) -> Self {
        let half_view = (fov / 2.0).tan();
        let aspect = hsize as f64 / vsize as f64;
        let half_width;
        let half_height;

        if aspect >= 1.0 {
            half_width = half_view;
            half_height = half_view / aspect as f64;
        } else {
            half_width = half_view * aspect as f64;
            half_height = half_view;
        }

        let pixel_size = (half_width * 2.0) / hsize as f64;
        Self {
            hsize,
            vsize,
            fov,
            transform: Matrix::identity(),
            half_width,
            half_height,
            pixel_size,
        }
    }

    pub fn set_transform(&mut self, t: Matrix<4>) {
        self.transform = t;
    }

    pub fn ray_for_pixel(&self, x: usize, y: usize) -> Ray {
        let xoffset: f64 = (x as f64 + 0.5) * self.pixel_size;
        let yoffset: f64 = (y as f64 + 0.5) * self.pixel_size;

        let world_x = self.half_width - xoffset;
        let world_y = self.half_height - yoffset;

        let inverse_view_transform = self.transform.inverse();

        let wall_point = inverse_view_transform * Tuple::point(world_x, world_y, -1.0);
        let origin = inverse_view_transform * Tuple::point(0.0, 0.0, 0.0);

        let direction = (wall_point - origin).normalize();
        Ray::new(origin, direction)
    }

    pub fn render(&self, w: &World) -> Canvas {
        #[cfg(feature = "progress_bar")]
            let sty = ProgressStyle::with_template(
                "[{elapsed_precise}] {bar:100.white} {pos:>7}/{len:7} {msg}",
            )
            .unwrap();
            #[cfg(feature = "progress_bar")]
            let pb = ProgressBar::new((self.hsize * self.vsize) as u64);
            #[cfg(feature = "progress_bar")]
            pb.set_style(sty);
        let canvas_mutex = Mutex::new(Canvas::new(self.hsize, self.vsize));

        (0..self.hsize - 1)
            .cartesian_product(0..self.vsize - 1)
            .par_bridge()
            .for_each(|(x, y)| {
                let ray = self.ray_for_pixel(x, y);
                let color = w.color_at(ray);
                let mut canvas = canvas_mutex.lock().unwrap();
                canvas.write_pixel(x, y, color);
                #[cfg(feature = "progress_bar")]
                pb.inc(1)
            });
        #[cfg(feature = "progress_bar")]
        pb.finish_with_message("Done rendering!");
        canvas_mutex.into_inner().unwrap()
        }
}

#[cfg(test)]
mod tests {
    use std::f64::consts::PI;

    use crate::{assert_fuzzy_eq, color::Color, tuple::Tuple, util::FuzzyEq};

    use super::*;

    #[test]
    fn constructing_a_camera() {
        let hsize = 160;
        let vsize = 120;
        let fov = PI / 2.0;
        let c = Camera::new(hsize, vsize, fov);

        assert_eq!(hsize, c.hsize);
        assert_eq!(vsize, c.vsize);
        fov.fuzzy_eq(c.fov);
        assert_fuzzy_eq!(Matrix::identity(), c.transform);
    }

    #[test]
    fn pixel_size_horizontal_canvas() {
        let c = Camera::new(200, 125, PI / 2.0);
        0.01.fuzzy_eq(c.pixel_size);
    }

    #[test]
    fn pixel_size_vertical_canvas() {
        let c = Camera::new(125, 200, PI / 2.0);
        0.01.fuzzy_eq(c.pixel_size);
    }

    #[test]
    fn construct_ray_through_center_of_canvas() {
        let c = Camera::new(201, 101, PI / 2.0);
        let r = c.ray_for_pixel(100, 50);

        assert_fuzzy_eq!(Tuple::point(0.0, 0.0, 0.0), r.origin);
        assert_fuzzy_eq!(Tuple::vector(0.0, 0.0, -1.0), r.direction);
    }

    #[test]
    fn construct_ray_through_corner_of_canvas() {
        let c = Camera::new(201, 101, PI / 2.0);
        let r = c.ray_for_pixel(0, 0);

        assert_fuzzy_eq!(Tuple::point(0.0, 0.0, 0.0), r.origin);
        assert_fuzzy_eq!(Tuple::vector(0.66519, 0.33259, -0.66851), r.direction);
    }

    #[test]
    fn construct_ray_when_camera_is_transformed() {
        let mut c = Camera::new(201, 101, PI / 2.0);
        c.set_transform(Matrix::rotation_y(PI / 4.0) * Matrix::translation(0.0, -2.0, 5.0));

        let r = c.ray_for_pixel(100, 50);
        assert_fuzzy_eq!(Tuple::point(0.0, 2.0, -5.0), r.origin);
        assert_fuzzy_eq!(
            Tuple::vector((2.0_f64.sqrt()) / 2.0, 0.0, -((2.0_f64.sqrt()) / 2.0)),
            r.direction
        );
    }

    #[test]
    fn rendering_world_with_camera() {
        let w = World::default();
        let mut c = Camera::new(11, 11, PI / 2.0);
        let from = Tuple::point(0.0, 0.0, -5.0);
        let to = Tuple::point(0.0, 0.0, 0.0);
        let up = Tuple::vector(0.0, 1.0, 0.0);
        c.set_transform(Matrix::view_transform(from, to, up));
        let img_canvas = c.render(&w);
        assert_fuzzy_eq!(
            Color::new(0.38066, 0.47583, 0.2855),
            img_canvas.pixel_at(5, 5)
        );
    }
}
