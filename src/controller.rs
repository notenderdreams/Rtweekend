use minifb::{Key, KeyRepeat, Window};

use crate::vec3::{Point, Vec3};
#[derive(Clone, Copy)]
pub struct CameraState {
    pub lookfrom: Point,
    pub lookat: Point,
    pub vup: Vec3,
    pub vfov: f32,
}

pub struct CameraController {
    pub current: CameraState,
    pub default: CameraState,
    pub move_speed: f32,
    pub rot_speed: f32,
}

impl CameraController {
    pub fn new(lookfrom: Point, lookat: Point, vup: Vec3, vfov: f32) -> Self {
        let state = CameraState {
            lookfrom,
            lookat,
            vup,
            vfov,
        };
        Self {
            current: state,
            default: state,
            move_speed: 0.3,
            rot_speed: 0.3,
        }
    }

    pub fn update(&mut self, win: &Window) -> bool {
        let mut changed = false;

        if win.is_key_pressed(Key::R, KeyRepeat::No) {
            self.current = self.default;
            return true;
        }

        let dir = self.current.lookat - self.current.lookfrom;
        let dist = dir.len();
        if dist < 1e-4 {
            return false;
        }

        let forward = dir / dist;
        let right = forward.cross(self.current.vup).normalize();
        let up = Vec3::new(0.0, 1.0, 0.0);

        let mut move_vec = Vec3::zero();
        if win.is_key_down(Key::W) {
            move_vec = move_vec + forward;
        }
        if win.is_key_down(Key::S) {
            move_vec = move_vec - forward;
        }
        if win.is_key_down(Key::A) {
            move_vec = move_vec - right;
        }
        if win.is_key_down(Key::D) {
            move_vec = move_vec + right;
        }
        if win.is_key_down(Key::E) {
            move_vec = move_vec + up;
        }
        if win.is_key_down(Key::Q) {
            move_vec = move_vec - up;
        }

        if move_vec.len_squared() > 1e-6 {
            let delta = move_vec.normalize() * self.move_speed;
            self.current.lookfrom = self.current.lookfrom + delta;
            changed = true;
        }

        if win.is_key_down(Key::Left) || win.is_key_down(Key::Right) {
            let angle = if win.is_key_down(Key::Left) {
                -self.rot_speed
            } else {
                self.rot_speed
            };
            let (cos_a, sin_a) = (angle.cos(), angle.sin());
            let current_dir = self.current.lookat - self.current.lookfrom;
            let new_x = current_dir.x * cos_a - current_dir.z * sin_a;
            let new_z = current_dir.x * sin_a + current_dir.z * cos_a;
            self.current.lookat = self.current.lookfrom + Vec3::new(new_x, current_dir.y, new_z);
            changed = true;
        }

        if win.is_key_down(Key::Up) || win.is_key_down(Key::Down) {
            let angle = if win.is_key_down(Key::Up) {
                self.rot_speed
            } else {
                -self.rot_speed
            };
            let (cos_a, sin_a) = (angle.cos(), angle.sin());
            let curr_fwd = (self.current.lookat - self.current.lookfrom).normalize();
            let curr_right = curr_fwd.cross(self.current.vup).normalize();
            let curr_up = curr_fwd.cross(curr_right).normalize();
            let new_fwd = (curr_fwd * cos_a + curr_up * sin_a).normalize();
            self.current.lookat = self.current.lookfrom + new_fwd;

            if new_fwd.y.abs() < 0.98 {
                self.current.lookat = self.current.lookfrom + new_fwd * dist;
                changed = true;
            }
        }

        if win.is_key_down(Key::LeftBracket) {
            self.current.vfov = (self.current.vfov - 1.0).max(5.0);
            changed = true;
        }

        if win.is_key_down(Key::RightBracket) {
            self.current.vfov = (self.current.vfov + 1.0).min(120.0);
            changed = true;
        }

        changed
    }

    pub fn status(&self) -> String {
        format!(
            "Pos: ({:.1}, {:.1}, {:.1}) | FOV: {:.0}°",
            self.current.lookfrom.x,
            self.current.lookfrom.y,
            self.current.lookfrom.z,
            self.current.vfov,
        )
    }
}
