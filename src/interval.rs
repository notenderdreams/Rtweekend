use std::ops::Add;

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Interval {
    pub min: f32,
    pub max: f32,
}

impl Interval {
    #[inline]
    pub fn new(min: f32, max: f32) -> Self {
        Self { min, max }
    }

    #[inline]
    pub fn enclose(a: &Self, b: &Self) -> Self {
        Self {
            min: a.min.min(b.min),
            max: a.max.max(b.max),
        }
    }

    #[inline]
    pub fn expand(&self, delta: f32) -> Self {
        let padding = delta / 2.0;
        Self::new(self.min - padding, self.max + padding)
    }

    #[inline]
    pub fn size(&self) -> f32 {
        self.max - self.min
    }

    #[inline]
    pub fn surrounds(&self, x: f32) -> bool {
        self.min < x && x < self.max
    }

    #[inline]
    pub fn contains(&self, x: f32) -> bool {
        self.min <= x && x <= self.max
    }

    #[inline]
    pub fn clamp(&self, x: f32) -> f32 {
        if x < self.min {
            self.min
        } else if x > self.max {
            self.max
        } else {
            x
        }
    }

    pub const EMPTY: Self = Self {
        min: f32::INFINITY,
        max: f32::NEG_INFINITY,
    };

    pub const UNIVERSE: Self = Self {
        min: f32::NEG_INFINITY,
        max: f32::INFINITY,
    };

    // pub fn empty() -> Self {
    //     Self::EMPTY
    // }

    // pub fn universe() -> Self {
    //     Self::UNIVERSE
    // }
}

impl Default for Interval {
    fn default() -> Self {
        Self::EMPTY
    }
}

impl Add<f32> for Interval {
    type Output = Self;

    fn add(self, displacement: f32) -> Self::Output {
        Self::new(self.min + displacement, self.max + displacement)
    }
}

impl Add<Interval> for f32 {
    type Output = Interval;

    fn add(self, interval: Interval) -> Interval {
        interval + self
    }
}
