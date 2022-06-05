mod checker;
mod solid_color;

use crate::{color::Color, material::Context};

pub use {checker::Checker, solid_color::SolidColor};

pub trait Texture {
    fn value(&self, ctx: Context) -> Color;
}
