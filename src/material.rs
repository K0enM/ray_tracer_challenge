use crate::{color::Color, light::Light, tuple::Tuple, util::FuzzyEq};

#[derive(Debug, Clone, Copy, PartialEq, PartialOrd, Builder)]
pub struct Material {
    pub color: Color,
    pub ambient: f64,
    pub diffuse: f64,
    pub specular: f64,
    pub shininess: f64,
}

impl Material {
    pub fn new(color: Color, ambient: f64, diffuse: f64, specular: f64, shininess: f64) -> Self {
        Self {
            color,
            ambient,
            diffuse,
            specular,
            shininess,
        }
    }

    pub fn with_color(color: Color) -> Self {
        Self {
            color,
            ..Default::default()
        }
    }

    pub fn lighting(
        &self,
        point: Tuple,
        light: Light,
        eyev: Tuple,
        normalv: Tuple,
        in_shadow: bool,
    ) -> Color {
        let effective_color = self.color * light.intensity;
        let lightv = (light.position - point).normalize();
        let ambient = effective_color * self.ambient;
        let diffuse;
        let specular;
        let light_dot_normal = lightv.dot(normalv);

        if light_dot_normal < 0.0 {
            diffuse = Color::black();
            specular = Color::black();
        } else {
            diffuse = effective_color * self.diffuse * light_dot_normal;
            let reflectv = -lightv.reflect(normalv);
            let reflect_dot_eye = reflectv.dot(eyev);

            if reflect_dot_eye <= 0.0 {
                specular = Color::black();
            } else {
                let factor = reflect_dot_eye.powf(self.shininess);
                specular = light.intensity * self.specular * factor;
            }
        }

        if in_shadow {
            ambient
        } else {
            ambient + diffuse + specular
        }
    }
}

impl Default for Material {
    fn default() -> Self {
        Self::new(Color::white(), 0.1, 0.9, 0.9, 200.0)
    }
}

impl FuzzyEq<Self> for Material {
    fn fuzzy_eq(&self, other: Self) -> bool {
        self.color.fuzzy_eq(other.color)
            && self.ambient.fuzzy_eq(other.ambient)
            && self.diffuse.fuzzy_eq(other.diffuse)
            && self.specular.fuzzy_eq(other.specular)
            && self.shininess.fuzzy_eq(other.shininess)
    }

    fn fuzzy_ne(&self, other: Self) -> bool {
        !self.fuzzy_eq(other)
    }
}

#[cfg(test)]
mod tests {
    use crate::assert_fuzzy_eq;

    use super::*;

    #[test]
    fn default_material() {
        let m = Material::default();
        assert_fuzzy_eq!(Color::white(), m.color);
        assert_fuzzy_eq!(0.1, m.ambient);
        assert_fuzzy_eq!(0.9, m.diffuse);
        assert_fuzzy_eq!(0.9, m.specular);
        assert_fuzzy_eq!(200.0, m.shininess);
    }

    #[test]
    fn lighting_with_eye_between_light_and_surface() {
        let material = Material::default();
        let position = Tuple::point(0.0, 0.0, 0.0);

        let eyev = Tuple::vector(0.0, 0.0, -1.0);
        let normalv = Tuple::vector(0.0, 0.0, -1.0);
        let light = Light::point(Tuple::point(0.0, 0.0, -10.0), Color::white());

        let expected = Color::new(1.9, 1.9, 1.9);
        let actual = material.lighting(position, light, eyev, normalv, false);

        assert_fuzzy_eq!(expected, actual);
    }

    #[test]
    fn lighting_with_eye_between_light_and_surface_offset_45_deg() {
        let material = Material::default();
        let position = Tuple::point(0.0, 0.0, 0.0);

        let sqrt_2_2 = (2.0_f64.sqrt()) / 2.0;
        let eyev = Tuple::vector(0.0, sqrt_2_2, -sqrt_2_2);
        let normalv = Tuple::vector(0.0, 0.0, -1.0);
        let light = Light::point(Tuple::point(0.0, 0.0, -10.0), Color::white());

        let expected = Color::new(1.0, 1.0, 1.0);
        let actual = material.lighting(position, light, eyev, normalv, false);

        assert_fuzzy_eq!(expected, actual);
    }

    #[test]
    fn lighting_with_eye_between_light_and_light_offset_45_deg() {
        let material = Material::default();
        let position = Tuple::point(0.0, 0.0, 0.0);

        let eyev = Tuple::vector(0.0, 0.0, -1.0);
        let normalv = Tuple::vector(0.0, 0.0, -1.0);
        let light = Light::point(Tuple::point(0.0, 10.0, -10.0), Color::white());

        let expected = Color::new(0.7364, 0.7364, 0.7364);
        let actual = material.lighting(position, light, eyev, normalv, false);

        assert_fuzzy_eq!(expected, actual);
    }

    #[test]
    fn lighting_with_eye_in_path_of_reflection_vector() {
        let material = Material::default();
        let position = Tuple::point(0.0, 0.0, 0.0);

        let sqrt_2_2 = (2.0_f64.sqrt()) / 2.0;
        let eyev = Tuple::vector(0.0, -sqrt_2_2, -sqrt_2_2);
        let normalv = Tuple::vector(0.0, 0.0, -1.0);
        let light = Light::point(Tuple::point(0.0, 10.0, -10.0), Color::white());

        let expected = Color::new(1.6364, 1.6364, 1.6364);
        let actual = material.lighting(position, light, eyev, normalv, false);

        assert_fuzzy_eq!(expected, actual);
    }

    #[test]
    fn lighting_with_light_behind_surface() {
        let material = Material::default();
        let position = Tuple::point(0.0, 0.0, 0.0);

        let eyev = Tuple::vector(0.0, 0.0, -1.0);
        let normalv = Tuple::vector(0.0, 0.0, -1.0);
        let light = Light::point(Tuple::point(0.0, 0.0, 10.0), Color::white());

        let expected = Color::new(0.1, 0.1, 0.1);
        let actual = material.lighting(position, light, eyev, normalv, false);

        assert_fuzzy_eq!(expected, actual);
    }

    #[test]
    fn lighting_with_surface_in_shadow() {
        let material = Material::default();
        let position = Tuple::point(0.0, 0.0, 0.0);

        let eyev = Tuple::vector(0.0, 0.0, -1.0);
        let normalv = Tuple::vector(0.0, 0.0, -1.0);
        let light = Light::point(Tuple::point(0.0, 0.0, -10.0), Color::white());
        let in_shadow = true;

        let expected = Color::new(0.1, 0.1, 0.1);
        let actual = material.lighting(position, light, eyev, normalv, in_shadow);

        assert_fuzzy_eq!(expected, actual);
    }
}
