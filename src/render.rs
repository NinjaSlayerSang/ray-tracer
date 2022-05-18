mod ppm_render;

use crate::{
    camera::Camera,
    color::Color,
    hittable::{HitRecord, Hittable},
    ray::Ray,
    scene::Scene,
};

pub trait Render {
    fn ray_color(
        &self,
        ray: &Ray,
        hittable: &impl Hittable,
        scene: &impl Scene,
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
                        * self.ray_color(
                            &scattered,
                            hittable,
                            scene,
                            t_range,
                            dissipation,
                            depth - 1,
                        )
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

    fn render(
        &self,
        pixel_coord: (i32, i32),
        image_size: (i32, i32),
        sampler: &(impl IntoIterator<Item = (f64, f64)> + Copy),
        camera: &Camera,
        hittable: &impl Hittable,
        scene: &impl Scene,
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

        for (u, v) in *sampler {
            let s = (i + u) / width;
            let t = (j + v) / height;
            let ray = camera.get_ray(s, t);

            pixel_color += self.ray_color(&ray, hittable, scene, t_range, dissipation, depth);
            count += 1;
        }

        (pixel_color / count).gamma_correction(gamma)
    }
}
