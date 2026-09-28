use std::ops::{Add, Div, Mul, Neg, Sub};

use crate::utils::Rng;

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

    pub fn rand(rng: &mut Rng) -> Self {
        Self::new(
            rng.random_range(0.0, 1.0),
            rng.random_range(0.0, 1.0),
            rng.random_range(0.0, 1.0),
        )
    }
    pub fn rand_in(min: f32, max: f32, rng: &mut Rng) -> Self {
        Self::new(
            rng.random_range(min, max),
            rng.random_range(min, max),
            rng.random_range(min, max),
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

    pub fn near_zero(&self) -> bool {
        let s = 1e-8;
        (self.x.abs() < s) && (self.y.abs() < s) && (self.z.abs() < s)
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

pub fn random_unit_vector(rng: &mut Rng) -> Vec3 {
    loop {
        let p = Vec3::rand_in(-1.0, 1.0, rng);
        let lensq = p.len_squared();
        if 1e-38 < lensq && lensq <= 1.0 {
            return p.normalize();
        }
    }
}

pub fn random_in_unit_disk(rng: &mut Rng) -> Vec3 {
    loop {
        let p = Vec3::new(
            rng.random_range(-1.0, 1.0),
            rng.random_range(-1.0, 1.0),
            0.0,
        );
        if p.len_squared() < 1.0 {
            return p;
        }
    }
}

pub fn reflect(v: Vec3, n: Vec3) -> Vec3 {
    v - 2.0 * v.dot(n) * n
}

pub fn refract(uv: Vec3, n: Vec3, etai_over_etat: f32) -> Vec3 {
    // Snell's Law (Vector Form):
    // Ref: Peter Shirley, "Ray Tracing in One Weekend", Section 11.2 "Snell's Law"
    // https://raytracing.github.io/books/RayTracingInOneWeekend.html#dielectrics/snell'slaw
    //
    //   η · sin(θ) = η' · sin(θ')  =>  sin(θ') = (η / η') · sin(θ)
    //
    // Decompose refracted ray R' into perpendicular and parallel components relative to n:
    //   R' = R'⟂ + R'∥
    //
    // Component Equations:
    //   cos(θ) = (-uv) · n
    //   R'⟂    = (η / η') · (uv + cos(θ) · n)
    //   R'∥    = -sqrt(1 - |R'⟂|²) · n
    //   R'     = R'∥ + R'⟂
    //
    //               Normal n
    //                  ▲
    //     Incoming     │
    //     Ray uv ╲     │
    //             ╲ θ  │
    //              ╲   │
    // ──────────────▼──┴────────────── Surface
    //               ╲  │
    //                ╲ │
    //                 ╲│ R'∥ (along -n)
    //                  ▼
    //                   ╲   R'⟂ (transverse)
    //                    ▼
    //                     R' = R'∥ + R'⟂
    //
    let cos_theta = (-uv).dot(n).min(1.0);

    let r_out_perp = etai_over_etat * (uv + cos_theta * n);

    let r_out_parallel = -(1.0 - r_out_perp.len_squared()).abs().sqrt() * n;

    r_out_parallel + r_out_perp
}
