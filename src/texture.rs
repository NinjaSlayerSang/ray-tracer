mod solid_color;

use crate::{color::Color, point3::Point3};

pub use solid_color::SolidColor;

pub trait Texture {
    fn value(&self, u: f64, v: f64, p: Point3) -> Color;
}
