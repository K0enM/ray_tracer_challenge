use crate::{matrix::Matrix, color::Color, shape::{Shape, ShapeFuncs}, tuple::Tuple};

pub trait PatternFuncs {
    fn color_at(&self, point: Tuple) -> Color;
    fn transform(&self) -> Matrix<4>;
}

#[derive(Debug, Clone, Copy, PartialEq, PartialOrd)]
pub enum Pattern {
    Stripe(StripePattern),
    Gradient(GradientPattern),
    Ring(RingPattern),
    Checker3D(CheckerPattern3D),
}

impl Pattern {
    fn color_at_object(&self, object: Shape, point: Tuple) -> Color {
        let object_point = object.world_point_to_object_point(point);
        let pattern_point = self.transform().inverse() * object_point;

        self.color_at(pattern_point)
    }
}

impl PatternFuncs for Pattern {
    fn color_at(&self, point: Tuple) -> Color {
        match self {
            Self::Stripe(s) => s.color_at(point),
            Self::Gradient(g) => g.color_at(point),
            Self::Ring(r) => r.color_at(point),
            Self::Checker3D(c) => c.color_at(point)
        }
    }

    fn transform(&self) -> Matrix<4> {
        match self {
            Self::Stripe(s) => s.transform(),
            Self::Gradient(g) => g.transform(),
            Self::Ring(r) => r.transform(),
            Self::Checker3D(c) => c.transform()
        }
    }
}

impl From<StripePattern> for Pattern {
    fn from(s: StripePattern) -> Self {
        Self::Stripe(s)
    }
}

impl From<GradientPattern> for Pattern {
    fn from(g: GradientPattern) -> Self {
        Self::Gradient(g)
    }
}

impl From<RingPattern> for Pattern {
    fn from(r: RingPattern) -> Self {
        Self::Ring(r)
    }
}

impl From<CheckerPattern3D> for Pattern {
    fn from(c: CheckerPattern3D) -> Self {
        Self::Checker3D(c)
    }
}

#[derive(Debug, Clone, Copy, PartialEq, PartialOrd, Builder)]
pub struct StripePattern {
    #[builder(default)]
    pub transform: Matrix<4>,
    #[builder(default)]
    pub color_a: Color,
    #[builder(default)]
    pub color_b: Color,
}

impl Default for StripePattern {
    fn default() -> Self {
        Self { transform: Matrix::identity(), color_a: Color::white(), color_b: Color::black() }
    }
}

impl PatternFuncs for StripePattern {
    fn color_at(&self, point: Tuple) -> Color {
        if point.x.floor() as i64 % 2 == 0 {
            return self.color_a
        }

        self.color_b
    }

    fn transform(&self) -> Matrix<4> {
        self.transform
    }
}

#[derive(Debug, Clone, Copy, PartialEq, PartialOrd, Builder)]
pub struct GradientPattern {
    #[builder(default)]
    pub transform: Matrix<4>,
    #[builder(default)]
    pub color_a: Color,
    #[builder(default)]
    pub color_b: Color, 
}

impl Default for GradientPattern {
    fn default() -> Self {
        Self { transform: Matrix::identity(), color_a: Color::white(), color_b: Color::black() }
    }
}

impl PatternFuncs for GradientPattern {
    fn transform(&self) -> Matrix<4> {
        self.transform
    }

    fn color_at(&self, point: Tuple) -> Color {
        self.color_a + (self.color_b - self.color_a) * (point.x - point.x.floor())  
    }
}

#[derive(Debug, Clone, Copy, PartialEq, PartialOrd, Builder)]
pub struct RingPattern {
    #[builder(default)]
    pub transform: Matrix<4>,
    #[builder(default)]
    pub color_a: Color,
    #[builder(default)]
    pub color_b: Color, 
}

impl Default for RingPattern {
    fn default() -> Self {
        Self { transform: Matrix::identity(), color_a: Color::white(), color_b: Color::black() }
    }
}

