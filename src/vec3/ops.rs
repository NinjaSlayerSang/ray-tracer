use std::ops::{Add, AddAssign, Div, DivAssign, Mul, MulAssign, Neg, Sub, SubAssign};

use super::Vec3;

// base operator

// v1 + v2 = v3
impl Add for Vec3 {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        Self(self.0 + rhs.0, self.1 + rhs.1, self.2 + rhs.2)
    }
}

// v1 - v2 = v3
impl Sub for Vec3 {
    type Output = Self;

    fn sub(self, rhs: Self) -> Self::Output {
        Self(self.0 - rhs.0, self.1 - rhs.1, self.2 - rhs.2)
    }
}

// v1 * v2 = v3
impl Mul for Vec3 {
    type Output = Self;

    fn mul(self, rhs: Self) -> Self::Output {
        Self(self.0 * rhs.0, self.1 * rhs.1, self.2 * rhs.2)
    }
}

// calculation with other type

// v1 * f = v2
impl Mul<f64> for Vec3 {
    type Output = Self;

    fn mul(self, rhs: f64) -> Self::Output {
        Self(self.0 * rhs, self.1 * rhs, self.2 * rhs)
    }
}

// v1 * i = v2
impl Mul<i32> for Vec3 {
    type Output = Self;

    fn mul(self, rhs: i32) -> Self::Output {
        self * rhs as f64
    }
}

// v1 / f = v2
impl Div<f64> for Vec3 {
    type Output = Self;

    fn div(self, rhs: f64) -> Self::Output {
        Self(self.0 / rhs, self.1 / rhs, self.2 / rhs)
    }
}

// v1 / i = v2
impl Div<i32> for Vec3 {
    type Output = Self;

    fn div(self, rhs: i32) -> Self::Output {
        self / rhs as f64
    }
}

// commutation

// f * v1 = v2
impl Mul<Vec3> for f64 {
    type Output = Vec3;

    fn mul(self, rhs: Vec3) -> Self::Output {
        rhs * self
    }
}

// i * v1 = v2
impl Mul<Vec3> for i32 {
    type Output = Vec3;

    fn mul(self, rhs: Vec3) -> Self::Output {
        rhs * self
    }
}

// assign

// v1 += v2
impl AddAssign for Vec3 {
    fn add_assign(&mut self, rhs: Self) {
        self.0 += rhs.0;
        self.1 += rhs.1;
        self.2 += rhs.2;
    }
}

// v1 -= v2
impl SubAssign for Vec3 {
    fn sub_assign(&mut self, rhs: Self) {
        self.0 -= rhs.0;
        self.1 -= rhs.1;
        self.2 -= rhs.2;
    }
}

// v1 *= v2
impl MulAssign for Vec3 {
    fn mul_assign(&mut self, rhs: Self) {
        self.0 *= rhs.0;
        self.1 *= rhs.1;
        self.2 *= rhs.2;
    }
}

// assign with other type

// v *= f
impl MulAssign<f64> for Vec3 {
    fn mul_assign(&mut self, rhs: f64) {
        self.0 *= rhs;
        self.1 *= rhs;
        self.2 *= rhs;
    }
}

// v *= i
impl MulAssign<i32> for Vec3 {
    fn mul_assign(&mut self, rhs: i32) {
        *self *= rhs as f64;
    }
}

// v /= f
impl DivAssign<f64> for Vec3 {
    fn div_assign(&mut self, rhs: f64) {
        self.0 /= rhs;
        self.1 /= rhs;
        self.2 /= rhs;
    }
}

// v /= i
impl DivAssign<i32> for Vec3 {
    fn div_assign(&mut self, rhs: i32) {
        *self /= rhs as f64
    }
}

// inverse element

// -v1 = v2
impl Neg for Vec3 {
    type Output = Self;

    fn neg(self) -> Self::Output {
        Self(-self.0, -self.1, -self.2)
    }
}

impl Vec3 {
    pub fn unit(self) -> Self {
        self / self.length()
    }

    pub fn length(self) -> f64 {
        Self::dot(self, self).sqrt()
    }

    pub fn dot(u: Self, v: Self) -> f64 {
        u.0 * v.0 + u.1 * v.1 + u.2 * v.2
    }

    pub fn cross(u: Self, v: Self) -> Self {
        Self(
            u.1 * v.2 - u.2 * v.1,
            u.2 * v.0 - u.0 * v.2,
            u.0 * v.1 - u.1 * v.0,
        )
    }

    pub fn mixed(i: Self, j: Self, k: Self) -> f64 {
        Self::dot(Self::cross(i, j), k)
    }

    pub fn cos_included_angle(u: Self, v: Self) -> f64 {
        Self::dot(u, v) / (u.length() * v.length())
    }

    pub fn veer(self, dir: Self) -> Self {
        self.length() * dir.unit()
    }

    pub fn projection(self, dir: Self) -> Self {
        Self::dot(self, dir) / Self::dot(dir, dir) * dir
    }

    pub fn reflect(self, n: Self) -> Self {
        self - 2 * self.projection(n)
    }
}
