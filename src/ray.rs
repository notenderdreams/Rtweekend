use crate::vec3::{Point, Vec3};

// P(t) = Origin + t * Direction
#[derive(Debug, Clone, Copy)]
pub struct Ray {
    pub origin: Point,
    pub direction: Vec3,
    pub time: f32,
}

impl Ray {
    #[inline]
    pub fn new(origin: Point, direction: Vec3, time: f32) -> Self {
        Self {
            origin,
            direction,
            time,
        }
    }

    #[inline]
    pub fn at(&self, t: f32) -> Point {
        self.origin + self.direction * t
    }
}
