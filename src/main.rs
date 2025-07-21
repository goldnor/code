use code::{
    camera::Camera,
    hittable_list::HittableList,
    material::{Dielectric, Lambertian, Material, Metal},
    prelude::*,
    sphere::Sphere,
};

fn main() -> std::io::Result<()> {
    let mut world = HittableList::new();

    let ground_material = Rc::new(Lambertian::new(Color::new(0.5, 0.5, 0.5)));
    world.add(Rc::new(Sphere::new(
        Point3::new(0.0, -1000.0, 0.0),
        1000.0,
        ground_material,
    )));

    for a in -11..11 {
        for b in -11..11 {
            let choose_mat: f64 = rand::random();
            let center = Point3::new(
                a as f64 + 0.9 * rand::random::<f64>(),
                0.2,
                b as f64 + 0.9 * rand::random::<f64>(),
            );

            if (center - Point3::new(4.0, 0.2, 0.0)).length() > 0.9 {
                let sphere_material: Rc<dyn Material> = if choose_mat < 0.8 {
                    // diffuse
                    let albedo = Color::random() * Color::random();

                    Rc::new(Lambertian::new(albedo))
                } else if choose_mat < 0.95 {
                    // metal
                    let albedo = Color::random_range(0.5, 1.0);
                    let fuzz = rand::random_range(0.0..0.5);

                    Rc::new(Metal::new(albedo, fuzz))
                } else {
                    // glass

                    Rc::new(Dielectric::new(1.5))
                };

                world.add(Rc::new(Sphere::new(center, 0.2, sphere_material)));
            }
        }
    }

    let material1 = Rc::new(Dielectric::new(1.5));
    world.add(Rc::new(Sphere::new(
        Point3::new(0.0, 1.0, 0.0),
        1.0,
        material1,
    )));

    let material2 = Rc::new(Lambertian::new(Color::new(0.4, 0.2, 0.1)));
    world.add(Rc::new(Sphere::new(
        Point3::new(-4.0, 1.0, 0.0),
        1.0,
        material2,
    )));

    let material3 = Rc::new(Metal::new(Color::new(0.7, 0.6, 0.5), 0.0));
    world.add(Rc::new(Sphere::new(
        Point3::new(4.0, 1.0, 0.0),
        1.0,
        material3,
    )));

    env_logger::init();

    Camera::default()
        .with_aspect_ratio(16.0 / 9.0)
        .with_image_width(1200)
        .with_samples_per_pixel(500)
        .with_max_depth(50)
        .with_vfov(20.0)
        .with_lookfrom(Point3::new(13.0, 2.0, 3.0))
        .with_lookat(Point3::new(0.0, 0.0, 0.0))
        .with_vup(Point3::new(0.0, 1.0, 0.0))
        .with_defocus_angle(0.6)
        .with_focus_dist(10.0)
        .render(&world)
}
