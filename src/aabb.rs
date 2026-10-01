use crate::{interval::Interval, ray::Ray, vec3::Point};

#[derive(Debug, Clone, Copy, Default, PartialEq)]
pub struct AABB {
    pub x: Interval,
    pub y: Interval,
    pub z: Interval,
}

impl AABB {
    pub const EMPTY: Self = Self {
        x: Interval::EMPTY,
        y: Interval::EMPTY,
        z: Interval::EMPTY,
    };

    pub const UNIVERSE: Self = Self {
        x: Interval::UNIVERSE,
        y: Interval::UNIVERSE,
        z: Interval::UNIVERSE,
    };

    #[inline]
    pub fn new(x: Interval, y: Interval, z: Interval) -> Self {
        Self { x, y, z }
    }

    pub fn from_points(a: Point, b: Point) -> Self {
        Self {
            x: Interval::new(a.x.min(b.x), a.x.max(b.x)),
            y: Interval::new(a.y.min(b.y), a.y.max(b.y)),
            z: Interval::new(a.z.min(b.z), a.z.max(b.z)),
        }
    }

    pub fn enclose(box0: &Self, box1: &Self) -> Self {
        Self {
            x: Interval::enclose(&box0.x, &box1.x),
            y: Interval::enclose(&box0.y, &box1.y),
            z: Interval::enclose(&box0.z, &box1.z),
        }
    }
    #[inline]
    pub fn axis_interval(&self, n: usize) -> &Interval {
        match n {
            1 => &self.y,
            2 => &self.z,
            _ => &self.x,
        }
    }

    /// Pads any flat side of the box so that no dimension has zero thickness.
    /// This prevents divide-by-zero or ray intersection misses on 2D planes/quads.
    pub fn pad(&self) -> Self {
        let delta = 0.0001;
        let new_x = if self.x.size() >= delta {
            self.x
        } else {
            self.x.expand(delta)
        };
        let new_y = if self.y.size() >= delta {
            self.y
        } else {
            self.y.expand(delta)
        };
        let new_z = if self.z.size() >= delta {
            self.z
        } else {
            self.z.expand(delta)
        };
        Self::new(new_x, new_y, new_z)
    }

    pub fn hit(&self, r: &Ray, mut ray_t: Interval) -> bool {
        let axes = [
            (self.x, r.origin.x, r.direction.x),
            (self.y, r.origin.y, r.direction.y),
            (self.z, r.origin.z, r.direction.z),
        ];

        for (ax, orig, dir) in axes {
            let inv_d = 1.0 / dir;
            let mut t0 = (ax.min - orig) * inv_d;
            let mut t1 = (ax.max - orig) * inv_d;

            if inv_d < 0.0 {
                std::mem::swap(&mut t0, &mut t1);
            }

            if t0 > ray_t.min {
                ray_t.min = t0;
            }
            if t1 < ray_t.max {
                ray_t.max = t1;
            }

            if ray_t.max <= ray_t.min {
                return false;
            }
        }
        true
    }
}
