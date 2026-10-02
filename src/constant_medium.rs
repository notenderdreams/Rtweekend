use std::{cell::RefCell, sync::Arc};

use crate::{
    aabb::AABB,
    interval::Interval,
    material::{Isotropic, Material},
    object::{HRecord, Object},
    ray::Ray,
    texture::Texture,
    utils::Rng,
    vec3::{Color, Vec3},
};

thread_local! {
    static THREAD_RNG: RefCell<Rng> = RefCell::new(Rng::from_entropy());
}

pub struct ConstantMedium {
    boundary: Arc<dyn Object>,
    neg_inv_density: f32,
    phase_function: Arc<dyn Material>,
}

impl ConstantMedium {
    pub fn new(boundary: Arc<dyn Object>, density: f32, tex: Arc<dyn Texture>) -> Self {
        Self {
            boundary,
            neg_inv_density: -1.0 / density,
            phase_function: Arc::new(Isotropic::new(tex)),
        }
    }

    pub fn from_color(boundary: Arc<dyn Object>, density: f32, albedo: Color) -> Self {
        Self {
            boundary,
            neg_inv_density: -1.0 / density,
            phase_function: Arc::new(Isotropic::from_color(albedo)),
        }
    }
}

impl Object for ConstantMedium {
    fn hit(&self, r: &Ray, ray_t: Interval, rec: &mut HRecord) -> bool {
        let mut rec1 = HRecord::new();
        let mut rec2 = HRecord::new();

        // 1. Find entry intersection across all space
        if !self.boundary.hit(r, Interval::UNIVERSE, &mut rec1) {
            return false;
        }

        // 2. Find exit intersection past rec1.t
        if !self
            .boundary
            .hit(r, Interval::new(rec1.t + 0.0001, f32::INFINITY), &mut rec2)
        {
            return false;
        }

        // 3. Clamp intervals to ray search range
        if rec1.t < ray_t.min {
            rec1.t = ray_t.min;
        }
        if rec2.t > ray_t.max {
            rec2.t = ray_t.max;
        }

        // 4. If entry is past exit, ray misses volume range
        if rec1.t >= rec2.t {
            return false;
        }

        // 5. If origin is inside the volume, clamp entry to 0
        if rec1.t < 0.0 {
            rec1.t = 0.0;
        }

        // 6. Calculate real-world travel distance inside boundary
        let ray_length = r.direction.len();
        let distance_inside_boundary = (rec2.t - rec1.t) * ray_length;

        // 7. Sample exponential free path
        let hit_distance = self.neg_inv_density
            * THREAD_RNG.with(|rng| rng.borrow_mut().random().max(f32::EPSILON).ln());

        if hit_distance > distance_inside_boundary {
            return false;
        }

        // 8. Populate hit record
        rec.t = rec1.t + hit_distance / ray_length;
        rec.p = r.at(rec.t);

        // Normal and front_face are arbitrary for volumetric scattering
        rec.normal = Vec3::new(1.0, 0.0, 0.0);
        rec.front_face = true;
        rec.mat = Some(Arc::clone(&self.phase_function));

        true
    }

    fn bounding_box(&self) -> AABB {
        self.boundary.bounding_box()
    }
}
