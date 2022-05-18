mod sky;

use crate::{color::Color, ray::Ray};

pub use sky::Sky;

pub trait Scene {
    fn scene_color(&self, ray: &Ray) -> Color;
}
