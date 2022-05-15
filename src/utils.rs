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

fn refractance(cosine: f64, ref_idx: f64) -> bool {
    let r0 = ((1f64 - ref_idx) / (1f64 + ref_idx)).powi(2);
    r0 + (1f64 - r0) * (1f64 - cosine).powi(5) < thread_rng().gen()
}

pub fn refleract(v: Vec3, un: Vec3, eta: f64, fuzz: f64) -> Vec3 {
    let cos_theta_lv = Vec3::dot(v, un);

    if cos_theta_lv == 0f64 {
        return v;
    }

    if eta > 0f64 {
        let cos_theta_sign = cos_theta_lv > 0f64;
        let e = if cos_theta_sign { eta } else { eta.recip() };
        let vertical = e * (v - cos_theta_lv * un);
        let discriminant = Vec3::dot(v, v) - Vec3::dot(vertical, vertical);
        if discriminant > 0f64 && refractance((cos_theta_lv / v.length()).abs(), e) {
            let parallel = discriminant.sqrt() * if cos_theta_sign { un } else { -un };
            return vertical + parallel;
        }
    }

    let reflected = v - 2f64 * cos_theta_lv * un;

    if fuzz > 0f64 {
        reflected + fuzz * cos_theta_lv * random_unit_vec3()
    } else {
        reflected
    }
}
