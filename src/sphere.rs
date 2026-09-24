use crate::{
    interval::Interval,
    object::{HRecord, Object},
    ray::Ray,
    vec3::Point,
};

pub struct Sphere {
    center: Point,
    radius: f32,
}

impl Sphere {
    pub fn new(center: Point, radius: f32) -> Self {
        Self {
            center,
            radius: radius.max(0.0),
        }
    }
}

impl Object for Sphere {
    fn hit(&self, r: &Ray, ray_t: Interval, rec: &mut HRecord) -> bool {
        let oc = self.center - r.origin;
        let a = r.direction.len_squared();
        let h = r.direction.dot(oc);
        let c = oc.len_squared() - self.radius * self.radius;

        let discriminant = h * h - a * c;
        if discriminant < 0.0 {
            return false;
        }

        let sqrtd = discriminant.sqrt();

        let mut root = (h - sqrtd) / a;
        if !ray_t.surrounds(root) {
            root = (h + sqrtd) / a;
            if !ray_t.surrounds(root) {
                return false;
            }
        }

        rec.t = root;
        rec.p = r.at(rec.t);
        let outward_normal = (rec.p - self.center) / self.radius;
        rec.set_face_normal(r, outward_normal);

        true
    }
}
