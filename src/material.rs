use std::sync::Arc;

use crate::{
    object::HRecord,
    ray::Ray,
    texture::{SolidColor, Texture},
    utils::Rng,
    vec3::{Color, Point, random_unit_vector, reflect, refract},
};

pub trait Material: Send + Sync {
    fn emitted(&self, _u: f32, _v: f32, _p: &Point) -> Color {
        Color::zero()
    }

    fn scatter(&self, ray: &Ray, hit: &HRecord, rng: &mut Rng) -> Option<(Color, Ray)>;
}

pub struct Lambertian {
    pub tex: Arc<dyn Texture>,
}

impl Lambertian {
    pub fn new(tex: Arc<dyn Texture>) -> Self {
        Self { tex }
    }
    pub fn from_color(albedo: Color) -> Self {
        Self {
            tex: Arc::new(SolidColor::new(albedo)),
        }
    }
}

impl Material for Lambertian {
    fn scatter(&self, ray: &Ray, hit: &HRecord, rng: &mut Rng) -> Option<(Color, Ray)> {
        let mut direction = hit.normal + random_unit_vector(rng);
        if direction.near_zero() {
            direction = hit.normal;
        }
        let scattered = Ray::new(hit.p, direction, ray.time);

        let attenuation = self.tex.value(hit.u, hit.v, &hit.p);
        Some((attenuation, scattered))
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
    fn scatter(&self, ray: &Ray, hit: &HRecord, rng: &mut Rng) -> Option<(Color, Ray)> {
        let reflected = reflect(ray.direction.normalize(), hit.normal);
        let direction = reflected + self.fuzz * random_unit_vector(rng);
        let scattered = Ray::new(hit.p, direction, ray.time);
        if scattered.direction.dot(hit.normal) <= 0.0 {
            return None;
        }
        Some((self.albedo, scattered))
    }
}

pub struct Dielectric {
    pub refraction_index: f32,
}
impl Dielectric {
    pub fn new(refraction_index: f32) -> Self {
        Self { refraction_index }
    }

    pub fn reflectance(cos: f32, refraction_index: f32) -> f32 {
        let r0 = ((1.0 - refraction_index) / (1.0 + refraction_index)).powi(2);
        r0 + (1.0 - r0) * (1.0 - cos).powi(5)
    }
}

impl Material for Dielectric {
    fn scatter(&self, ray: &Ray, hit: &HRecord, rng: &mut Rng) -> Option<(Color, Ray)> {
        let attenuation = Color::new(1.0, 1.0, 1.0);

        let ri = if hit.front_face {
            1.0 / self.refraction_index
        } else {
            self.refraction_index
        };

        let unit_direction = ray.direction.normalize();

        let cos_theta = (-unit_direction).dot(hit.normal).min(1.0);
        let sin_theta = (1.0 - cos_theta * cos_theta).sqrt();

        let cant_refract = ri * sin_theta > 1.0;

        let direction = if cant_refract || Self::reflectance(cos_theta, ri) > rng.random() {
            reflect(unit_direction, hit.normal)
        } else {
            refract(unit_direction, hit.normal, ri)
        };

        let scattered = Ray::new(hit.p, direction, ray.time);
        Some((attenuation, scattered))
    }
}

pub struct DiffuseLight {
    tex: Arc<dyn Texture>,
}

impl DiffuseLight {
    pub fn new(tex: Arc<dyn Texture>) -> Self {
        Self { tex }
    }

    pub fn from_color(emit: Color) -> Self {
        Self {
            tex: Arc::new(SolidColor::new(emit)),
        }
    }

    pub fn from_rgb(r: f32, g: f32, b: f32) -> Self {
        Self::from_color(Color::new(r, g, b))
    }
}

impl Material for DiffuseLight {
    fn emitted(&self, u: f32, v: f32, p: &Point) -> Color {
        self.tex.value(u, v, p)
    }

    fn scatter(&self, _ray: &Ray, _hit: &HRecord, _rng: &mut Rng) -> Option<(Color, Ray)> {
        None
    }
}
