mod checker;
mod image_map;
mod marble;
mod solid_color;

use crate::{color::Color, material::Context};

pub use {checker::Checker, image_map::ImageMap, marble::Marble, solid_color::SolidColor};

pub trait Texture {
    fn value(&self, ctx: Context) -> Color;
}
