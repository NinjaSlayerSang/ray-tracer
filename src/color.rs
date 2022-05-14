pub mod primary_color {
    #![allow(dead_code)]

    use super::Color;

    pub const BLACK: Color = Color {
        x: 0f64,
        y: 0f64,
        z: 0f64,
    };
    pub const WHITE: Color = Color {
        x: 1f64,
        y: 1f64,
        z: 1f64,
    };
    pub const RED: Color = Color {
        x: 1f64,
        y: 0f64,
        z: 0f64,
    };
    pub const GREEN: Color = Color {
        x: 0f64,
        y: 1f64,
        z: 0f64,
    };
    pub const BLUE: Color = Color {
        x: 0f64,
        y: 0f64,
        z: 1f64,
    };
}

use crate::vec3::Vec3;

pub type Color = Vec3;

const SCALE: f64 = 255f64;

impl Color {
    pub fn gamma_correction(self, gamma: f64) -> Self {
        let exp = gamma.recip();
        Self {
            x: self.x.powf(exp),
            y: self.y.powf(exp),
            z: self.z.powf(exp),
        }
    }

    pub fn into_rgb_str(&self) -> String {
        let (min, max) = (0f64, 1f64);
        format!(
            "{:.0} {:.0} {:.0}",
            SCALE * self.x.clamp(min, max),
            SCALE * self.y.clamp(min, max),
            SCALE * self.z.clamp(min, max)
        )
    }
}
