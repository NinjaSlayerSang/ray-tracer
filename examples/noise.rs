use ray_tracer::{
    camera::Camera,
    color::Color,
    hittable::{HittableList, Sphere},
    material::Lambertian,
    point3::Point3,
    render::PPMRender,
    sampler::RandomSampler,
    scene::Sky,
    texture::Noise,
    utils::{LinearGradientColor, Perlin},
    vec3::Vec3,
};
use std::{
    env::args,
    fs::File,
    io::{stdout, Write},
    sync::Arc,
};

fn main() {
    let image_size = (640, 480);
    let (image_width, image_height) = image_size;
    let aspect_ratio = (image_width as f64) / (image_height as f64);
    let sampler = RandomSampler(100);
    let time_range = (0.0, 1.0);

    let mut objects = HittableList::default();

    let perlin = Arc::new(Perlin::new(0.3));
    let pertext = Arc::new(Noise::new(perlin));
    objects.add(Arc::new(Sphere::new(
        Point3::new(0, -1000, 0),
        1000.0,
        Arc::new(Lambertian::new(pertext.clone())),
    )));
    objects.add(Arc::new(Sphere::new(
        Point3::new(0, 2, 0),
        2.0,
        Arc::new(Lambertian::new(pertext)),
    )));

    let sun_position = Vec3::new(-300, 1500, 300);
    let scene = Sky::new(
        sun_position,
        LinearGradientColor::new(Color::new(0.0, 0.1, 0.2), Color::new(0.5, 0.7, 1.0)),
    );

    let look_from = Point3::new(13, 2, 3);
    let look_at = Point3::new(0, 0, 0);
    let vup = Vec3::new(0, 1, 0);
    let vfov = 20.0;
    let aperture = 0.01;
    let focus_dist = 10.0;

    let camera = Camera::new(
        (look_from, look_at),
        vup,
        vfov,
        aspect_ratio,
        aperture,
        focus_dist,
        time_range,
    );

    let out_file_path = &args().collect::<Vec<_>>()[1];
    let out_file = File::create(out_file_path).unwrap();

    PPMRender::default()
        .draw(
            out_file,
            image_size,
            sampler,
            Arc::new(camera),
            Arc::new(objects),
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
