use std::{
    io::{Result, Write},
    sync::Arc,
    thread::spawn,
};

use crate::{camera::Camera, color::Color, hittable::Hittable, scene::Scene};

use super::render;

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
            depth: 100,
            gamma: 2.2,
        }
    }
}

impl PPMRender {
    #![allow(dead_code)]

    pub fn set_t_range(mut self, t_range: (f64, f64)) -> Self {
        self.t_range = t_range;
        self
    }

    pub fn set_dissipation(mut self, dissipation: Color) -> Self {
        self.dissipation = dissipation;
        self
    }

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
        sampler: impl IntoIterator<Item = (f64, f64)> + Copy + Send + 'static,
        camera: Arc<Camera>,
        hittable: Arc<dyn Hittable + Send + Sync>,
        scene: Arc<dyn Scene + Send + Sync>,
        progress: &mut impl FnMut(f64),
    ) -> Result<()> {
        let (image_width, image_height) = image_size;
        let d = image_height as f64;
        writeln!(out, "P3\n{} {}\n255", image_width, image_height)?;

        for j in (0..image_height).rev() {
            progress(1f64 - j as f64 / d);
            for i in 0..image_width {
                if false {
                    let t_range = self.t_range;
                    let dissipation = self.dissipation;
                    let depth = self.depth;
                    let gamma = self.gamma;
                    let shared_camera = camera.clone();
                    let shared_hittable = hittable.clone();
                    let shared_scene = scene.clone();
                    spawn(move || {
                        render(
                            (i, j),
                            image_size,
                            sampler,
                            shared_camera,
                            shared_hittable,
                            shared_scene,
                            t_range,
                            dissipation,
                            depth,
                            gamma,
                        )
                    });
                }

                let color = render(
                    (i, j),
                    image_size,
                    sampler,
                    camera.clone(),
                    hittable.clone(),
                    scene.clone(),
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
