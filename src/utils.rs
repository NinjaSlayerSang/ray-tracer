#![allow(unused)]

use rand::Rng;
use std::{
    cmp::Ordering,
    ops::{Range, RangeInclusive},
};

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

pub fn random_double(range: Range<f64>) -> f64 {
    rand::thread_rng().gen_range(range)
}

pub trait Clamp
where
    Self: Copy + PartialOrd,
{
    fn clamp(self, range: RangeInclusive<Self>) -> Self {
        if range.contains(&self) {
            self
        } else {
            self
        }
    }
}
