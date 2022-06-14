use rand::{thread_rng, Rng};
use ray_tracer::{
    camera::Camera,
    color::Color,
    hittable::{BVHNode, HittableList, MovingSphere, Sphere},
    material::{Dielectric, DiffuseLight, Lambertian, Metal},
    point3::Point3,
    render::PPMRender,
    sampler::{GridSampler, RandomSampler},
    scene::Sky,
    texture::{Checker, SolidColor},
    utils::LinearGradientColor,
    vec3::Vec3,
};
use std::{
    env::args,
    fs::File,
    io::{stdout, Write},
    sync::Arc,
};

fn main() {
    // Const

    let image_size = (1920, 1080);
    let (image_width, image_height) = image_size;
    let aspect_ratio = (image_width as f64) / (image_height as f64);
    let samplers = (GridSampler(10), RandomSampler(100));
    let time_range = (0.0, 1.0);

    // World & Scene

    let mut hittable_list = HittableList::default();

    // ground
    let ground_texture = Arc::new(SolidColor::new(Color::new(0.5, 0.5, 0.5)));
    let ground_material = Arc::new(Lambertian::new(ground_texture));
    hittable_list.add(Arc::new(Sphere::new(
        Point3::new(0, -1000, 0),
        1000.0,
        ground_material,
    )));

    let glass = Arc::new(Dielectric::new(1.5));

    // small ball
    let anchor = Point3::new(4, 0.2, 0);
    for a in -11..11 {
        for b in -11..11 {
            let center = Point3::new(
                a as f64 + thread_rng().gen_range(0.0..=0.9),
                0.2,
                b as f64 + thread_rng().gen_range(0.0..=0.9),
            );
            let random_color_texture = Arc::new(SolidColor::new(Color::random() * Color::random()));
            let random_color_lambertian = Arc::new(Lambertian::new(random_color_texture));
            if (center - anchor).length() > 0.9 {
                let choose_mat = thread_rng().gen::<f64>();
                if choose_mat < 0.05 {
                    // moving
                    hittable_list.add(Arc::new(MovingSphere::new(
                        (
                            center,
                            center + Vec3::new(0, thread_rng().gen_range(0.0..=0.5), 0),
                        ),
                        time_range,
                        0.2,
                        random_color_lambertian,
                    )))
                } else if choose_mat < 0.4 {
                    // diffuse
                    hittable_list.add(Arc::new(Sphere::new(center, 0.2, random_color_lambertian)));
                } else if choose_mat < 0.85 {
                    // metal
                    let random_color_metal = Arc::new(Metal::new(
                        Color::random_range(0.5..=1.0),
                        thread_rng().gen_range(0.0..=0.5),
                    ));
                    hittable_list.add(Arc::new(Sphere::new(center, 0.2, random_color_metal)));
                } else {
                    // glass
                    hittable_list.add(Arc::new(Sphere::new(center, 0.2, glass.clone())));
                }
            }
        }
    }

    // glass ball
    hittable_list.add(Arc::new(Sphere::new(Point3::new(0, 1, 0), 1.0, glass)));

    // checker ball
    let white_solid_color = Arc::new(SolidColor::new(Color::white()));
    let black_solid_color = Arc::new(SolidColor::new(Color::default()));
    let checker_texture = Arc::new(Checker::new(
        white_solid_color,
        black_solid_color,
        (30.0, 15.0),
    ));
    let checker_lambertian = Arc::new(Lambertian::new(checker_texture));
    hittable_list.add(Arc::new(
        Sphere::new(Point3::new(-4, 1, 0), 1.0, checker_lambertian)
            .set_axis((Vec3::new(1, 3, 1), Vec3::new(1, 0, 0))),
    ));

    // glossy ball
    let mirrorlike_metal = Arc::new(Metal::new(Color::new(0.7, 0.6, 0.5), 0.0));
    hittable_list.add(Arc::new(Sphere::new(
        Point3::new(4, 1, 0),
        1.0,
        mirrorlike_metal,
    )));

    let sun_position = Vec3::new(-300, 1500, 300);

    // sun
    let lightsource = Arc::new(DiffuseLight::new(Arc::new(SolidColor::new(Color::new(
        0.9, 0.9, 0.9,
    )))));
    hittable_list.add(Arc::new(Sphere::new(sun_position, 300.0, lightsource)));

    let bvh_node = BVHNode::from_list(&hittable_list.objects, time_range);

    let scene = Sky::new(
        sun_position,
        LinearGradientColor::new(Color::new(0.0, 0.1, 0.2), Color::new(0.5, 0.7, 1.0)),
    );

    // Camera

    let look_from = Point3::new(13, 2, 3);
    let look_at = Point3::new(0, 0, 0);
    let vup = Vec3::new(0, 1, 0);
    let vfov = 25.0;
    let aperture = 0.05;
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

    // Render

    let out_file_path = &args().collect::<Vec<_>>()[1];
    let out_file = File::create(out_file_path).unwrap();

    PPMRender::default()
        .draw(
            out_file,
            image_size,
            samplers.0,
            Arc::new(camera),
            Arc::new(bvh_node),
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
