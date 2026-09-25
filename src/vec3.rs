use std::ops::{Add, Div, Mul, Neg, Sub};

use crate::utils::random_range;

#[derive(Debug, Clone, Copy, Default)]
pub struct Vec3 {
    pub x: f32,
    pub y: f32,
    pub z: f32,
}

pub type Point = Vec3;
pub type Color = Vec3;

impl Vec3 {
    pub fn new(x: f32, y: f32, z: f32) -> Self {
        Self { x, y, z }
    }

    pub fn zero() -> Self {
        Self::new(0.0, 0.0, 0.0)
    }

    pub fn rand() -> Self {
        Self::new(
            random_range(0.0, 1.0),
            random_range(0.0, 1.0),
            random_range(0.0, 1.0),
        )
    }
    pub fn rand_in(min: f32, max: f32) -> Self {
        Self::new(
            random_range(min, max),
            random_range(min, max),
            random_range(min, max),
        )
    }

    pub fn len_squared(&self) -> f32 {
        self.x * self.x + self.y * self.y + self.z * self.z
    }
    pub fn len(&self) -> f32 {
        self.len_squared().sqrt()
    }
    pub fn dot(&self, other: Self) -> f32 {
        self.x * other.x + self.y * other.y + self.z * other.z
    }
    pub fn cross(&self, other: Self) -> Self {
        Self::new(
            self.y * other.z - self.z * other.y,
            self.z * other.x - self.x * other.z,
            self.x * other.y - self.y * other.x,
        )
    }
    pub fn normalize(&self) -> Self {
        *self / self.len()
    }
}

pub fn random_unit_vector() -> Vec3 {
    loop {
        let p = Vec3::rand_in(-1.0, 1.0);
        let lensq = p.len_squared();
        if 1e-38 < lensq && lensq <= 1.0 {
            return p.normalize();
        }
    }
}

pub fn random_on_hemisphere(normal: Vec3) -> Vec3 {
    let on_unit_sphere = random_unit_vector();
    if on_unit_sphere.dot(normal) > 0.0 {
        on_unit_sphere
    } else {
        -on_unit_sphere
    }
}

impl Add for Vec3 {
    type Output = Self;
    fn add(self, rhs: Self) -> Self {
        Self::new(self.x + rhs.x, self.y + rhs.y, self.z + rhs.z)
    }
}

impl Sub for Vec3 {
    type Output = Self;
    fn sub(self, rhs: Self) -> Self {
        Self::new(self.x - rhs.x, self.y - rhs.y, self.z - rhs.z)
    }
}

impl Mul<f32> for Vec3 {
    type Output = Self;
    fn mul(self, t: f32) -> Self {
        Self::new(self.x * t, self.y * t, self.z * t)
    }
}

impl Mul<Vec3> for f32 {
    type Output = Vec3;
    fn mul(self, v: Vec3) -> Vec3 {
        v * self
    }
}

impl Mul<Vec3> for Vec3 {
    type Output = Self;

    fn mul(self, rhs: Vec3) -> Self {
        Self::new(self.x * rhs.x, self.y * rhs.y, self.z * rhs.z)
    }
}

impl Div<f32> for Vec3 {
    type Output = Self;
    fn div(self, t: f32) -> Self {
        self * (1.0 / t)
    }
}

impl Neg for Vec3 {
    type Output = Self;
    fn neg(self) -> Self {
        Self::new(-self.x, -self.y, -self.z)
    }
}
