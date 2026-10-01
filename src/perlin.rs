use crate::{utils::Rng, vec3::Point};

pub const POINT_COUNT: usize = 256;

pub struct Perlin {
    randfloat: [f32; POINT_COUNT],
    perm_x: [usize; POINT_COUNT],
    perm_y: [usize; POINT_COUNT],
    perm_z: [usize; POINT_COUNT],
}

impl Perlin {
    pub fn new() -> Self {
        let mut rng = Rng::new(0);

        let mut randfloat = [0.0; POINT_COUNT];
        for val in &mut randfloat {
            *val = rng.random();
        }

        let perm_x = Self::generate_perm(&mut rng);
        let perm_y = Self::generate_perm(&mut rng);
        let perm_z = Self::generate_perm(&mut rng);

        Self {
            randfloat,
            perm_x,
            perm_y,
            perm_z,
        }
    }

    /// Evaluates hashed lattice noise at point p.
    pub fn noise(&self, p: &Point) -> f32 {
        // Use floor() to handle negative coordinates smoothly
        let i = ((4.0 * p.x).floor() as i32 & 255) as usize;
        let j = ((4.0 * p.y).floor() as i32 & 255) as usize;
        let k = ((4.0 * p.z).floor() as i32 & 255) as usize;

        let index = self.perm_x[i] ^ self.perm_y[j] ^ self.perm_z[k];
        self.randfloat[index]
    }

    fn generate_perm(rng: &mut Rng) -> [usize; POINT_COUNT] {
        let mut p = [0usize; POINT_COUNT];
        for i in 0..POINT_COUNT {
            p[i] = i;
        }

        Self::permute(&mut p, rng);
        p
    }

    /// Fisher-Yates shuffle
    fn permute(p: &mut [usize; POINT_COUNT], rng: &mut Rng) {
        for i in (1..POINT_COUNT).rev() {
            let target = (rng.random() * (i + 1) as f32) as usize;
            p.swap(i, target.min(i));
        }
    }
}

impl Default for Perlin {
    fn default() -> Self {
        Self::new()
    }
}
