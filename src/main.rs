use std::io::Write;

mod vec3;
use vec3::Vec3;

fn test() {
    // let v0 = vec3::zero();
    let v1 = vec3::unit();
    let v = Vec3 {
        x: 1.0,
        y: 2.0,
        z: 3.0,
    };

    println!("{} + {} = {}", v, v1, v + v1);
    println!("{} - {} = {}", v, v1, v - v1);
    println!("-{} = {}", v, -v);
    println!("{} * {} = {}", v, 2.0, v * 2.0);
    println!("{} * {} = {}", 2.0, v, 2.0 * v);
    println!("{} * {} * {} = {}", v, 2.0, v1, v * 2.0 * v1);
    println!("{} / {} = {}", v, 2.0, v / 2.0);
    println!("len{} = {}", v, v.length());

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
