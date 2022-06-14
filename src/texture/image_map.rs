use image::{open, DynamicImage, GenericImageView, ImageError};

use crate::{color::Color, material::Context};

use super::Texture;

#[derive(Clone)]
pub struct ImageMap {
    data: DynamicImage,
}

impl ImageMap {
    pub fn try_new(url: &str) -> Result<Self, ImageError> {
        let data = open(url)?;

        Ok(Self { data })
    }

    fn cast_uv_to_xy(&self, u: f64, v: f64) -> (u32, u32) {
        let (width, height) = self.data.dimensions();

        let x = (width as f64 * u) as u32;
        let y = (height as f64 * (1f64 - v)) as u32;

        (x, y)
    }

    fn get_color_by_uv(&self, u: f64, v: f64) -> Color {
        let (x, y) = self.cast_uv_to_xy(u, v);

        Color::from_rgba(self.data.get_pixel(x, y).0)
    }
}

impl Texture for ImageMap {
    fn value(&self, ctx: Context) -> Color {
        match ctx {
            Context::UVP { u, v, .. } => self.get_color_by_uv(u, v),
            _ => Color::default(),
        }
    }
}
