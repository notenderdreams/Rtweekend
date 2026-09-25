use crate::{
    object::HRecord,
    ray::Ray,
    vec3::{Color, random_unit_vector, reflect},
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
        let mut direction = hit.normal + random_unit_vector();
        if direction.near_zero() {
            direction = hit.normal;
        }
        let scattered = Ray::new(hit.p, direction);
        Some((self.albedo, scattered))
    }
}

pub struct Metal {
    pub albedo: Color,
    pub fuzz: f32,
}

impl Metal {
    pub fn new(albedo: Color, fuzz: f32) -> Self {
        Self { albedo, fuzz }
    }
}

impl Material for Metal {
    fn scatter(&self, ray: &Ray, hit: &HRecord) -> Option<(Color, Ray)> {
        let reflected = reflect(ray.direction.normalize(), hit.normal);
        let direction = reflected + self.fuzz * random_unit_vector();
        let scattered = Ray::new(hit.p, direction);
        if scattered.direction.dot(hit.normal) <= 0.0 {
            return None;
        }
        Some((self.albedo, scattered))
    }
}
