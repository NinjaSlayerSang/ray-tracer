mod sky;
mod space;

use crate::{color::Color, ray::Ray};

pub use {sky::Sky, space::Space};

pub trait Scene {
    fn scene_color(&self, ray: &Ray) -> Color;
}
