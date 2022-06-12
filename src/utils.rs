use rand::{prelude::SliceRandom, thread_rng, Rng};
use std::{cmp::Ordering, f64::consts::TAU};

use crate::{color::Color, vec3::Vec3};

pub enum QuadraticEquationRealRoot {
    Double(f64, f64),
    Single(f64),
    None,
}

pub fn solve_quadratic_equation(a: f64, hb: f64, c: f64) -> QuadraticEquationRealRoot {
    use Ordering::{Equal, Greater};
    use QuadraticEquationRealRoot::{Double, None, Single};

    let d = hb * hb - a * c;
    match d.partial_cmp(&0f64) {
        Some(Greater) => {
            let sd = d.sqrt();
            Double((-hb - sd) / a, (-hb + sd) / a)
        }
        Some(Equal) => Single(-hb / a),
        _ => None,
    }
}

pub fn random_xy_in_circle(radius: f64) -> (f64, f64) {
    let theta = thread_rng().gen_range(0f64..=TAU);
    let r = thread_rng().gen_range(0f64..=radius);
    (r * theta.cos(), r * theta.sin())
}

fn refractance(cosine: f64, ref_idx: f64) -> bool {
    let r0 = ((1f64 - ref_idx) / (1f64 + ref_idx)).powi(2);
    r0 + (1f64 - r0) * (1f64 - cosine).powi(5) < thread_rng().gen::<f64>()
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
        reflected + fuzz * cos_theta_lv * Vec3::random_unit()
    } else {
        reflected
    }
}

#[derive(Clone, Copy)]
pub struct LinearGradientColor(Color, Color);

impl Default for LinearGradientColor {
    fn default() -> Self {
        Self(Default::default(), Color::white())
    }
}

impl LinearGradientColor {
    pub fn new(zero: Color, one: Color) -> Self {
        Self(zero, one)
    }
}

impl LinearGradientColor {
    pub fn linear(&self, t: f64) -> Color {
        (1f64 - t) * self.0 + t * self.1
    }
}

pub trait Permutable {
    fn permute(&mut self);
}

impl<T> Permutable for [T] {
    fn permute(&mut self) {
        for i in (1..self.len()).rev() {
            self.swap(i, thread_rng().gen_range(0..i))
        }
    }
}

const POINT_COUNT: usize = 256;

#[derive(Clone)]
pub struct Perlin {
    ranfloat: Vec<f64>,
    perm_x: Vec<usize>,
    perm_y: Vec<usize>,
    perm_z: Vec<usize>,
    size: f64,
}

impl Perlin {
    pub fn new(size: f64) -> Self {
        Self {
            ranfloat: Vec::from({
                let mut r = [f64::default(); POINT_COUNT];
                for i in 0..POINT_COUNT {
                    r[i] = thread_rng().gen();
                }
                r
            }),
            perm_x: Self::generate_perm(),
            perm_y: Self::generate_perm(),
            perm_z: Self::generate_perm(),
            size,
        }
    }

    pub fn noise(&self, p: Vec3) -> f64 {
        let i = self.cast_by_size(p.x());
        let j = self.cast_by_size(p.y());
        let k = self.cast_by_size(p.z());

        self.ranfloat[self.perm_x[i] ^ self.perm_y[j] ^ self.perm_z[k]]
    }

    fn cast_by_size(&self, p: f64) -> usize {
        (((p / self.size).floor() as i64) & (POINT_COUNT as i64 - 1)) as usize
    }

    fn generate_perm() -> Vec<usize> {
        let mut p = [usize::default(); POINT_COUNT];

        for i in 0..POINT_COUNT {
            p[i] = i;
        }

        if true {
            // use rand::seq::SliceRandom::shuffle
            p.shuffle(&mut thread_rng());
        } else {
            // use custom Permutable::permute
            p.permute();
        }

        Vec::from(p)
    }
}
