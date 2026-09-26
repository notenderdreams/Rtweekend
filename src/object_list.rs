use crate::{
    interval::Interval,
    object::{HRecord, Object},
    ray::Ray,
};

pub struct ObjectList {
    pub objects: Vec<Box<dyn Object>>,
}

impl ObjectList {
    pub fn new() -> Self {
        Self { objects: vec![] }
    }

    pub fn add(&mut self, obj: Box<dyn Object>) {
        self.objects.push(obj);
    }
}

impl Object for ObjectList {
    fn hit(&self, r: &Ray, ray_t: Interval, rec: &mut HRecord) -> bool {
        let mut hit_anything = false;
        let mut closest_so_far = ray_t.max;

        for obj in &self.objects {
            let mut tmp_rec = HRecord::new();

            if obj.hit(r, Interval::new(ray_t.min, closest_so_far), &mut tmp_rec) {
                hit_anything = true;
                closest_so_far = tmp_rec.t;
                *rec = tmp_rec;
            }
        }
        hit_anything
    }
}
