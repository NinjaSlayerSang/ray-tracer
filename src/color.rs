#![allow(unused)]

pub type Color = crate::vec3::Vec3;

const D: f64 = 255.0;

impl Color {
    pub fn into_rgb_str(self) -> String {
        format!("{:.0} {:.0} {:.0}", D * self.x, D * self.y, D * self.z)
    }
}
