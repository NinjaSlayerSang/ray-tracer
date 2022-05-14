pub mod primary_color {
    #![allow(dead_code)]

    use crate::vec3::Vec3;

    use super::Color;

    pub const BLACK: Color = Vec3(0f64, 0f64, 0f64);
    pub const WHITE: Color = Vec3(1f64, 1f64, 1f64);
    pub const RED: Color = Vec3(1f64, 0f64, 0f64);
    pub const GREEN: Color = Vec3(0f64, 1f64, 0f64);
    pub const BLUE: Color = Vec3(0f64, 0f64, 1f64);
}

use crate::vec3::Vec3;

pub type Color = Vec3;

const SCALE: f64 = 255f64;

impl Color {
    pub fn gamma_correction(self, gamma: f64) -> Self {
        let exp = gamma.recip();
        Self(self.0.powf(exp), self.1.powf(exp), self.2.powf(exp))
    }

    pub fn into_rgb_str(&self) -> String {
        let (min, max) = (0f64, 1f64);
        format!(
            "{:.0} {:.0} {:.0}",
            SCALE * self.0.clamp(min, max),
            SCALE * self.1.clamp(min, max),
            SCALE * self.2.clamp(min, max)
        )
    }
}
