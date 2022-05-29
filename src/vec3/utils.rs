use rand::{thread_rng, Rng};
use std::f64::consts::TAU;

use super::Vec3;

impl Vec3 {
    pub fn random_unit() -> Self {
        let theta = thread_rng().gen_range(0f64..=TAU);
        let phi = thread_rng().gen_range(0f64..=TAU);
        let sin_theta = theta.sin();
        Self(sin_theta * phi.cos(), sin_theta * phi.sin(), theta.cos())
    }
}

impl Into<[f64; 3]> for Vec3 {
    fn into(self) -> [f64; 3] {
        [self.0, self.1, self.2]
    }
}
