mod camera;
mod color;
mod hittable;
mod point3;
mod ray;
mod utils;
mod vec3;

use rand::{thread_rng, Rng};
use std::io::{stderr, stdout, Write};

use camera::Camera;
use color::{
    primary_color::{BLACK, WHITE},
    Color,
};
use hittable::{HitRecord, Hittable, HittableList, Sphere};
use point3::Point3;
use ray::Ray;
use utils::random_unit_vec3;

const T_MIN: f64 = 0.001;
const REFLECTIVITY: f64 = 0.5;

fn ray_color(ray: &Ray, world: &HittableList, depth: i32) -> Color {
    if depth > 0 {
        let mut rec = HitRecord::default();

        if world.hit(ray, T_MIN, f64::MAX, &mut rec) {
            let target = rec.point + rec.normal + random_unit_vec3();
            REFLECTIVITY
                * ray_color(
                    &Ray::new(rec.point, Point3::vector(rec.point, target)),
                    world,
                    depth - 1,
                )
        } else {
            let unit_direction = ray.direction.unit();
            let t = 0.5 * (unit_direction.y + 1.0);
            (1.0 - t) * Color::from(WHITE) + t * Color::from((0.5, 0.7, 1.0))
        }
    } else {
        Color::from(BLACK)
    }
}

fn main() {
    // Image

    let (image_width, image_height) = (640, 360);
    let aspect_ratio = (image_width as f64) / (image_height as f64);
    let samples_per_pixel = 32;
    let max_depth = 8;

    // World

    let mut world = HittableList::default();

    world.add(Sphere::new(Point3::new(0.0, 0.0, -1.0), 0.5));
    world.add(Sphere::new(Point3::new(0.0, -100.5, -1.0), 100.0));

    // Camera

    let viewport_height = 2.0;
    let viewport_width = aspect_ratio * viewport_height;
    let focal_length = 1.0;

    let camera = Camera::new_regular(viewport_width, viewport_height, focal_length);

    // Render

    let mut std_out = stdout();
    let mut std_err = stderr();

    writeln!(std_out, "P3\n{} {}\n255", image_width, image_height).unwrap();

    for j in (0..image_height).rev() {
        write!(std_err, "\rScanlines remaining: {} ", j).unwrap();
        std_err.flush().unwrap();

        for i in 0..image_width {
            let mut pixel_color = Color::default();

            for _ in 0..=samples_per_pixel {
                let u = (i as f64 + thread_rng().gen::<f64>()) / (image_width - 1) as f64;
                let v = (j as f64 + thread_rng().gen::<f64>()) / (image_height - 1) as f64;
                let ray = camera.get_ray(u, v);
                pixel_color += ray_color(&ray, &world, max_depth);
            }

            writeln!(
                std_out,
                "{}\n",
                (pixel_color / samples_per_pixel as f64)
                    .gamma_correction(2.2)
                    .into_rgb_str()
            )
            .unwrap();
        }
    }

    write!(std_err, "\nDone.").unwrap();

    // Clear

    world.clear();
}
