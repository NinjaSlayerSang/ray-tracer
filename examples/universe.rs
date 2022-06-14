use ray_tracer::{
    camera::Camera,
    color::Color,
    hittable::{HittableList, Sphere},
    material::{DiffuseLight, Lambertian},
    point3::Point3,
    render::PPMRender,
    sampler::RandomSampler,
    scene::Space,
    texture::{ImageMap, SolidColor},
    utils::random_xy_in_circle,
    vec3::Vec3,
};
use std::{
    env::args,
    fs::File,
    io::{stdout, Write},
    sync::Arc,
};

fn main() {
    let image_size = (1280, 720);
    let (image_width, image_height) = image_size;
    let aspect_ratio = (image_width as f64) / (image_height as f64);
    let sampler = RandomSampler(100);
    let time_range = (0.0, 1.0);

    let mut objects = HittableList::default();

    let sun_position = Vec3::new(-400, 400, 1000);
    objects.add(Arc::new(Sphere::new(
        sun_position,
        300.0,
        Arc::new(DiffuseLight::new(Arc::new(SolidColor::new(Color::white())))),
    )));
    let image_map = Arc::new(ImageMap::try_new("./asset/earthmap.jpg").unwrap());
    objects.add(Arc::new(
        Sphere::new(
            Point3::new(0, 1, 0),
            2.0,
            Arc::new(Lambertian::new(image_map)),
        )
        .set_axis({
            let (x, y) = random_xy_in_circle(1.0);
            (Vec3::new(1, 3, 1), Vec3::new(x, y, 0))
        }),
    ));

    let look_from = Point3::new(0, 6, 16);
    let look_at = Point3::new(0, 1, 0);
    let vup = Vec3::new(0, 1, 0);
    let vfov = 20.0;
    let aperture = 0.001;
    let focus_dist = 20.0;

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
            Arc::new(Space::new(0.001 * Color::white())),
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
