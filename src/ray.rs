use crate::vec3::{Point, Vec3};

// P(t) = Origin + t * Direction
pub struct Ray {
    pub origin: Point,
    pub direction: Vec3,
}

impl Ray {
    pub fn new(origin: Point, direction: Vec3) -> Self {
        Self { origin, direction }
    }
    pub fn at(&self, t: f32) -> Point {
        self.origin + self.direction * t
    }
}
