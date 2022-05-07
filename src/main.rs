mod color;
mod vec3;

use color::Color;
use std::io::Write;

fn main() {
    let image_width = 256;
    let image_height = 256;

    println!("P3\n{} {}\n255", image_width, image_height);

    for j in (0..image_height).rev() {
        eprint!("\rScanlines remaining: {} ", j);
        std::io::stderr().flush().unwrap();
        for i in 0..image_width {
            let r = i as f64 / (image_width - 1) as f64;
            let g = j as f64 / (image_height - 1) as f64;
            let b: f64 = 0.25;

            let c = Color::from((r, g, b));

            println!("{}", c.into_rgb_str());
        }
    }

    eprintln!("\nDone.");
}
