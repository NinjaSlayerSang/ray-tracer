#![allow(unused)]

use crate::vec3::{F64Tuple3, Vec3};

pub type Color = Vec3;

const SCALE: f64 = 255f64;

pub const BLACK: F64Tuple3 = (0f64, 0f64, 0f64);
pub const RED: F64Tuple3 = (1f64, 0f64, 0f64);
pub const GREEN: F64Tuple3 = (0f64, 1f64, 0f64);
pub const BLUE: F64Tuple3 = (0f64, 0f64, 1f64);
pub const WHITE: F64Tuple3 = (1f64, 1f64, 1f64);

impl Color {
    pub fn into_rgb_str(&self) -> String {
        format!(
            "{:.0} {:.0} {:.0}",
            SCALE * self.x,
            SCALE * self.y,
            SCALE * self.z
        )
    }
}
