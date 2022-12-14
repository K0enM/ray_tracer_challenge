use crate::{color::Color, tuple::Tuple};

#[derive(Debug, Clone, Copy, PartialEq, PartialOrd, Eq)]
pub enum LightType {
    Point,
}

#[derive(Debug, Clone, Copy, PartialEq, PartialOrd)]
pub struct Light {
    typ: LightType,
    pub position: Tuple,
    pub intensity: Color,
}

impl Default for Light {
    fn default() -> Self {
        Self::point(Tuple::point(-10.0, 10.0, -10.0), Color::white())
    }
}

impl Light {
    pub fn new(typ: LightType, position: Tuple, intensity: Color) -> Self {
        Self {
            typ,
            position,
            intensity,
        }
    }

    pub fn point(position: Tuple, intensity: Color) -> Self {
        Self {
            typ: LightType::Point,
            position,
            intensity,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::assert_fuzzy_eq;
    use crate::util::FuzzyEq;

    #[test]
    fn point_light_has_position_and_intensity() {
        let intensity = Color::white();
        let position = Tuple::point(0.0, 0.0, 0.0);
        let light = Light::new(LightType::Point, position, intensity);

        assert_fuzzy_eq!(position, light.position);
        assert_fuzzy_eq!(intensity, light.intensity);
    }
}
