#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Context {
    None,
}

impl Default for Context {
    fn default() -> Self {
        Self::None
    }
}
