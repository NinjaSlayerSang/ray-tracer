use rand::{thread_rng, Rng};
use std::{cmp::Ordering, f64::consts::TAU};

use crate::vec3::Vec3;

pub enum QuadraticEquationRealRoot {
    Double(f64, f64),
    Single(f64),
    None,
}

pub fn solve_quadratic_equation(a: f64, hb: f64, c: f64) -> QuadraticEquationRealRoot {
    let d = hb * hb - a * c;
    match d.partial_cmp(&0f64) {
        Some(Ordering::Greater) => {
            let sd = d.sqrt();
            QuadraticEquationRealRoot::Double((-hb - sd) / a, (-hb + sd) / a)
        }
        Some(Ordering::Equal) => QuadraticEquationRealRoot::Single(-hb / a),
        _ => QuadraticEquationRealRoot::None,
    }
}

pub fn random_unit_vec3() -> Vec3 {
    let theta = thread_rng().gen_range(0f64..=TAU);
    let phi = thread_rng().gen_range(0f64..=TAU);
    let sin_theta = theta.sin();
    Vec3::new(sin_theta * phi.cos(), sin_theta * phi.sin(), theta.cos())
}

pub fn reflect_unit_vec3(v: Vec3, n: Vec3) -> Vec3 {
    v - 2f64 * v * n * n
}
