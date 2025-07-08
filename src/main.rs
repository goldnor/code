use code::color::{Color, write_color};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Image

    const IMAGE_WIDTH: u32 = 256;
    const IMAGE_HEIGHT: u32 = 256;

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
