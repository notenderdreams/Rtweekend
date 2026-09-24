use crate::{
    object::{HRecord, Object},
    ray::Ray,
    vec3::{Point, Vec3},
};

pub struct ObjectList {
    pub objects: Vec<Box<dyn Object>>,
}

impl ObjectList {
    pub fn new() -> Self {
        Self { objects: vec![] }
    }
    pub fn clear(&mut self) {
        self.objects.clear();
    }

    pub fn add(&mut self, obj: Box<dyn Object>) {
        self.objects.push(obj);
    }
}

impl Object for ObjectList {
    fn hit(&self, r: &Ray, tmin: f32, tmax: f32, rec: &mut HRecord) -> bool {
        let mut hit_anything = false;
        let mut closest = tmax;

        for obj in &self.objects {
            let mut tmp_rec = HRecord {
                p: Point::zero(),
                normal: Vec3::zero(),
                t: 0.0,
                front_face: false,
            };
            if obj.hit(r, tmin, closest, &mut tmp_rec) {
                hit_anything = true;
                closest = tmp_rec.t;
                *rec = tmp_rec;
            }
        }
        hit_anything
    }
}
