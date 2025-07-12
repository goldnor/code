use crate::{
    hittable::{HitRecord, Hittable},
    prelude::*,
};

#[derive(Default)]
pub struct HittableList {
    pub objects: Vec<Rc<dyn Hittable>>,
}

impl HittableList {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn clear(&mut self) {
        self.objects.clear();
    }

    pub fn add(&mut self, object: Rc<dyn Hittable>) {
        self.objects.push(object);
    }
}

impl Hittable for HittableList {
    fn hit(&self, r: Ray, ray_t: Interval) -> Option<HitRecord> {
        self.objects
            .iter()
            .filter_map(|obj| obj.hit(r, ray_t))
            .min_by(|a, b| a.t.partial_cmp(&b.t).expect("no NaN value"))
    }
}
