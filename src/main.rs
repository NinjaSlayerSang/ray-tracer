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
use rand::{thread_rng, Rng};
use render::PPMRender;
use sampler::{GridSampler, RandomSampler};
use scene::Sky;
use utils::LinearGradientColor;
use vec3::Vec3;

fn demo_random_world() -> HittableList {
    let mut world = HittableList::default();

    world.add(Rc::new(Sphere::new(
        Point3::new(0, -1000, 0),
        1000.0,
        Rc::new(Lambertian::new(Color::new(0.5, 0.5, 0.5))),
    )));

    let anchor = Point3::new(4, 0.2, 0);
    for a in -11..11 {
        for b in -11..11 {
            let center = Point3::new(
                a as f64 + thread_rng().gen_range(0.0..=0.9),
                0.2,
                b as f64 + thread_rng().gen_range(0.0..=0.9),
            );
            if (center - anchor).length() > 0.9 {
                let choose_mat = thread_rng().gen::<f64>();
                if choose_mat < 0.8 {
                    // diffuse
                    world.add(Rc::new(Sphere::new(
                        center,
                        0.2,
                        Rc::new(Lambertian::new(Color::random() * Color::random())),
                    )));
                } else if choose_mat < 0.95 {
                    // metal
                    world.add(Rc::new(Sphere::new(
                        center,
                        0.2,
                        Rc::new(Metal::new(
                            Color::random_range(0.5..=1.0),
                            thread_rng().gen_range(0.0..=0.5),
                        )),
                    )));
                } else {
                    // glass
                    world.add(Rc::new(Sphere::new(
                        center,
                        0.2,
                        Rc::new(Dielectric::new(1.5)),
                    )));
                }
            }
        }
    }

    world.add(Rc::new(Sphere::new(
        Point3::new(0, 1, 0),
        1.0,
        Rc::new(Dielectric::new(1.5)),
    )));

    world.add(Rc::new(Sphere::new(
        Point3::new(-4, 1, 0),
        1.0,
        Rc::new(Lambertian::new(Color::new(0.4, 0.2, 0.1))),
    )));

    world.add(Rc::new(Sphere::new(
        Point3::new(4, 1, 0),
        1.0,
        Rc::new(Metal::new(Color::new(0.7, 0.6, 0.5), 0.0)),
    )));

    world
}

fn main() {
    // Const

    let image_size = (1800, 1200);
    let aspect_ratio = (image_size.0 as f64) / (image_size.1 as f64);
    #[allow(unused_variables)]
    let random_sampler = RandomSampler(50);
    #[allow(unused_variables)]
    let grid_sampler = GridSampler(7);

    // World & Scene

    let mut world = demo_random_world();

    let sun_position = Vec3::new(-300, 1500, 300);

    world.add(Rc::new(Sphere::new(
        sun_position,
        300.0,
        Rc::new(LightSource::new(Color::new(0.9, 0.9, 0.9))),
    )));

    let scene = Sky::new(
        sun_position,
        LinearGradientColor::new(Color::new(0.0, 0.1, 0.2), Color::new(0.5, 0.7, 1.0)),
    );

    // Camera

    let look_from = Point3::new(13, 2, 3);
    let look_at = Point3::new(0, 0, 0);
    let vup = Vec3::new(0, 1, 0);
    let vfov = 20.0;
    let aperture = 0.1;
    let focus_dist = 10.0;

    let camera = Camera::new(
        look_from,
        look_at,
        vup,
        vfov,
        aspect_ratio,
        aperture,
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
            &grid_sampler,
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
