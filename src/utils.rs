use std::{cmp::Ordering, ops::RangeInclusive};

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

pub trait ClampRange
where
    Self: Copy + PartialOrd,
{
    fn clamp_range(self, range: RangeInclusive<Self>) -> Self {
        let (&start, &end) = (range.start(), range.end());
        if self < start {
            start
        } else if self > end {
            end
        } else {
            self
        }
    }
}
