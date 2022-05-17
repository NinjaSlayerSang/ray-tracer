#[derive(Clone, Copy, PartialEq)]
pub enum Context {
    None,
}

impl Default for Context {
    fn default() -> Self {
        Self::None
    }
}
