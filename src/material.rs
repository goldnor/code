use crate::{hittable::HitRecord, prelude::*};

pub trait Material {
    fn scatter(&self, _r_in: Ray, _rec: HitRecord) -> Option<(Ray, Color)> {
        None
    }
}
