#![allow(unused)]

use std::{cmp::Ordering, f64::consts::PI};

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

pub fn degrees_to_radians(degrees: f64) -> f64 {
    degrees / 180f64 * PI
}
