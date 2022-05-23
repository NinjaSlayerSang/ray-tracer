mod ppm_render;

use std::sync::Arc;

use crate::{
    camera::Camera,
    color::Color,
    hittable::{HitRecord, Hittable},
    ray::Ray,
    scene::Scene,
};

pub use ppm_render::PPMRender;

fn ray_color(
    ray: &Ray,
    hittable: Arc<dyn Hittable>,
    scene: Arc<dyn Scene>,
    t_range: (f64, f64),
    dissipation: Color,
    depth: i32,
) -> Color {
    if depth > 0 {
        let mut rec = HitRecord::default();
        if hittable.hit(ray, t_range, &mut rec) {
            let mut scattered = Ray::default();
            let mut attenuation = Color::default();
            if rec
                .material
                .scatter(ray, &rec, &mut attenuation, &mut scattered)
            {
                attenuation
                    * ray_color(&scattered, hittable, scene, t_range, dissipation, depth - 1)
            } else {
                attenuation
            }
        } else {
            scene.scene_color(ray)
        }
    } else {
        dissipation
    }
}

pub fn render(
    pixel_coord: (usize, usize),
    image_size: (usize, usize),
    sampler: impl IntoIterator<Item = (f64, f64)> + Copy,
    camera: Arc<Camera>,
    hittable: Arc<dyn Hittable>,
    scene: Arc<dyn Scene>,
    t_range: (f64, f64),
    dissipation: Color,
    depth: i32,
    gamma: f64,
) -> Color {
    let (i, j) = pixel_coord;
    let (width, height) = image_size;
    let (i, j, width, height) = (i as f64, j as f64, width as f64, height as f64);

    let mut pixel_color = Color::default();
    let mut count = 0;

    for (u, v) in sampler {
        let s = (i + u) / width;
        let t = (j + v) / height;
        let ray = camera.get_ray(s, t);

        pixel_color += ray_color(
            &ray,
            hittable.clone(),
            scene.clone(),
            t_range,
            dissipation,
            depth,
        );
        count += 1;
    }

    (pixel_color / count).gamma_correction(gamma)
}
