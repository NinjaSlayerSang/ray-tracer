use std::io::{Result, Write};

use crate::{camera::Camera, color::Color, hittable::Hittable, scene::Scene};

use super::Render;

#[derive(Clone, Copy)]
pub struct PPMRender {
    t_range: (f64, f64),
    dissipation: Color,
    depth: i32,
    gamma: f64,
}

impl Default for PPMRender {
    fn default() -> Self {
        Self {
            t_range: (0.001, f64::MAX),
            dissipation: Default::default(),
            depth: 64,
            gamma: 2.2,
        }
    }
}

impl PPMRender {
    pub fn set_depth(mut self, depth: i32) -> Self {
        self.depth = depth;
        self
    }

    pub fn set_gamma(mut self, gamma: f64) -> Self {
        self.gamma = gamma;
        self
    }
}

impl PPMRender {
    pub fn draw(
        &self,
        out: &mut impl Write,
        image_size: (i32, i32),
        sampler: &(impl IntoIterator<Item = (f64, f64)> + Copy),
        camera: &Camera,
        hittable: &impl Hittable,
        scene: &impl Scene,
        progress: &mut impl FnMut(f64),
    ) -> Result<()> {
        let (image_width, image_height) = image_size;
        let d = image_height as f64;
        writeln!(out, "P3\n{} {}\n255", image_width, image_height)?;

        for j in (0..image_height).rev() {
            progress(j as f64 / d);
            for i in 0..image_width {
                let color = self.render(
                    (i, j),
                    image_size,
                    sampler,
                    camera,
                    hittable,
                    scene,
                    self.t_range,
                    self.dissipation,
                    self.depth,
                    self.gamma,
                );
                writeln!(out, "{}", color.into_rgb_str())?;
            }
        }

        Ok(())
    }
}

impl Render for PPMRender {}
