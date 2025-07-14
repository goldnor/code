use crate::prelude::*;

pub type Color = Vec3;

pub fn write_color(mut out: impl std::io::Write, pixel_color: Color) -> std::io::Result<()> {
    let r = pixel_color.x();
    let g = pixel_color.y();
    let b = pixel_color.z();

    // Translate the [0,1] component values to the byte range [0,255].
    const INTENSITY: Interval = Interval::new(0.000, 0.999);
    let rbyte = (256.0 * INTENSITY.clamp(r)) as i32;
    let gbyte = (256.0 * INTENSITY.clamp(g)) as i32;
    let bbyte = (256.0 * INTENSITY.clamp(b)) as i32;

    // Write out the pixel color components.
    writeln!(out, "{rbyte} {gbyte} {bbyte}")
}
