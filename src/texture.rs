mod solid_color;

use crate::{color::Color, material::Context};

pub use solid_color::SolidColor;

pub trait Texture {
    fn value(&self, ctx: Context) -> Color;
}
