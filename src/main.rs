use code::{
    camera::Camera,
    hittable_list::HittableList,
    material::{Dielectric, Lambertian, Metal},
    prelude::*,
    sphere::Sphere,
};

fn main() -> std::io::Result<()> {
    let mut world = HittableList::new();

    let r = f64::cos(PI / 4.0);

    let material_left = Rc::new(Lambertian::new(Color::new(0.0, 0.0, 1.0)));
    let material_right = Rc::new(Lambertian::new(Color::new(1.0, 0.0, 0.0)));

    world.add(Rc::new(Sphere::new(
        Point3::new(-r, 0.0, -1.0),
        r,
        material_left,
    )));
    world.add(Rc::new(Sphere::new(
        Point3::new(r, 0.0, -1.0),
        r,
        material_right,
    )));

    env_logger::init();

    Camera::default()
        .with_aspect_ratio(16.0 / 9.0)
        .with_image_width(400)
        .with_samples_per_pixel(100)
        .with_max_depth(50)
        .with_vfov(90.0)
        .render(&world)
}
