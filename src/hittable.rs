use crate::{
    ray::Ray,
    vec3::{Point3, Vec3, dot},
};

#[derive(Debug, Default, Clone, Copy)]
pub struct HitRecord {
    pub p: Point3,
    pub normal: Vec3,
    pub t: f64,
    pub front_face: bool,
}

impl HitRecord {
    pub fn set_face_normal(&mut self, r: Ray, outward_normal: Vec3) {
        // Sets the hit record normal vector.
        // NOTE: the parameter `outward_normal` is assumed to have unit length.

        let normal;
        let front_face;
        if dot(r.direction(), outward_normal) > 0.0 {
            // ray is inside the sphere
            normal = -outward_normal;
            front_face = false;
        } else {
            // ray is outside the sphere
            normal = outward_normal;
            front_face = true;
        }

        self.front_face = front_face;
        self.normal = normal;
    }
}

pub trait Hittable {
    fn hit(&self, r: Ray, ray_tmin: f64, ray_tmax: f64) -> Option<HitRecord>;
}
