mod aabb;
mod camera;
mod color;
mod hittable;
mod material;
mod point3;
mod ray;
mod render;
mod sampler;
mod scene;
mod semaphore;
mod utils;
mod vec3;

use std::{
    env::args,
    fs::File,
    io::{stdout, Write},
    sync::Arc,
};

use camera::Camera;
use color::Color;
use hittable::{HittableList, MovingSphere, Sphere};
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

    world.add(Arc::new(Sphere::new(
        Point3::new(0, -1000, 0),
        1000.0,
        Arc::new(Lambertian::new(Color::new(0.5, 0.5, 0.5))),
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
                if choose_mat < 0.15 {
                    // moving
                    world.add(Arc::new(MovingSphere::new(
                        (
                            center,
                            center + Vec3::new(0, thread_rng().gen_range(0.0..=0.5), 0),
                        ),
                        (0.0, 1.0),
                        0.2,
                        Arc::new(Lambertian::new(Color::random() * Color::random())),
                    )))
                } else if choose_mat < 0.4 {
                    // diffuse
                    world.add(Arc::new(Sphere::new(
                        center,
                        0.2,
                        Arc::new(Lambertian::new(Color::random() * Color::random())),
                    )));
                } else if choose_mat < 0.85 {
                    // metal
                    world.add(Arc::new(Sphere::new(
                        center,
                        0.2,
                        Arc::new(Metal::new(
                            Color::random_range(0.5..=1.0),
                            thread_rng().gen_range(0.0..=0.5),
                        )),
                    )));
                } else {
                    // glass
                    world.add(Arc::new(Sphere::new(
                        center,
                        0.2,
                        Arc::new(Dielectric::new(1.5)),
                    )));
                }
            }
        }
    }

    world.add(Arc::new(Sphere::new(
        Point3::new(0, 1, 0),
        1.0,
        Arc::new(Dielectric::new(1.5)),
    )));

    world.add(Arc::new(Sphere::new(
        Point3::new(-4, 1, 0),
        1.0,
        Arc::new(Lambertian::new(Color::new(0.4, 0.2, 0.1))),
    )));

    world.add(Arc::new(Sphere::new(
        Point3::new(4, 1, 0),
        1.0,
        Arc::new(Metal::new(Color::new(0.7, 0.6, 0.5), 0.0)),
    )));

    world
}

fn main() {
    // Const

    let image_size = (1920, 1080); // 1080p
    let (image_width, image_height) = image_size;
    let aspect_ratio = (image_width as f64) / (image_height as f64);
    #[allow(unused_variables)]
    let random_sampler = RandomSampler(100);
    #[allow(unused_variables)]
    let grid_sampler = GridSampler(10);

    // World & Scene

    let mut world = demo_random_world();

    let sun_position = Vec3::new(-300, 1500, 300);

    world.add(Arc::new(Sphere::new(
        sun_position,
        300.0,
        Arc::new(LightSource::new(Color::new(0.9, 0.9, 0.9))),
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
    let time_range = (0.0, 1.0);

    let camera = Camera::new(
        (look_from, look_at),
        vup,
        vfov,
        aspect_ratio,
        aperture,
        focus_dist,
        time_range,
    );

    // Render

    let out_file_path = &args().collect::<Vec<String>>()[1];
    let out_file = File::create(out_file_path).unwrap();

    PPMRender::default()
        .draw(
            out_file,
            image_size,
            grid_sampler,
            Arc::new(camera),
            Arc::new(world),
            Arc::new(scene),
            |progress: Option<f64>| {
                let mut std_out = stdout();

                if let Some(p) = progress {
                    write!(std_out, "\rprogress: {:.2}%", p * 100.0).unwrap();
                    std_out.flush().unwrap();
                } else {
                    writeln!(std_out, "\nDone.").unwrap();
                }
            },
        )
        .unwrap();
}
