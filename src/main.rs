mod color;
mod point3;
mod ray;
mod vec3;

use color::{Color, WHITE};
use point3::Point3;
use ray::Ray;
use std::io::Write;
use vec3::Vec3;

fn ray_color(r: &Ray) -> Color {
    let u_dir = r.direction().unit();
    let t = 0.5 * (u_dir.y + 1.0);
    (1.0 - t) * Color::from(WHITE) + t * Color::from((0.5, 0.7, 1.0))
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
