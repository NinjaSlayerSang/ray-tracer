#![allow(unused)]

use ray_tracer::{vec3, vec3::Vec3};

#[test]
fn vec3_base() {
    let v0 = vec3::zero();
    let v1 = Vec3::new(1.0, 1.0, 1.0);
    let v2 = Vec3 {
        x: 1.0,
        y: 2.0,
        z: 3.0,
    };
    let v3 = Vec3::from((-1.5, 0.0, 0.0));

    println!("{}: {:#?}", v0, v0);

    println!("{} + {} = {}", v1, v2, v1 + v2);

    println!("{} - {} = {}", v1, v2, v1 - v2);
    println!("-{} = {}", v2, -v2);

    println!("{} * {} = {}", v2, 2.0, v2 * 2.0);
    println!("{} * {} = {}", 2.0, v2, 2.0 * v2);

    println!("{} * {} = {}", v1, v2, v1 * v2);
    println!("{} ‧ {} = {}", v2, v3, Vec3::dot(v2, v3));

    println!("{} / {} = {}", v2, 2.0, v2 / 2.0);

    println!("len{} = {}", v2, v2.length());

    println!("{} × {} = {}", v1, v2, Vec3::cross(v1, v2));
    println!("{} × {} = {}", v2, v1, Vec3::cross(v2, v1));

    println!("[{} {} {}] = {}", v1, v2, v3, Vec3::mixed(v1, v2, v3));

    println!("unit{} = {}", v2, v2.unit());

    println!("{} → {} = {}", v1, v2, v1.veer(v2));

    println!("{} ∠ {} = {}", v1, v2, v1.projection(v2));
}
