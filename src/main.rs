mod camera;
mod color;
mod hittable;
mod material;
mod point3;
mod ray;
mod render;
mod sampler;
mod scene;
mod utils;
mod vec3;

use rand::{thread_rng, Rng};
use std::{
    io::{stderr, stdout, Write},
    rc::Rc,
};

use camera::Camera;
use color::Color;
use hittable::{HitRecord, Hittable, HittableList, Sphere};
use material::{Dielectric, Lambertian, LightSource, Metal};
use point3::Point3;
use ray::Ray;
use utils::LinearGradientColor;
use vec3::Vec3;

const SUN_POS: Vec3 = Vec3(-30.0, 150.0, -30.0);
const BLUE_GRADIENT: LinearGradientColor =
    LinearGradientColor(Vec3(0.0, 0.1, 0.2), Vec3(0.5, 0.7, 1.0));

fn scene_color(ray: &Ray) -> Color {
    let cos_theta = Vec3::cos_included_angle(ray.direction, SUN_POS);
    let t = 0.5 * (cos_theta + 1.0);
    BLUE_GRADIENT.linear(t)
}

const T_MIN: f64 = 0.001;

fn ray_color(ray: &Ray, world: &HittableList, depth: i32) -> Color {
    if depth > 0 {
        let mut rec = HitRecord::default();

        if world.hit(ray, (T_MIN, f64::MAX), &mut rec) {
            let mut scattered = Ray::default();
            let mut attenuation = Color::default();
            if rec
                .material
                .scatter(ray, &rec, &mut attenuation, &mut scattered)
            {
                attenuation * ray_color(&scattered, world, depth - 1)
            } else {
                attenuation
            }
        } else {
            scene_color(ray)
        }
    } else {
        Color::default()
    }
}

fn main() {
    // Image

    let (image_width, image_height) = (1280, 720);
    let aspect_ratio = (image_width as f64) / (image_height as f64);
    let max_depth = 32;
    let random_sample = false;
    let sample_rank = 4;
    let samples_per_pixel = if random_sample {
        16
    } else {
        sample_rank * sample_rank
    };

    // World

    let mut world = HittableList::default();

    // sun
    world.add(Rc::new(Sphere::new(
        SUN_POS,
        50.0,
        Rc::new(LightSource::default()),
    )));
    // ground
    world.add(Rc::new(Sphere::new(
        Point3::new(0.0, -100.5, -1.0),
        100.0,
        Rc::new(Lambertian::new(Color::new(0.8, 0.8, 0.0))),
    )));
    // center
    world.add(Rc::new(Sphere::new(
        Point3::new(0.0, 0.0, -1.0),
        0.5,
        Rc::new(Lambertian::new(Color::new(0.1, 0.2, 0.5))),
    )));
    // left
    let glass = Rc::new(Dielectric::new(1.5));
    world.add(Rc::new(Sphere::new(
        Point3::new(-1.01, 0.0, -1.0),
        0.5,
        glass.clone(),
    )));
    world.add(Rc::new(Sphere::new(
        Point3::new(-1.0, 0.0, -1.0),
        -0.4,
        glass.clone(),
    )));
    // right
    world.add(Rc::new(Sphere::new(
        Point3::new(1.01, 0.0, -1.0),
        0.5,
        Rc::new(Metal::new(Color::new(0.8, 0.6, 0.2), 0.1)),
    )));

    // Camera

    let look_from = Point3::new(-2, 1, 3);
    let look_at = Point3::new(0, 0, -1);
    let focus_dist = Vec3::vector(look_from, look_at).length();

    let camera = Camera::new(
        look_from,
        look_at,
        Vec3::new(0, 1, 0),
        35.0,
        aspect_ratio,
        0.3,
        focus_dist,
    );

    // Render

    let mut std_out = stdout();
    let mut std_err = stderr();

    writeln!(std_out, "P3\n{} {}\n255", image_width, image_height).unwrap();

    let image_width_denominator = image_width as f64;
    let image_height_denominator = image_height as f64;
    for j in (0..image_height).rev() {
        write!(std_err, "\rScanlines remaining: {} ", j).unwrap();
        std_err.flush().unwrap();

        for i in 0..image_width {
            let mut pixel_color = Color::default();

            if random_sample {
                for _ in 0..samples_per_pixel {
                    let s = (i as f64 + thread_rng().gen::<f64>()) / image_width_denominator;
                    let t = (j as f64 + thread_rng().gen::<f64>()) / image_height_denominator;
                    let ray = camera.get_ray(s, t);
                    pixel_color += ray_color(&ray, &world, max_depth);
                }
            } else {
                let sample_denominator = (sample_rank + 1) as f64;
                for u in 1..=sample_rank {
                    for v in 1..=sample_rank {
                        let s =
                            (i as f64 + u as f64 / sample_denominator) / image_width_denominator;
                        let t =
                            (j as f64 + v as f64 / sample_denominator) / image_height_denominator;
                        let ray = camera.get_ray(s, t);
                        pixel_color += ray_color(&ray, &world, max_depth);
                    }
                }
            }

            writeln!(
                std_out,
                "{}",
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