impl PatternFuncs for RingPattern {
    fn transform(&self) -> Matrix<4> {
        self.transform
    }

    fn color_at(&self, point: Tuple) -> Color {
        if (point.x.powi(2) + point.z.powi(2)).sqrt() as i64 % 2 == 0 {
            return self.color_a
        }

        self.color_b
    }
}

#[derive(Debug, Clone, Copy, PartialEq, PartialOrd, Builder)]
pub struct CheckerPattern3D {
    #[builder(default)]
    pub transform: Matrix<4>,
    #[builder(default)]
    pub color_a: Color,
    #[builder(default)]
    pub color_b: Color, 
}

impl Default for CheckerPattern3D {
    fn default() -> Self {
        Self { transform: Matrix::identity(), color_a: Color::white(), color_b: Color::black() }
    }
}

impl PatternFuncs for CheckerPattern3D {
    fn transform(&self) -> Matrix<4> {
        self.transform
    }

    fn color_at(&self, point: Tuple) -> Color {
        if (point.x.floor() + point.y.floor() + point.z.floor()) as i64 % 2 == 0 {
            return self.color_a
        }

        self.color_b
    }
}

#[cfg(test)]
mod tests {
    use crate::assert_fuzzy_eq;
    use crate::sphere::{SphereBuilder};
    use crate::util::FuzzyEq;
    use super::*;

    #[test]
    fn creating_stripe_pattern() {
        let stripe = StripePatternBuilder::default().color_b(Color::white() ).build().unwrap();

        assert_fuzzy_eq!(Color::black(), stripe.color_a);
        assert_fuzzy_eq!(Color::white(), stripe.color_b);
    }

    #[test]
    fn stripe_pattern_is_constant_in_y() {
        let p: Pattern = StripePattern::default().into();
        assert_fuzzy_eq!(Color::white(), p.color_at(Tuple::point(0.0, 0.0, 0.0)));
        assert_fuzzy_eq!(Color::white(), p.color_at(Tuple::point(0.0, 1.0, 0.0)));
        assert_fuzzy_eq!(Color::white(), p.color_at(Tuple::point(0.0, 2.0, 0.0)));
    }

    #[test]
    fn stripe_pattern_is_constant_in_z() {
        let p: Pattern = StripePattern::default().into();
        assert_fuzzy_eq!(Color::white(), p.color_at(Tuple::point(0.0, 0.0, 0.0)));
        assert_fuzzy_eq!(Color::white(), p.color_at(Tuple::point(0.0, 0.0, 1.0)));
        assert_fuzzy_eq!(Color::white(), p.color_at(Tuple::point(0.0, 0.0, 2.0)));
    }
    
    #[test]
    fn stripe_pattern_alternates_in_x() {
        let p: Pattern = StripePattern::default().into();
        assert_fuzzy_eq!(Color::white(), p.color_at(Tuple::point(0.0, 0.0, 0.0)));
        assert_fuzzy_eq!(Color::white(), p.color_at(Tuple::point(0.9, 0.0, 0.0)));
        assert_fuzzy_eq!(Color::black(), p.color_at(Tuple::point(1.0, 0.0, 0.0)));
        assert_fuzzy_eq!(Color::black(), p.color_at(Tuple::point(-0.1, 0.0, 0.0)));
        assert_fuzzy_eq!(Color::black(), p.color_at(Tuple::point(-1.0, 0.0, 0.0)));
        assert_fuzzy_eq!(Color::white(), p.color_at(Tuple::point(-1.1, 0.0, 0.0)));
    }

    #[test]
    fn stripe_with_object_transformation() {
        let object: Shape = SphereBuilder::default().transform(Matrix::scaling(2.0, 2.0, 2.0)).build().unwrap().into();
        let pattern: Pattern = StripePattern::default().into();

        let c = pattern.color_at_object(object, Tuple::point(1.5, 0.0, 0.0));
        assert_fuzzy_eq!(Color::white(), c);
    }

