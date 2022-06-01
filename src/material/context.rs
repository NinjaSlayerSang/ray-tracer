#[derive(Clone, Copy)]
pub enum Context {
    None,
    UV { u: f64, v: f64 },
}

impl Default for Context {
    fn default() -> Self {
        Self::None
    }
}
