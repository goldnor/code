use crate::prelude::*;

pub type Color = Vec3;

#[inline]
pub fn linear_to_gamma(linear_component: f64) -> f64 {
    if linear_component > 0.0 {
        f64::sqrt(linear_component)
    } else {
        0.0
    }
}

pub fn write_color(mut out: impl std::io::Write, pixel_color: Color) -> std::io::Result<()> {
    let r = pixel_color.x();
    let g = pixel_color.y();
    let b = pixel_color.z();

    // Apply a linear to gamma transform for gamma 2
    let r = linear_to_gamma(r);
    let g = linear_to_gamma(g);
    let b = linear_to_gamma(b);

    // Translate the [0,1] component values to the byte range [0,255].
    const INTENSITY: Interval = Interval::new(0.000, 0.999);
    let rbyte = (256.0 * INTENSITY.clamp(r)) as i32;
    let gbyte = (256.0 * INTENSITY.clamp(g)) as i32;
    let bbyte = (256.0 * INTENSITY.clamp(b)) as i32;

    // Write out the pixel color components.
    writeln!(out, "{rbyte} {gbyte} {bbyte}")
}
