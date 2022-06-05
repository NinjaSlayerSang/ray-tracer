#[derive(Clone, Copy)]
pub enum Context {
    UV { u: f64, v: f64 },
    None,
}

impl Default for Context {
    fn default() -> Self {
        Self::None
    }
}
