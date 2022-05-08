use super::Vec3;

pub type F64Tuple3 = (f64, f64, f64);

impl From<F64Tuple3> for Vec3 {
    fn from(tuple: F64Tuple3) -> Self {
        Self {
            x: tuple.0,
            y: tuple.1,
            z: tuple.2,
        }
    }
}

impl Into<F64Tuple3> for Vec3 {
    fn into(self) -> F64Tuple3 {
        (self.x, self.y, self.z)
    }
}

impl std::fmt::Display for Vec3 {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "({}, {}, {})", self.x, self.y, self.z)
    }
}
