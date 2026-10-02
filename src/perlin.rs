use crate::{
    utils::Rng,
    vec3::{Point, Vec3, random_unit_vector},
};

pub const POINT_COUNT: usize = 256;

pub struct Perlin {
    randvec: [Vec3; POINT_COUNT],
    perm_x: [usize; POINT_COUNT],
    perm_y: [usize; POINT_COUNT],
    perm_z: [usize; POINT_COUNT],
}

impl Perlin {
    pub fn new() -> Self {
        let mut rng = Rng::new(0);

        let mut randvec = [Vec3::zero(); POINT_COUNT];
        for val in &mut randvec {
            *val = random_unit_vector(&mut rng);
        }

        let perm_x = Self::generate_perm(&mut rng);
        let perm_y = Self::generate_perm(&mut rng);
        let perm_z = Self::generate_perm(&mut rng);

        Self {
            randvec,
            perm_x,
            perm_y,
            perm_z,
        }
    }

    pub fn noise(&self, p: &Point) -> f32 {
        let u = p.x - p.x.floor();
        let v = p.y - p.y.floor();
        let w = p.z - p.z.floor();

        let i = p.x.floor() as i32;
        let j = p.y.floor() as i32;
        let k = p.z.floor() as i32;

        let mut c = [[[Vec3::zero(); 2]; 2]; 2];

        for di in 0..2 {
            for dj in 0..2 {
                for dk in 0..2 {
                    let idx = self.perm_x[((i + di as i32) & 255) as usize]
                        ^ self.perm_y[((j + dj as i32) & 255) as usize]
                        ^ self.perm_z[((k + dk as i32) & 255) as usize];

                    c[di][dj][dk] = self.randvec[idx];
                }
            }
        }

        Self::perlin_interp(&c, u, v, w)
    }

    pub fn turb(&self, p: &Point, depth: usize) -> f32 {
        let mut accum = 0.0;
        let mut temp_p = *p;
        let mut weight = 1.0;

        for _ in 0..depth {
            accum += weight * self.noise(&temp_p);
            weight *= 0.5;
            temp_p = temp_p * 2.0;
        }

        accum.abs()
    }

    fn perlin_interp(c: &[[[Vec3; 2]; 2]; 2], u: f32, v: f32, w: f32) -> f32 {
        let uu = u * u * (3.0 - 2.0 * u);
        let vv = v * v * (3.0 - 2.0 * v);
        let ww = w * w * (3.0 - 2.0 * w);

        let mut accum = 0.0;

        for i in 0..2 {
            for j in 0..2 {
                for k in 0..2 {
                    let fi = i as f32;
                    let fj = j as f32;
                    let fk = k as f32;

                    let weight_v = Vec3::new(u - fi, v - fj, w - fk);

                    accum += (fi * uu + (1.0 - fi) * (1.0 - uu))
                        * (fj * vv + (1.0 - fj) * (1.0 - vv))
                        * (fk * ww + (1.0 - fk) * (1.0 - ww))
                        * c[i][j][k].dot(weight_v);
                }
            }
        }

        accum
    }

    fn generate_perm(rng: &mut Rng) -> [usize; POINT_COUNT] {
        let mut p = [0usize; POINT_COUNT];
        for i in 0..POINT_COUNT {
            p[i] = i;
        }

        Self::permute(&mut p, rng);
        p
    }

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
