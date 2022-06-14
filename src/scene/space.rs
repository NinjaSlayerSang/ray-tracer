use crate::{color::Color, ray::Ray};

use super::Scene;

#[derive(Clone, Copy, Default)]
pub struct Space(Color);

impl Space {
    pub fn new(background: Color) -> Self {
        Self(background)
    }
}

impl Scene for Space {
    fn scene_color(&self, _: &Ray) -> Color {
        self.0
    }
}
