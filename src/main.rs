use code::{
    camera::Camera,
    hittable_list::HittableList,
    material::{Dielectric, Lambertian, Metal},
    prelude::*,
    sphere::Sphere,
};

fn main() -> std::io::Result<()> {
    let mut world = HittableList::new();

    let material_ground = Rc::new(Lambertian::new(Color::new(0.8, 0.8, 0.0)));
    let material_center = Rc::new(Lambertian::new(Color::new(0.1, 0.2, 0.5)));
    let material_left = Rc::new(Dielectric::new(1.5));
    let material_right = Rc::new(Metal::new(Color::new(0.8, 0.6, 0.2), 1.0));

    world.add(Rc::new(Sphere::new(
        Point3::new(0.0, -100.5, -1.0),
        100.0,
        material_ground,
    )));
    world.add(Rc::new(Sphere::new(
        Point3::new(0.0, 0.0, -1.2),
        0.5,
        material_center,
    )));
    world.add(Rc::new(Sphere::new(
        Point3::new(-1.0, 0.0, -1.0),
        0.5,
        material_left,
    )));
    world.add(Rc::new(Sphere::new(
        Point3::new(1.0, 0.0, -1.0),
        0.5,
        material_right,
    )));

    env_logger::init();

    Camera::default()
        .with_aspect_ratio(16.0 / 9.0)
        .with_image_width(400)
        .with_samples_per_pixel(100)
        .with_max_depth(50)
        .render(&world)
}
