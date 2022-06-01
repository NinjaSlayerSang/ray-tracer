use crate::{color::Color, point3::Point3};

use super::Texture;

pub struct SolidColor(Color);

impl SolidColor {
    pub fn new(color: Color) -> Self {
        Self(color)
    }
}

impl Texture for SolidColor {
    fn value(&self, _: f64, _: f64, _: Point3) -> Color {
        self.0
    }
}
