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

use std::{
    io::{stderr, stdout, Write},
    rc::Rc,
};

use camera::Camera;
use color::Color;
use hittable::{HittableList, Sphere};
use material::{Dielectric, Lambertian, LightSource, Metal};
use point3::Point3;
use render::PPMRender;
use sampler::{GridSampler, RandomSampler};
use scene::Sky;
use utils::LinearGradientColor;
use vec3::Vec3;

fn main() {
    // Const
    let image_size = (480, 320);
    let aspect_ratio = (image_size.0 as f64) / (image_size.1 as f64);
    #[allow(unused_variables)]
    let random_sampler = RandomSampler(32);
    #[allow(unused_variables)]
    let grid_sampler = GridSampler(4);

    // World & Scene

    let sun_position = Vec3::new(-30, 150, 30);

    let mut world = HittableList::default();

    // sun
    world.add(Rc::new(Sphere::new(
        sun_position,
        50.0,
        Rc::new(LightSource::new(Color::new(0.9, 0.9, 0.9))),
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

    let scene = Sky::new(
        sun_position,
        LinearGradientColor::new(Color::new(0.0, 0.1, 0.2), Color::new(0.5, 0.7, 1.0)),
    );

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

    PPMRender::default()
        .set_depth(64)
        .set_gamma(2.2)
        .draw(
            &mut std_out,
            image_size,
            &random_sampler,
            &camera,
            &world,
            &scene,
            &mut |p: f64| {
                write!(std_err, "\rprogress: {:.1}%", p * 100.0).unwrap();
                std_err.flush().unwrap();
            },
        )
        .unwrap();
    writeln!(std_err, "\nDone.").unwrap();

    // Clear

    world.clear();
}
