use std::sync::Arc;

use crate::{color::Color, material::Context, utils::Perlin};

use super::Texture;

pub struct Noise {
    noise: Arc<Perlin>,
}

impl Noise {
    pub fn new(noise: Arc<Perlin>) -> Self {
        Self { noise }
    }
}

impl Texture for Noise {
    fn value(&self, ctx: Context) -> Color {
        todo!()
    }
}
