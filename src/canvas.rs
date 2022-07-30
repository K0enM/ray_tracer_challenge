use crate::{color::Color, png::ToPNG, ppm::ToPPM, rgb::ToRgbA32, two_dimensional::TwoDimensional};

#[derive(Debug)]
pub struct Canvas {
    pub width: usize,
    pub height: usize,
    pub pixels: Vec<Color>,
}

impl Canvas {
    pub fn new(width: usize, height: usize) -> Self {
        Self {
            width,
            height,
            pixels: vec![Color::black(); width * height],
        }
    }

    pub fn pixel_at(&self, x: usize, y: usize) -> Color {
        self.pixels[self.get_pixel_index(x, y)]
    }

    pub fn write_pixel(&mut self, x: usize, y: usize, c: Color) {
        let index = self.get_pixel_index(x, y);
        self.pixels[index] = c;
    }

    fn get_pixel_index(&self, x: usize, y: usize) -> usize {
        y * self.width + x
    }

    fn pixels_as_rgba32(&self) -> Vec<u8> {
        self.pixels.iter().flat_map(|c| c.to_rgba32()).collect()
    }
}

impl TwoDimensional for Canvas {
    fn width(&self) -> usize {
        self.width
    }

    fn height(&self) -> usize {
        self.height
    }
}

impl ToPPM for Canvas {
    fn to_ppm(&self) -> Vec<u8> {
        let mut last_image_row: usize = 0;
        let mut column_count: usize = 0;

        let rgb_colors: Vec<u8> = self.pixels_as_rgba32();

        let pixel_data = rgb_colors
            .into_iter()
            .map(|byte| format!("{}", byte))
            .enumerate()
            .filter(|(i, _)| (i + 1) % 4 != 0)
            .enumerate()
            .flat_map(|(i, (_, pixel_string))| {
                let mut data: Vec<u8> = Vec::new();

                let current_image_row = i / (self.width() * 3);
                if current_image_row != last_image_row {
                    last_image_row = current_image_row;
                    data.extend(String::from("\n").into_bytes());
                    column_count = 0;
                }

                let mut needed_space: usize = 0;

                if column_count != 0 {
                    needed_space += 1;
                }

                needed_space += pixel_string.len();

                if column_count + needed_space > 70 {
                    data.extend(String::from("\n").into_bytes());
                    column_count = 0;
                }

                if column_count != 0 {
                    data.extend(String::from(" ").into_bytes());
                    column_count += 1;
                }

                data.extend(pixel_string.clone().into_bytes());
                column_count += pixel_string.len();

                data
            });

        self.header()
            .into_iter()
            .chain(pixel_data)
            .chain(String::from("\n").into_bytes())
            .collect()
    }
}

impl ToPNG for Canvas {
    fn to_png(self) -> Vec<u8> {
        let mut data = Vec::new();
        let mut encoder = png::Encoder::new(&mut data, self.width() as u32, self.height() as u32);
        encoder.set_color(png::ColorType::Rgba);
        encoder.set_depth(png::BitDepth::Eight);
        let mut writer = encoder.write_header().unwrap();
        writer.write_image_data(&self.pixels_as_rgba32()).unwrap();

        drop(writer);

        data
    }
}

#[cfg(test)]
mod tests {
    use crate::util::FuzzyEq;

    use super::*;

    #[test]
    fn create_canvas() {
        let c = Canvas::new(10, 20);

        assert_eq!(10, c.width);
        assert_eq!(20, c.height);

        for x in 0..c.width {
            for y in 0..c.height {
                assert!(c.pixel_at(x, y).fuzzy_eq(Color::black()))
            }
        }
    }

    #[test]
    fn write_pixels_to_canvas() {
        let mut c = Canvas::new(10, 20);
        let red = Color::red();

        c.write_pixel(2, 3, red);
        assert_eq!(c.pixel_at(2, 3), red);
    }

    #[test]
    fn constructing_ppm_header() {
        let c = Canvas::new(5, 3);

        let expected = String::from("P3\n5 3\n255\n").into_bytes();
        let actual = c.header();

        assert_eq!(actual, expected);
    }

    #[test]
    fn constructing_ppm_pixel_data() {
        let mut canvas = Canvas::new(5, 3);
        let c1 = Color::new(1.5, 0.0, 0.0);
        let c2 = Color::new(0.0, 0.5, 0.0);
        let c3 = Color::new(-0.5, 0.0, 1.0);

        canvas.write_pixel(0, 0, c1);
        canvas.write_pixel(2, 1, c2);
        canvas.write_pixel(4, 2, c3);

        let actual = canvas.to_ppm();
        let header = String::from("P3\n5 3\n255\n").into_bytes();
        let pixel_data = String::from(
      "255 0 0 0 0 0 0 0 0 0 0 0 0 0 0\n0 0 0 0 0 0 0 128 0 0 0 0 0 0 0\n0 0 0 0 0 0 0 0 0 0 0 0 0 0 255\n",
    ).into_bytes();
        let mut expected: Vec<u8> = Vec::new();

        expected.extend(header);
        expected.extend(pixel_data);

        assert_eq!(actual, expected)
    }

    #[test]
    fn split_ppm_files_at_70_characters() {
        let mut canvas = Canvas::new(10, 2);
        let color = Color::new(1.0, 0.8, 0.6);

        for x in 0..canvas.width() {
            for y in 0..canvas.height() {
                canvas.write_pixel(x, y, color);
            }
        }

        let actual = canvas.to_ppm();
        let header = String::from("P3\n10 2\n255\n").into_bytes();

        let pixel_data = String::from(
      "255 204 153 255 204 153 255 204 153 255 204 153 255 204 153 255 204\n153 255 204 153 255 204 153 255 204 153 255 204 153\n255 204 153 255 204 153 255 204 153 255 204 153 255 204 153 255 204\n153 255 204 153 255 204 153 255 204 153 255 204 153\n",
    ).into_bytes();
        let mut expected: Vec<u8> = Vec::new();
        expected.extend(header);
        expected.extend(pixel_data);

        assert_eq!(actual, expected);
    }
}
