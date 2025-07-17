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

#[derive(Debug, Default, Clone, Copy)]
pub struct Metal {
    albedo: Color,
}

impl Metal {
    pub fn new(albedo: Color) -> Self {
        Self { albedo }
    }
}

impl Material for Metal {
    fn scatter(&self, r_in: Ray, rec: HitRecord) -> Option<(Ray, Color)> {
        let reflected = reflect(r_in.direction(), rec.normal);
        let scattered = Ray::new(rec.p, reflected);
        let attenuation = self.albedo;

        Some((scattered, attenuation))
    }
}
