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
