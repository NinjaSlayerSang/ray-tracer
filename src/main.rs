mod color;
mod hittable;
mod point3;
mod ray;
mod utils;
mod vec3;

use color::{Color, WHITE};
use point3::Point3;
use ray::Ray;
use std::io::Write;
use utils::{solve_quadratic_equation, QuadraticEquationRealRoot};
use vec3::Vec3;

fn hit_sphere(center: Point3, radius: f64, ray: &Ray) -> QuadraticEquationRealRoot {
    let oc = ray.origin - center;
    let d = ray.direction;

    let a = d * d;
    let hb = d * oc;
    let c = oc * oc - radius * radius;

    solve_quadratic_equation(a, hb, c)
}

fn ray_color(ray: &Ray) -> Color {
    let center = Point3::from((0.0, 0.0, -1.0));
    let radius = 0.5;
    match hit_sphere(center, radius, ray) {
        QuadraticEquationRealRoot::Double(t, _) => {
            let n = (ray.at(t) - center).unit();
            0.5 * (n + Vec3::from((1.0, 1.0, 1.0)))
        }
        _ => {
            let unit_direction = ray.direction.unit();
            let t = 0.5 * (unit_direction.y + 1.0);
            (1.0 - t) * Color::from(WHITE) + t * Color::from((0.5, 0.7, 1.0))
        }
    }
}

fn main() {
    // Image

    let aspect_ratio = 16.0 / 9.0;
    let image_width = 400;
    let image_height = (image_width as f64 / aspect_ratio) as i32;

    // Camera

    let viewport_height = 2.0;
    let viewport_width = aspect_ratio * viewport_height;
    let focal_length = 1.0;

    let origin = Point3::from((0.0, 0.0, 0.0));
    let horizontal = Vec3::from((viewport_width, 0.0, 0.0));
    let vertical = Vec3::from((0.0, viewport_height, 0.0));
    let lower_left_corner =
        origin - horizontal / 2.0 - vertical / 2.0 - Vec3::from((0.0, 0.0, focal_length));

    // Render

    println!("P3\n{} {}\n255", image_width, image_height);

    for j in (0..image_height).rev() {
        eprint!("\rScanlines remaining: {} ", j);
        std::io::stderr().flush().unwrap();
        for i in 0..image_width {
            let u = i as f64 / (image_width - 1) as f64;
            let v = j as f64 / (image_height - 1) as f64;
            let r = Ray::new(
                origin,
                lower_left_corner + u * horizontal + v * vertical - origin,
            );
            let pixel_color = ray_color(&r);

            println!("{}", pixel_color.into_rgb_str());
        }
    }

    eprintln!("\nDone.");
}
