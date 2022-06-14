use rand::{distributions::uniform::SampleRange, thread_rng, Rng};

use crate::vec3::Vec3;

pub type Color = Vec3;

const SCALE: f64 = 255f64;

impl Color {
    pub fn white() -> Self {
        Self::new(1, 1, 1)
    }

    pub fn red() -> Self {
        Self::new(1, 0, 0)
    }

    pub fn green() -> Self {
        Self::new(0, 1, 0)
    }

    pub fn blue() -> Self {
        Self::new(0, 0, 1)
    }

    pub fn random() -> Self {
        Self::from(thread_rng().gen::<(f64, f64, f64)>())
    }

    pub fn random_range(range: impl SampleRange<f64> + Clone) -> Self {
        Self::new(
            thread_rng().gen_range(range.clone()),
            thread_rng().gen_range(range.clone()),
            thread_rng().gen_range(range),
        )
    }

    pub fn gamma_correction(self, gamma: f64) -> Self {
        let exp = gamma.recip();
        Self::new(self.x().powf(exp), self.y().powf(exp), self.z().powf(exp))
    }

    pub fn into_rgb_str(self) -> String {
        let (min, max) = (0f64, 1f64);
        format!(
            "{:.0} {:.0} {:.0}",
            SCALE * self.x().clamp(min, max),
            SCALE * self.y().clamp(min, max),
            SCALE * self.z().clamp(min, max)
        )
    }

    pub fn from_rgba(rgba: [u8; 4]) -> Self {
        let [r, g, b, a] = rgba;

        let fr = r as f64 / SCALE;
        let fg = g as f64 / SCALE;
        let fb = b as f64 / SCALE;
        let fa = a as f64 / SCALE;

        Self::new(fr * fa, fg * fa, fb * fa)
    }
}
