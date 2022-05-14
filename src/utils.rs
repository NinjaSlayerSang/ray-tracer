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

pub fn reflect_unit_normal(v: Vec3, unit_n: Vec3) -> Vec3 {
    v - 2 * (Vec3::dot(v, unit_n) * unit_n)
}

pub fn refract_unit_normal(v: Vec3, unit_n: Vec3, etai_over_etat: f64) -> Option<Vec3> {
    let unit_v = v.unit();
    let cos_theta = -Vec3::dot(unit_v, unit_n);
    let out_perp = etai_over_etat * (unit_v + cos_theta * unit_n);
    let k = 1f64 - Vec3::dot(out_perp, out_perp);
    if k.is_sign_positive() {
        let out_parallel = -k.sqrt() * unit_n;
        Some(out_perp + out_parallel)
    } else {
        None
    }
}
