use crate::vec3::Vec3;

pub type Color = Vec3;

const SCALE: f64 = 255f64;

impl Color {
    pub fn white() -> Color {
        Color::new(1, 1, 1)
    }

    pub fn red() -> Color {
        Color::new(1, 0, 0)
    }

    pub fn green() -> Color {
        Color::new(0, 1, 0)
    }

    pub fn blue() -> Color {
        Color::new(0, 0, 1)
    }

    pub fn gamma_correction(self, gamma: f64) -> Self {
        let exp = gamma.recip();
        Self::new(self.x().powf(exp), self.y().powf(exp), self.z().powf(exp))
    }

    pub fn into_rgb_str(&self) -> String {
        let (min, max) = (0f64, 1f64);
        format!(
            "{:.0} {:.0} {:.0}",
            SCALE * self.x().clamp(min, max),
            SCALE * self.y().clamp(min, max),
            SCALE * self.z().clamp(min, max)
        )
    }
}
