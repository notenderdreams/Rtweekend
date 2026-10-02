use std::sync::{
    Arc, Mutex,
    atomic::{AtomicBool, AtomicU32, Ordering},
};

use minifb::{Key, KeyRepeat, Window, WindowOptions};

use crate::controller::CameraController;

pub struct Preview {
    pub buffer: Arc<[AtomicU32]>,
    pub abort: Arc<AtomicBool>,
    pub restart: Arc<AtomicBool>,
    pub clay: Arc<AtomicBool>,
    pub controller: Arc<Mutex<CameraController>>,
    w: usize,
    h: usize,
}

impl Preview {
    pub fn new(w: usize, h: usize, controller: Arc<Mutex<CameraController>>) -> Self {
        let buffer: Vec<AtomicU32> = (0..w * h).map(|_| AtomicU32::new(0)).collect();
        Self {
            buffer: Arc::from(buffer.into_boxed_slice()),
            abort: Arc::new(AtomicBool::new(false)),
            restart: Arc::new(AtomicBool::new(false)),
            clay: Arc::new(AtomicBool::new(false)),
            controller,
            w,
            h,
        }
    }

    pub fn set_pixel(&self, x: usize, y: usize, r: u8, g: u8, b: u8) {
        let i = y * self.w + x;
        let pkd = ((r as u32) << 16) | ((g as u32) << 8) | (b as u32);
        self.buffer[i].store(pkd, Ordering::Relaxed);
    }

    pub fn to_ppm(&self) -> String {
        let mut ppm = format!("P3\n{} {}\n255\n", self.w, self.h);
        for y in 0..self.h {
            for x in 0..self.w {
                let pkd = self.buffer[y * self.w + x].load(Ordering::Relaxed);
                let r = (pkd >> 16) & 0xff;
                let g = (pkd >> 8) & 0xff;
                let b = pkd & 0xff;
                ppm.push_str(&format!("{r} {g} {b} "));
            }
            ppm.push('\n');
        }
        ppm
    }

    pub fn run(&self, title: &str) {
        let mut win = Window::new(title, self.w, self.h, WindowOptions::default())
            .expect("failed to open preview window");
        win.set_target_fps(30);

        let mut local = vec![0u32; self.w * self.h];

        while win.is_open() && !win.is_key_down(Key::Escape) {
            if win.is_key_pressed(Key::Tab, KeyRepeat::No) {
                let next_clay = !self.clay.load(Ordering::Relaxed);
                self.clay.store(next_clay, Ordering::Relaxed);
                self.restart.store(true, Ordering::Relaxed);
            }

            if win.is_key_pressed(Key::P, KeyRepeat::No) {
                if std::fs::write("output.ppm", self.to_ppm()).is_ok() {
                    println!("Saved screenshot to output.ppm");
                }
            }

            if self.controller.lock().unwrap().update(&win) {
                self.restart.store(true, Ordering::Relaxed);
                win.set_title(&self.controller.lock().unwrap().status());
            }

            for (dst, src) in local.iter_mut().zip(self.buffer.iter()) {
                *dst = src.load(Ordering::Relaxed);
            }
            win.update_with_buffer(&local, self.w, self.h).unwrap();
        }

        self.abort.store(true, Ordering::Relaxed);
    }
}
