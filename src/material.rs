use crate::{hittable::HitRecord, prelude::*};

pub trait Material {
    fn scatter(&self, _r_in: Ray, _rec: HitRecord) -> Option<(Ray, Color)> {
        None
    }
}

#[derive(Debug, Default, Clone, Copy)]
pub struct Lambertian {
    albedo: Color,
}

impl Lambertian {
    pub fn new(albedo: Color) -> Self {
        Self { albedo }
    }
}

impl Material for Lambertian {
    fn scatter(&self, _r_in: Ray, rec: HitRecord) -> Option<(Ray, Color)> {
        let mut scatter_direction = rec.normal + random_unit_vector();

        // Catch degenerate scatter direction
        if scatter_direction.near_zero() {
            scatter_direction = rec.normal;
        }

        let scattered = Ray::new(rec.p, scatter_direction);
        let attenuation = self.albedo;

        Some((scattered, attenuation))
    }
}
