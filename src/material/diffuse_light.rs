use std::sync::Arc;

use crate::{color::Color, texture::Texture};

use super::{Context, Material};

#[derive(Clone)]
pub struct DiffuseLight {
    emit: Arc<dyn Texture + Send + Sync>,
}

impl DiffuseLight {
    pub fn new(emit: Arc<dyn Texture + Send + Sync>) -> Self {
        Self { emit }
    }
}

impl Material for DiffuseLight {
    fn emitted(&self, ctx: Context) -> Color {
        self.emit.value(ctx)
    }
}
