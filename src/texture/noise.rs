use std::sync::Arc;

use crate::{color::Color, material::Context, utils::Perlin};

use super::Texture;

#[derive(Clone)]
pub struct Noise(Arc<Perlin>);

impl Noise {
    pub fn new(noise: Arc<Perlin>) -> Self {
        Self(noise)
    }
}

impl Texture for Noise {
    fn value(&self, ctx: Context) -> Color {
        match ctx {
            Context::UVP { p, .. } => self.0.noise(p) * Color::white(),
            _ => Color::default(),
        }
    }
}
