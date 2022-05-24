use std::{
    io::Write,
    sync::{mpsc::channel, Arc, Condvar, Mutex},
    thread::spawn,
    thread::Result,
};

use crate::{camera::Camera, color::Color, hittable::Hittable, scene::Scene};

use super::render;

#[derive(Clone, Copy)]
pub struct PPMRender {
    t_range: (f64, f64),
    dissipation: Color,
    depth: i32,
    gamma: f64,
    concurrent: usize,
}

impl Default for PPMRender {
    fn default() -> Self {
        Self {
            t_range: (0.001, f64::MAX),
            dissipation: Default::default(),
            depth: 100,
            gamma: 2.2,
            concurrent: 4,
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

    pub fn set_concurrent(mut self, concurrent: usize) -> Self {
        self.concurrent = concurrent;
        self
    }
}

impl PPMRender {
    pub fn draw(
        &self,
        mut out: impl Write + Send + 'static,
        image_size: (usize, usize),
        sampler: impl IntoIterator<Item = (f64, f64)> + Copy + Send + 'static,
        camera: Arc<Camera>,
        hittable: Arc<dyn Hittable + Send + Sync>,
        scene: Arc<dyn Scene + Send + Sync>,
        progress: impl Fn(Option<f64>) + Send + 'static,
    ) -> Result<()> {
        let (image_width, image_height) = image_size;
        let capacity = (image_width * image_height) as f64;

        let handle = {
            let (result_sender, result_receiver) = channel::<(usize, Color)>();

            let concurrent = self.concurrent;
            let handle = spawn(move || {
                writeln!(out, "P3\n{} {}\n255", image_width, image_height).unwrap();

                let mut offset = 0usize;
                let mut cache = Vec::<(usize, Color)>::with_capacity(2 * concurrent);

                for result in result_receiver {
                    // enqueue
                    let mut index = cache.len();
                    for (i, item) in cache.iter().enumerate() {
                        if item.0 > result.0 {
                            index = i;
                            break;
                        }
                    }
                    cache.insert(index, result);

                    // outqueue
                    while cache.len() > 0 && cache[0].0 == offset {
                        offset += 1;
                        writeln!(out, "{}", cache.remove(0).1.into_rgb_str()).unwrap();
                    }

                    progress(Some(offset as f64 / capacity));
                }

                progress(None);
            });

            let semaphore = Arc::new((Mutex::new(self.concurrent as i32), Condvar::new()));

            let mut index = 0;
            for j in (0..image_height).rev() {
                for i in 0..image_width {
                    let (mtx, cvar) = &*semaphore;
                    {
                        let mut k = cvar.wait_while(mtx.lock().unwrap(), |k| *k <= 0).unwrap();
                        *k -= 1;
                    }

                    let t_range = self.t_range;
                    let dissipation = self.dissipation;
                    let depth = self.depth;
                    let gamma = self.gamma;
                    let shared_camera = camera.clone();
                    let shared_hittable = hittable.clone();
                    let shared_scene = scene.clone();
                    let shared_result_sender = result_sender.clone();
                    let shared_semaphore = semaphore.clone();
                    spawn(move || {
                        let color = render(
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
                        );

                        shared_result_sender.send((index, color)).unwrap();

                        let (mtx, cvar) = &*shared_semaphore;
                        {
                            let mut k = mtx.lock().unwrap();
                            *k += 1;
                        }
                        cvar.notify_one();
                    });

                    index += 1;
                }
            }

            handle
        };

        handle.join()
    }
}
