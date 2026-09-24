use crate::{
    ray::Ray,
    vec3::{Point, Vec3},
};

pub struct HRecord {
    pub p: Point,
    pub normal: Vec3,
    pub t: f32,
    pub front_face: bool,
}

impl HRecord {
    pub fn set_face_normal(&mut self, r: &Ray, outward_normal: Vec3) {
        self.front_face = r.direction.dot(outward_normal) < 0.0;
        self.normal = if self.front_face {
            outward_normal
        } else {
            -outward_normal
        };
    }
}

pub trait Object {
    fn hit(&self, r: &Ray, tmin: f32, tmax: f32, rec: &mut HRecord) -> bool;
}
