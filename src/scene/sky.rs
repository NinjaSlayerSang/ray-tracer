use crate::{color::Color, ray::Ray, utils::LinearGradientColor, vec3::Vec3};

use super::Scene;

#[derive(Clone, Copy)]
pub struct Sky {
    pole: Vec3,
    linear_gradient: LinearGradientColor,
}

impl Default for Sky {
    fn default() -> Self {
        Self {
            pole: Vec3::new(0, 1, 0),
            linear_gradient: Default::default(),
        }
    }
}

impl Sky {
    pub fn new(pole: Vec3, linear_gradient: LinearGradientColor) -> Self {
        Self {
            pole,
            linear_gradient,
        }
    }
}

impl Scene for Sky {
    fn scene_color(&self, ray: &Ray) -> Color {
        let cos_theta = Vec3::cos_included_angle(ray.direction, self.pole);
        let t = 0.5 * (cos_theta + 1.0);
        self.linear_gradient.linear(t)
    }
}
