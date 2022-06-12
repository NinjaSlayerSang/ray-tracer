use crate::point3::Point3;

#[derive(Clone, Copy)]
pub enum Context {
    UVP { u: f64, v: f64, p: Point3 },
    None,
}

impl Default for Context {
    fn default() -> Self {
        Self::None
    }
}
