use std::sync::Arc;

use crate::{
    aabb::AABB,
    interval::Interval,
    material::Material,
    ray::Ray,
    vec3::{Point, Vec3},
};

pub struct HRecord {
    pub p: Point,
    pub normal: Vec3,
    pub mat: Option<Arc<dyn Material>>,
    pub t: f32,
    pub u: f32,
    pub v: f32,
    pub front_face: bool,
}

impl HRecord {
    pub fn new() -> Self {
        Self {
            p: Point::zero(),
            normal: Vec3::zero(),
            mat: None,
            t: 0.0,
            u: 0.0,
            v: 0.0,
            front_face: false,
        }
    }

    pub fn set_face_normal(&mut self, r: &Ray, outward_normal: Vec3) {
        self.front_face = r.direction.dot(outward_normal) < 0.0;
        self.normal = if self.front_face {
            outward_normal
        } else {
            -outward_normal
        };
    }
}

pub trait Object: Send + Sync {
    fn hit(&self, r: &Ray, ray_t: Interval, rec: &mut HRecord) -> bool;

    fn bounding_box(&self) -> AABB;
}
