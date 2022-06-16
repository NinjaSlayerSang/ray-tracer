use crate::{
    color::Color,
    material::Context,
    utils::{LinearGradientColor, Perlin},
    vec3::Vec3,
};

use super::Texture;

#[derive(Clone)]
pub struct Marble {
    perlin: Perlin,
    gradient: LinearGradientColor,
    depth: i32,
    turbulent_period: f64,
    scale: Vec3,
}

impl Marble {
    pub fn new(scale: Vec3) -> Self {
        Self {
            perlin: Perlin::default(),
            gradient: LinearGradientColor::default(),
            depth: 7,
            turbulent_period: 10.0,
            scale,
        }
    }
}

impl Marble {
    pub fn set_gradient(mut self, gradient: LinearGradientColor) -> Self {
        self.gradient = gradient;
        self
    }

    pub fn set_depth(mut self, depth: i32) -> Self {
        self.depth = depth;
        self
    }

    pub fn set_turbulent_period(mut self, turbulent_period: f64) -> Self {
        self.turbulent_period = turbulent_period;
        self
    }
}

impl Texture for Marble {
    fn value(&self, ctx: Context) -> Color {
        match ctx {
            Context::UVP { p, .. } => self.gradient.linear(
                0.5 * (1.0
                    + (Vec3::dot(p, self.scale)
                        + self.turbulent_period * self.perlin.turb(p, self.depth))
                    .sin()),
            ),
            _ => Color::default(),
        }
    }
}
