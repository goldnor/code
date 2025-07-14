use code::{camera::Camera, hittable_list::HittableList, prelude::*, sphere::Sphere};

fn main() -> std::io::Result<()> {
    let mut world = HittableList::new();

    world.add(Rc::new(Sphere::new(Point3::new(0.0, 0.0, -1.0), 0.5)));
    world.add(Rc::new(Sphere::new(Point3::new(0.0, -100.5, -1.0), 100.0)));

    env_logger::init();

    Camera::default()
        .with_aspect_ratio(16.0 / 9.0)
        .with_image_width(400)
        .with_samples_per_pixel(100)
        .render(&world)
}
