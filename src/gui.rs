use std::sync::{
    Arc,
    atomic::{AtomicBool, AtomicU32, Ordering},
};

use minifb::{Key, Window, WindowOptions};

pub struct Preview {
    pub buffer: Arc<[AtomicU32]>,
    pub done: Arc<AtomicBool>,
    pub abort: Arc<AtomicBool>,
    w: usize,
    h: usize,
}

impl Preview {
    pub fn new(w: usize, h: usize) -> Self {
        let buffer: Vec<AtomicU32> = (0..w * h).map(|_| AtomicU32::new(0)).collect();
        Self {
            buffer: Arc::from(buffer.into_boxed_slice()),
            done: Arc::new(AtomicBool::new(false)),
            abort: Arc::new(AtomicBool::new(false)),
            w,
            h,
        }
    }

    pub fn set_pixel(&self, x: usize, y: usize, r: u8, g: u8, b: u8) {
        let i = y * self.w + x;
        let pkd = ((r as u32) << 16) | ((g as u32) << 8) | (b as u32);
        self.buffer[i].store(pkd, Ordering::Relaxed);
    }

    pub fn run(&self, title: &str) {
        let mut win = Window::new(title, self.w, self.h, WindowOptions::default())
            .expect("failed to open preview window");
        win.set_target_fps(30);

        let mut local = vec![0u32; self.w * self.h];
        while win.is_open() && !win.is_key_down(Key::Escape) {
            for (dst, src) in local.iter_mut().zip(self.buffer.iter()) {
                *dst = src.load(Ordering::Relaxed);
            }
            win.update_with_buffer(&local, self.w, self.h).unwrap();
        }

        if !self.done.load(Ordering::Relaxed) {
            self.abort.store(true, Ordering::Relaxed);
        }
    }
}
