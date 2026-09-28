use std::sync::Arc;

use crate::{
    interval::Interval,
    material::Material,
    object::{HRecord, Object},
    ray::Ray,
    vec3::Point,
};

pub struct Sphere {
    center: Point,
    radius: f32,
    mat: Arc<dyn Material>,
}

impl Sphere {
    pub fn new(center: Point, radius: f32, mat: Arc<dyn Material>) -> Self {
        Self {
            center,
            radius: radius.max(0.0),
            mat,
        }
    }
}

impl Object for Sphere {
    fn hit(&self, r: &Ray, ray_t: Interval, rec: &mut HRecord) -> bool {
        // Let center C = (cx, cy, cz) and radius R and a Point P(x,y,z)
        // R²  = (P - C) · (P - C)
        //     = ((O + t * D) - C) · ((O + t * D) - C)  ; given P(t) = O + t * D
        //     = (t * D + V) · (t * D + V)              ; given V = O - C
        //
        // (D · D) * t² + 2 * (D · V) * t + (V · V) - R² = 0
        //
        // Quadratic equation in t:
        //      a * t² + b * t + c = 0
        // where:
        //      a = D · D
        //      b = 2 * (D · V) = 2 * (D · (O - C))
        //      c = V · V - R²
        //
        // Optimization: Let oc = (C - O) = -V, and let h = D · oc
        // Then b = -2h. Substituting b into t = (-b ± √(b² - 4ac)) / 2a:
        //      t = (2h ± √(4h² - 4ac)) / 2a
        //      t = (2h ± 2 * √(h² - ac)) / 2a
        //      t = (h ± √(h² - ac)) / a
        //
        // Therefore:
        //      discriminant = h² - ac
        //      root = (h ± √discriminant) / a

        let oc = self.center - r.origin;
        let a = r.direction.len_squared();
        let h = r.direction.dot(oc);
        let c = oc.len_squared() - self.radius * self.radius;

        let discriminant = h * h - a * c;
        if discriminant < 0.0 {
            return false;
        }

        // The quadratic equation has two roots:
        //      near root = (h - sqrtd) / a   (entry point)
        //      far root  = (h + sqrtd) / a   (exit point)
        // Find the nearest root that lies in the acceptable [t_min, t_max] range.
        let sqrtd = discriminant.sqrt();

        let mut root = (h - sqrtd) / a;

        if !ray_t.surrounds(root) {
            // Near root is behind ray origin or occluded; try the far root
            // (e.g. inside a glass bubble)
            root = (h + sqrtd) / a;
            if !ray_t.surrounds(root) {
                return false;
            }
        }

        rec.t = root;
        rec.p = r.at(rec.t);
        // Vector from center to hit point (P - C) points outward.
        // Since |P - C| = radius, dividing by radius normalizes it.
        let outward_normal = (rec.p - self.center) / self.radius;

        rec.set_face_normal(r, outward_normal);
        rec.mat = Some(Arc::clone(&self.mat));

        true
    }
}
