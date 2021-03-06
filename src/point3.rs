use crate::vec3::Vec3;

pub type Point3 = Vec3;

impl Point3 {
    pub fn vector(origin: Self, destination: Self) -> Vec3 {
        destination - origin
    }
}
