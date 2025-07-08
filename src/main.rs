use code::{
    color::{Color, write_color},
    ray::Ray,
    vec3::{Point3, Vec3, unit_vector},
};

fn ray_color(r: Ray) -> Color {
    let unit_direction = unit_vector(r.direction());
    let a = 0.5 * (unit_direction.y() + 1.0);
    (1.0 - a) * Color::new(1.0, 1.0, 1.0) + a * Color::new(0.5, 0.7, 1.0)
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Image

    const ASPECT_RATIO: f64 = 16.0 / 9.0;
    const IMAGE_WIDTH: i32 = 400;

    // Calculate the image height, and ensure that it's at least 1.
    const IMAGE_HEIGHT: i32 = {
        let image_height = (IMAGE_WIDTH as f64 / ASPECT_RATIO) as i32;
        if image_height < 1 { 1 } else { image_height }
    };

    // Camera

    let focal_length = 1.0;
    let viewport_height = 2.0;
    let viewport_width = viewport_height * (IMAGE_WIDTH as f64) / (IMAGE_HEIGHT as f64);
    let camera_center = Point3::new(0.0, 0.0, 0.0);

    // Calculate the vectors across the horizontal and down the vertical viewport edges.
    let viewport_u = Vec3::new(viewport_width, 0.0, 0.0);
    let viewport_v = Vec3::new(0.0, -viewport_height, 0.0);

    // Calculate the horizontal and vertical delta vectors from pixel to pixel.
    let pixel_delta_u = viewport_u / IMAGE_WIDTH as f64;
    let pixel_delta_v = viewport_v / IMAGE_HEIGHT as f64;

    // Calculate the location of the upper left pixel.
    let viewport_upper_left =
        camera_center - Vec3::new(0.0, 0.0, focal_length) - viewport_u / 2.0 - viewport_v / 2.0;
    let pixel00_loc = viewport_upper_left + 0.5 * (pixel_delta_u + pixel_delta_v);

    // Render

    env_logger::init();
    println!("P3");
    println!("{IMAGE_WIDTH} {IMAGE_HEIGHT}");
    println!("255");

    for j in 0..IMAGE_HEIGHT {
        log::info!("Scanlines remaining: {}", IMAGE_HEIGHT - j);
        for i in 0..IMAGE_WIDTH {
            let pixel_center =
                pixel00_loc + (i as f64) * pixel_delta_u + (j as f64) * pixel_delta_v;
            let ray_direction = pixel_center - camera_center;
            let r = Ray::new(camera_center, ray_direction);

            let pixel_color = ray_color(r);
            write_color(std::io::stdout(), pixel_color)?;
        }
    }
    log::info!("Done.");

    Ok(())
}
