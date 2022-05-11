#![allow(unused)]

use crate::point3::Point3;
use crate::vec3::Vec3;

#[derive(Clone, Debug)]
pub struct Camera {
    origin: Point3,
    lower_left_corner: Point3,
    horizontal: Vec3,
    vertical: Vec3,
}

impl Camera {
    pub fn regular(viewport_size: (f64, f64), focal_length: f64) -> Self {
        let (width, height) = viewport_size;

        let origin = Point3::default();
        let horizontal = Vec3::from((width, 0f64, 0f64));
        let vertical = Vec3::from((0f64, height, 0f64));
        let lower_left_corner =
            origin - horizontal / 2f64 - vertical / 2f64 - Vec3::from((0f64, 0f64, focal_length));
        Self {
            origin,
            lower_left_corner,
            horizontal,
            vertical,
        }
    }
}
