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

const U8_SIZE: usize = 256;

#[derive(Clone)]
pub struct Perlin {
    ranvec: Vec<Vec3>,
    perm_x: Vec<usize>,
    perm_y: Vec<usize>,
    perm_z: Vec<usize>,
}

impl Default for Perlin {
    fn default() -> Self {
        Self {
            ranvec: (0..U8_SIZE).map(|_| Vec3::random_unit()).collect(),
            perm_x: Self::generate_perm(),
            perm_y: Self::generate_perm(),
            perm_z: Self::generate_perm(),
        }
    }
}

impl Perlin {
    pub fn noise(&self, p: Vec3) -> f64 {
        let (x, y, z) = p.into();

        let i = x.floor() as i32;
        let j = y.floor() as i32;
        let k = z.floor() as i32;

        let u = x - x.floor();
        let v = y - y.floor();
        let w = z - z.floor();

        let mut c = [[[Vec3::default(); 2]; 2]; 2];

        for di in 0..2 {
            for dj in 0..2 {
                for dk in 0..2 {
                    c[di][dj][dk] = self.ranvec[self.perm_x[Self::cast_index(i + di as i32)]
                        ^ self.perm_y[Self::cast_index(j + dj as i32)]
                        ^ self.perm_z[Self::cast_index(k + dk as i32)]];
                }
            }
        }

        Self::trilinear_interp(&c, u, v, w)
    }

    pub fn turb(&self, p: Vec3, depth: i32) -> f64 {
        let mut accum = 0.0;
        let mut temp_p = p;
        let mut weight = 1.0;

        for _ in 0..depth {
            accum += weight * self.noise(temp_p);
            weight *= 0.5;
            temp_p *= 2;
        }

        accum.abs()
    }

    fn generate_perm() -> Vec<usize> {
        let mut p = [0usize; U8_SIZE];

        for i in 0..U8_SIZE {
            p[i] = i;
        }

        p.shuffle(&mut thread_rng());

        Vec::from(p)
    }

    fn hermite_cubic(x: f64) -> f64 {
        x * x * (3f64 - 2f64 * x)
    }

    fn cast_index(i: i32) -> usize {
        (i & 255) as usize
    }

    fn trilinear_interp(c: &[[[Vec3; 2]; 2]; 2], u: f64, v: f64, w: f64) -> f64 {
        let uu = Self::hermite_cubic(u);
        let vv = Self::hermite_cubic(v);
        let ww = Self::hermite_cubic(w);
        let mut accum = 0f64;

        for i in 0..2 {
            for j in 0..2 {
                for k in 0..2 {
                    let (fi, fj, fk) = (i as f64, j as f64, k as f64);
                    let weight_v = Vec3::new(u - fi, v - fj, w - fk);
                    accum += (fi * uu + (1f64 - fi) * (1f64 - uu))
                        * (fj * vv + (1f64 - fj) * (1f64 - vv))
                        * (fk * ww + (1f64 - fk) * (1f64 - ww))
                        * Vec3::dot(c[i][j][k], weight_v);
                }
            }
        }

        accum
    }
}
