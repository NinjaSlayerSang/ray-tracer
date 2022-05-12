mod camera;
mod color;
mod hittable;
mod point3;
mod ray;
mod utils;
mod vec3;

use rand::Rng;
use std::io::{stderr, stdout, Write};

use camera::Camera;
use color::{primary_color::WHITE, Color};
use hittable::{HitRecord, Hittable, HittableList, Sphere};
use point3::Point3;
use ray::Ray;
use vec3::Vec3;

fn ray_color(ray: &Ray, world: &HittableList) -> Color {
    let mut hit_record = HitRecord::default();

    if world.hit(ray, 0f64..=f64::MAX, &mut hit_record) {
        0.5 * (hit_record.normal + Vec3::from((1.0, 1.0, 1.0)))
    } else {
        let unit_direction = ray.direction.unit();
        let t = 0.5 * (unit_direction.y + 1.0);
        (1.0 - t) * Color::from(WHITE) + t * Color::from((0.5, 0.7, 1.0))
    }
}

fn main() {
    // Image

    let aspect_ratio = 16.0 / 9.0;
    let image_width = 400;
    let image_height = (image_width as f64 / aspect_ratio) as i32;
    let samples_per_pixel = 100;

    // World

    let mut world = HittableList::default();

    world.add(Sphere::new(Point3::from((0.0, 0.0, -1.0)), 0.5));
    world.add(Sphere::new(Point3::from((0.0, -100.5, -1.0)), 100.0));

    // Camera

    let viewport_height = 2.0;
    let viewport_width = aspect_ratio * viewport_height;
    let focal_length = 1.0;

    let camera = Camera::new_regular(viewport_width, viewport_height, focal_length);

    // Render

    let mut std_out = stdout();
    let mut std_err = stderr();

    let mut rng = rand::thread_rng();
    let unit_f64_range = 0f64..=1f64;
    let mut random = || rng.gen_range(unit_f64_range.clone());

    writeln!(std_out, "P3\n{} {}\n255", image_width, image_height).unwrap();

    for j in (0..image_height).rev() {
        write!(std_err, "\rScanlines remaining: {} ", j).unwrap();
        std_err.flush().unwrap();

        for i in 0..image_width {
            let mut pixel_color = Color::default();

            for _ in 0..=samples_per_pixel {
                let u = (i as f64 + random()) / (image_width - 1) as f64;
                let v = (j as f64 + random()) / (image_height - 1) as f64;
                let ray = camera.get_ray(u, v);
                pixel_color += ray_color(&ray, &world);
            }

            _ = writeln!(
                std_out,
                "{}\n",
                (pixel_color / samples_per_pixel as f64).into_rgb_str()
            );
        }
    }

    write!(std_err, "\nDone.").unwrap();

    // Clear

    world.clear();
}
