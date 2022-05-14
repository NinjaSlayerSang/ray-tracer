use std::ops::{Add, AddAssign, Div, DivAssign, Mul, MulAssign, Neg, Sub, SubAssign};

use super::Vec3;

// v1 + v2 = v3
impl Add for Vec3 {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        Self {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
            z: self.z + rhs.z,
        }
    }
}

// -v1 = v2
impl Neg for Vec3 {
    type Output = Self;

    fn neg(self) -> Self::Output {
        Self {
            x: -self.x,
            y: -self.y,
            z: -self.z,
        }
    }
}

// v1 - v2 = v3
impl Sub for Vec3 {
    type Output = Self;

    fn sub(self, rhs: Self) -> Self::Output {
        self + (-rhs)
    }
}

// v1 * v2 = f
impl Mul for Vec3 {
    type Output = f64;

    fn mul(self, rhs: Self) -> Self::Output {
        self.x * rhs.x + self.y * rhs.y + self.z * rhs.z
    }
}

// v1 * f = v2
impl Mul<f64> for Vec3 {
    type Output = Self;

    fn mul(self, rhs: f64) -> Self::Output {
        Self {
            x: self.x * rhs,
            y: self.y * rhs,
            z: self.z * rhs,
        }
    }
}

// f * v1 = v2
impl Mul<Vec3> for f64 {
    type Output = Vec3;

    fn mul(self, rhs: Vec3) -> Self::Output {
        rhs * self
    }
}

// v1 / v2 = f or NaN
impl Div for Vec3 {
    type Output = f64;

    fn div(self, rhs: Self) -> Self::Output {
        let p = self * rhs;
        let ls = self.length();
        let lr = rhs.length();

        if p / (ls * lr) == 1f64 {
            return ls / lr;
        } else {
            return f64::NAN;
        }
    }
}

// v1 / f = v2
impl Div<f64> for Vec3 {
    type Output = Self;

    fn div(self, rhs: f64) -> Self::Output {
        self * (1.0 / rhs)
    }
}

// assign

// v1 += v2
impl AddAssign for Vec3 {
    fn add_assign(&mut self, rhs: Self) {
        self.x += rhs.x;
        self.y += rhs.y;
        self.z += rhs.z;
    }
}

// v1 -= v2
impl SubAssign for Vec3 {
    fn sub_assign(&mut self, rhs: Self) {
        self.x -= rhs.x;
        self.y -= rhs.y;
        self.z -= rhs.z;
    }
}

// v *= f
impl MulAssign<f64> for Vec3 {
    fn mul_assign(&mut self, rhs: f64) {
        self.x *= rhs;
        self.y *= rhs;
        self.z *= rhs;
    }
}

// v /= f
impl DivAssign<f64> for Vec3 {
    fn div_assign(&mut self, rhs: f64) {
        self.x /= rhs;
        self.y /= rhs;
        self.z /= rhs;
    }
}

impl Vec3 {
    pub fn length(self) -> f64 {
        (self * self).sqrt()
    }

    pub fn unit(self) -> Self {
        self / self.length()
    }

    pub fn dot(u: Self, v: Self) -> f64 {
        u * v
    }

    pub fn cross(u: Self, v: Self) -> Self {
        Self {
            x: u.y * v.z - u.z * v.y,
            y: u.z * v.x - u.x * v.z,
            z: u.x * v.y - u.y * v.x,
        }
    }

    pub fn matrix_mul(u: Self, v: Self) -> Self {
        Self {
            x: u.x * v.x,
            y: u.y * v.y,
            z: u.z * v.z,
        }
    }

    pub fn mixed(i: Self, j: Self, k: Self) -> f64 {
        Self::dot(Self::cross(i, j), k)
    }

    pub fn cos_included_angle(u: Self, v: Self) -> f64 {
        u * v / (u.length() * v.length())
    }

    pub fn parallel(u: Self, v: Self) -> bool {
        Self::cos_included_angle(u, v) == 1f64
    }

    pub fn vertical(u: Self, v: Self) -> bool {
        u * v == 0f64
    }

    pub fn veer(self, dir: Self) -> Self {
        self.length() * dir.unit()
    }

    pub fn projection(self, dir: Self) -> Self {
        self * dir / (dir * dir) * dir
    }

    pub fn reflect(self, n: Self) -> Self {
        self - 2f64 * self.projection(n)
    }
}
