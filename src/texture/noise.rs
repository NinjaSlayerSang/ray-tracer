use crate::{color::Color, material::Context, utils::Perlin};

use super::Texture;

#[derive(Clone)]
pub struct Noise {
    perlin: Perlin,
    scale: f64,
}

impl Noise {
    pub fn new(scale: f64) -> Self {
        Self {
            perlin: Perlin::default(),
            scale,
        }
    }
}

impl Texture for Noise {
    fn value(&self, ctx: Context) -> Color {
        match ctx {
            Context::UVP { p, .. } => self.perlin.noise(self.scale * p) * Color::white(),
            _ => Color::default(),
        }
    }
}