    #[test]
    fn stripe_with_pattern_transformation() {
        let object: Shape = SphereBuilder::default().build().unwrap().into();
        let pattern: Pattern = StripePatternBuilder::default().transform(Matrix::scaling(2.0, 2.0, 2.0)).color_a(Color::white())
            .build()
            .unwrap()
            .into();
     
        let c = pattern.color_at_object(object, Tuple::point(1.5, 0.0, 0.0));
        assert_fuzzy_eq!(Color::white(), c);
    }

    #[test]
    fn stripes_with_object_and_pattern_transformation() {
        let object: Shape = SphereBuilder::default().transform(Matrix::scaling(2.0, 2.0, 2.0)).build().unwrap().into();
        let pattern: Pattern = StripePatternBuilder::default()
            .color_a(Color::white())
            .transform(Matrix::translation(0.5, 0.0, 0.0))
            .build()
            .unwrap()
            .into();

        let c = pattern.color_at_object(object, Tuple::point(2.5, 0.0, 0.0));
        assert_fuzzy_eq!(Color::white(), c);
    }

    #[test]
    fn gradient_lineary_interpolates_between_colors() {
        let p: Pattern = GradientPattern::default().into();
        assert_fuzzy_eq!(Color::white(), p.color_at(Tuple::point(0.0, 0.0, 0.0)));
        assert_fuzzy_eq!(Color::new(0.75, 0.75, 0.75), p.color_at(Tuple::point(0.25, 0.0, 0.0)));
        assert_fuzzy_eq!(Color::new(0.5, 0.5, 0.5), p.color_at(Tuple::point(0.5, 0.0, 0.0)));
        assert_fuzzy_eq!(Color::new(0.25, 0.25, 0.25), p.color_at(Tuple::point(0.75, 0.0, 0.0)));
    }

    #[test]
    fn ring_pattern_should_extend_both_x_and_z_direction() {
        let p: Pattern = RingPattern::default().into();
        assert_fuzzy_eq!(Color::white(), p.color_at(Tuple::point(0.0, 0.0, 0.0)));
        assert_fuzzy_eq!(Color::black(), p.color_at(Tuple::point(1.0, 0.0, 0.0)));
        assert_fuzzy_eq!(Color::black(), p.color_at(Tuple::point(0.0, 0.0, 1.0)));
        assert_fuzzy_eq!(Color::black(), p.color_at(Tuple::point(0.708, 0.0, 0.708)));
    }

    #[test]
    fn checkers_should_repeat_in_x() {
        let p: Pattern = CheckerPattern3D::default().into();
        assert_fuzzy_eq!(Color::white(), p.color_at(Tuple::point(0.0, 0.0, 0.0)));
        assert_fuzzy_eq!(Color::white(), p.color_at(Tuple::point(0.99, 0.0, 0.0)));
        assert_fuzzy_eq!(Color::black(), p.color_at(Tuple::point(1.01, 0.0, 0.0)));
    }

    #[test]
    fn checkers_should_repeat_in_y() {
        let p: Pattern = CheckerPattern3D::default().into();
        assert_fuzzy_eq!(Color::white(), p.color_at(Tuple::point(0.0, 0.0, 0.0)));
        assert_fuzzy_eq!(Color::white(), p.color_at(Tuple::point(0.0, 0.99, 0.0)));
        assert_fuzzy_eq!(Color::black(), p.color_at(Tuple::point(0.0, 1.01, 0.0)));
    }

    #[test]
    fn checkers_should_repeat_in_z() {
        let p: Pattern = CheckerPattern3D::default().into();
        assert_fuzzy_eq!(Color::white(), p.color_at(Tuple::point(0.0, 0.0, 0.0)));
        assert_fuzzy_eq!(Color::white(), p.color_at(Tuple::point(0.0, 0.0, 0.99)));
        assert_fuzzy_eq!(Color::black(), p.color_at(Tuple::point(0.0, 0.0, 1.01)));
    }
}