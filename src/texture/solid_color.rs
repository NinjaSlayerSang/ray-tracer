use crate::{color::Color, material::Context};

use super::Texture;

#[derive(Clone, Copy)]
pub struct SolidColor(Color);

impl SolidColor {
    pub fn new(color: Color) -> Self {
        Self(color)
    }
}

impl Texture for SolidColor {
    fn value(&self, _: Context) -> Color {
        self.0
    }
}
