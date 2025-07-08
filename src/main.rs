use code::color::{Color, write_color};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Image

    const ASPECT_RATIO: f64 = 16.0 / 9.0;
    const IMAGE_WIDTH: i32 = 400;

    // Calculate the image height, and ensure that it's at least 1.
    const IMAGE_HEIGHT: i32 = {
        let image_height = (IMAGE_WIDTH as f64 / ASPECT_RATIO) as i32;
        if image_height < 1 { 1 } else { image_height }
    };

    // Viewport widths less than one are ok since they are real valued.
    let viewport_height = 2.0;
    let viewport_width = viewport_height * (IMAGE_WIDTH as f64) / (IMAGE_HEIGHT as f64);

    // Render

    env_logger::init();
    println!("P3");
    println!("{IMAGE_WIDTH} {IMAGE_HEIGHT}");
    println!("255");

    for j in 0..IMAGE_HEIGHT {
        log::info!("Scanlines remaining: {}", IMAGE_HEIGHT - j);
        for i in 0..IMAGE_WIDTH {
            let pixel_color = Color::new(
                i as f64 / (IMAGE_WIDTH - 1) as f64,
                j as f64 / (IMAGE_HEIGHT - 1) as f64,
                0.0,
            );
            write_color(std::io::stdout(), pixel_color)?;
        }
    }
    log::info!("Done.");

    Ok(())
}
