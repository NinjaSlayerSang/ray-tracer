use std::io::Write;

mod vec3;
use vec3::Vec3;

fn test() {
    let v1 = Vec3::from((1.0, 1.0, 1.0));
    let v2 = Vec3 {
        x: 1.0,
        y: 2.0,
        z: 3.0,
    };
    let v3 = Vec3::from([-1.5, 0.5, 2.0]);

    println!("{} + {} = {}", v1, v2, v1 + v2);
    println!("{} - {} = {}", v1, v2, v1 - v2);
    println!("-{} = {}", v2, -v2);
    println!("{} * {} = {}", v2, 2.0, v2 * 2.0);
    println!("{} * {} = {}", 2.0, v2, 2.0 * v2);
    println!("{} * {} = {}", v1, v2, v1 * v2);
    println!("{} / {} = {}", v2, 2.0, v2 / 2.0);
    println!("len{} = {}", v2, v2.length());
    println!("{} × {} = {}", v1, v2, Vec3::cross(v1, v2));
    println!("{} × {} = {}", v2, v1, Vec3::cross(v2, v1));
    println!("unit{} = {}", v2, v2.unit());
    println!("[{} {} {}] = {}", v1, v2, v3, Vec3::mixed(v1, v2, v3));

    std::process::exit(0);
}

fn main() {
    test();

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

            let ir = (255.999 * r) as i64;
            let ig = (255.999 * g) as i64;
            let ib = (255.999 * b) as i64;

            println!("{} {} {}", ir, ig, ib);
        }
    }

    eprintln!("\nDone.");
}
