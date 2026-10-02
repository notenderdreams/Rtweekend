use std::{
    f32::{INFINITY, NEG_INFINITY},
    sync::Arc,
};

use crate::{
    aabb::AABB,
    interval::Interval,
    object::{HRecord, Object},
    ray::Ray,
    vec3::{Point, Vec3},
};

pub struct Translate {
    object: Arc<dyn Object>,
    offset: Vec3,
    bbox: AABB,
}

impl Translate {
    pub fn new(object: Arc<dyn Object>, offset: Vec3) -> Self {
        let bbox = object.bounding_box() + offset;
        Self {
            object,
            offset,
            bbox,
        }
    }
}

impl Object for Translate {
    fn hit(&self, r: &Ray, ray_t: Interval, rec: &mut HRecord) -> bool {
        let offset_ray = Ray::new(r.origin - self.offset, r.direction, r.time);

        if !self.object.hit(&offset_ray, ray_t, rec) {
            return false;
        }

        rec.p = rec.p + self.offset;
        true
    }

    fn bounding_box(&self) -> AABB {
        self.bbox
    }
}

pub struct RotateY {
    object: Arc<dyn Object>,
    sin_theta: f32,
    cos_theta: f32,
    bbox: AABB,
}

impl RotateY {
    pub fn new(object: Arc<dyn Object>, angle: f32) -> Self {
        let radians = angle.to_radians();
        let sin_theta = radians.sin();
        let cos_theta = radians.cos();
        let original_bbox = object.bounding_box();

        let mut min = Point::new(INFINITY, INFINITY, INFINITY);
        let mut max = Point::new(NEG_INFINITY, NEG_INFINITY, NEG_INFINITY);

        for i in 0..2 {
            for j in 0..2 {
                for k in 0..2 {
                    let fi = i as f32;
                    let fj = j as f32;
                    let fk = k as f32;

                    let x = fi * original_bbox.x.max + (1.0 - fi) * original_bbox.x.min;
                    let y = fj * original_bbox.y.max + (1.0 - fj) * original_bbox.y.min;
                    let z = fk * original_bbox.z.max + (1.0 - fk) * original_bbox.z.min;

                    // Forward rotation of corner
                    let new_x = cos_theta * x + sin_theta * z;
                    let new_z = -sin_theta * x + cos_theta * z;

                    min.x = min.x.min(new_x);
                    max.x = max.x.max(new_x);
                    min.y = min.y.min(y);
                    max.y = max.y.max(y);
                    min.z = min.z.min(new_z);
                    max.z = max.z.max(new_z);
                }
            }
        }
        let bbox = AABB::from_points(min, max).pad();
        Self {
            object,
            sin_theta,
            cos_theta,
            bbox,
        }
    }
}

impl Object for RotateY {
    fn hit(&self, r: &Ray, ray_t: Interval, rec: &mut HRecord) -> bool {
        let origin = Point::new(
            self.cos_theta * r.origin.x - self.sin_theta * r.origin.z,
            r.origin.y,
            self.sin_theta * r.origin.x + self.cos_theta * r.origin.z,
        );

        let direction = Vec3::new(
            self.cos_theta * r.direction.x - self.sin_theta * r.direction.z,
            r.direction.y,
            self.sin_theta * r.direction.x + self.cos_theta * r.direction.z,
        );

        let rotated_ray = Ray::new(origin, direction, r.time);
        if !self.object.hit(&rotated_ray, ray_t, rec) {
            return false;
        }

        rec.p = Point::new(
            self.cos_theta * rec.p.x + self.sin_theta * rec.p.z,
            rec.p.y,
            -self.sin_theta * rec.p.x + self.cos_theta * rec.p.z,
        );

        rec.normal = Vec3::new(
            self.cos_theta * rec.normal.x + self.sin_theta * rec.normal.z,
            rec.normal.y,
            -self.sin_theta * rec.normal.x + self.cos_theta * rec.normal.z,
        );

        true
    }

    fn bounding_box(&self) -> AABB {
        self.bbox.clone()
    }
}
