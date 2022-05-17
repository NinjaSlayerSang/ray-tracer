mod sky;

use crate::{color::Color, ray::Ray};

pub trait Scene {
    fn scene_color(&self, ray: &Ray) -> Color;
}
