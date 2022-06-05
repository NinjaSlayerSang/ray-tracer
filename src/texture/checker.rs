use std::{f64::consts::PI, sync::Arc};

use crate::{color::Color, material::Context};

use super::Texture;

#[derive(Clone)]
pub struct Checker {
    odd: Arc<dyn Texture + Send + Sync>,
    even: Arc<dyn Texture + Send + Sync>,
    scales: (f64, f64),
}

impl Checker {
    pub fn new(
        odd: Arc<dyn Texture + Send + Sync>,
        even: Arc<dyn Texture + Send + Sync>,
        scales: (f64, f64),
    ) -> Self {
        Self { odd, even, scales }
    }
}

impl Texture for Checker {
    fn value(&self, ctx: Context) -> Color {
        let (tu, tv) = self.scales;
        match ctx {
            Context::UV { u, v } => {
                if ((u * tu * PI).sin() * (v * tv * PI).sin()).is_sign_positive() {
                    self.odd.value(ctx)
                } else {
                    self.even.value(ctx)
                }
            }
            _ => Color::default(),
        }
    }
}
