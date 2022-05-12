use crate::vec3::Vec3;

pub mod primary_color {
    #![allow(dead_code)]

    use crate::vec3::F64Tuple3;

    pub const BLACK: F64Tuple3 = (0f64, 0f64, 0f64);
    pub const RED: F64Tuple3 = (1f64, 0f64, 0f64);
    pub const GREEN: F64Tuple3 = (0f64, 1f64, 0f64);
    pub const BLUE: F64Tuple3 = (0f64, 0f64, 1f64);
    pub const WHITE: F64Tuple3 = (1f64, 1f64, 1f64);
}

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
