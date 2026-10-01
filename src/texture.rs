use std::sync::Arc;

use crate::vec3::{Color, Point};

pub trait Texture: Send + Sync {
    fn value(&self, u: f32, v: f32, p: &Point) -> Color;
}

pub struct SolidColor {
    albedo: Color,
}

impl SolidColor {
    pub fn new(albedo: Color) -> Self {
        Self { albedo }
    }

    pub fn from_rgb(r: f32, g: f32, b: f32) -> Self {
        Self::new(Color::new(r, g, b))
    }
}

impl Texture for SolidColor {
    fn value(&self, _u: f32, _v: f32, _p: &Point) -> Color {
        self.albedo
    }
}

pub struct CheckerTexture {
    inv_scale: f32,
    even: Arc<dyn Texture>,
    odd: Arc<dyn Texture>,
}

impl CheckerTexture {
    pub fn new(scale: f32, even: Arc<dyn Texture>, odd: Arc<dyn Texture>) -> Self {
        Self {
            inv_scale: 1.0 / scale,
            even,
            odd,
        }
    }

    pub fn from_colors(scale: f32, even: Color, odd: Color) -> Self {
        Self::new(
            scale,
            Arc::new(SolidColor::new(even)),
            Arc::new(SolidColor::new(odd)),
        )
    }
}

impl Texture for CheckerTexture {
    fn value(&self, u: f32, v: f32, p: &Point) -> Color {
        let x_int = (self.inv_scale * p.x).floor() as i32;
        let y_int = (self.inv_scale * p.y).floor() as i32;
        let z_int = (self.inv_scale * p.z).floor() as i32;

        // rem_euclid to correctly handle negative coordinate remainders
        let is_even = (x_int + y_int + z_int).rem_euclid(2) == 0;
        if is_even {
            self.even.value(u, v, p)
        } else {
            self.odd.value(u, v, p)
        }
    }
}
