use crate::vec3::Vec3;

pub type Color = Vec3;

pub fn write_color(mut out: impl std::io::Write, pixel_color: Color) -> std::io::Result<()> {
    let r = pixel_color.x();
    let g = pixel_color.y();
    let b = pixel_color.z();

    let rbyte = (255.999 * r) as i32;
    let gbyte = (255.999 * g) as i32;
    let bbyte = (255.999 * b) as i32;

    writeln!(out, "{rbyte} {gbyte} {bbyte}")
}
