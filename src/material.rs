use crate::{
    object::HRecord,
    ray::Ray,
    vec3::{Color, random_unit_vector},
};

pub trait Material: Send + Sync {
    fn scatter(&self, ray: &Ray, hit: &HRecord) -> Option<(Color, Ray)>;
}

pub struct Lambertian {
    pub albedo: Color,
}

impl Lambertian {
    pub fn new(albedo: Color) -> Self {
        Self { albedo }
    }
}

impl Material for Lambertian {
    fn scatter(&self, ray: &Ray, hit: &HRecord) -> Option<(Color, Ray)> {
        let direction = hit.normal + random_unit_vector();
        let scattered = Ray::new(hit.p, direction);
        Some((self.albedo, scattered))
    }
}
