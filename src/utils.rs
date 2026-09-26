use std::{
    io::{self, Write},
    sync::atomic::{AtomicU64, Ordering},
    time::Instant,
};

use crate::{interval::Interval, vec3::Color};

// xorshift64
static SEED_COUNTER: AtomicU64 = AtomicU64::new(0x853c_49e6_748f_ea9b);

#[derive(Clone, Copy)]
pub struct Rng {
    state: u64,
}

impl Rng {
    pub fn new(seed: u64) -> Self {
        Self {
            state: if seed == 0 {
                0x853c_49e6_748f_ea9b
            } else {
                seed
            },
        }
    }

    pub fn from_entropy() -> Self {
        let mut z = SEED_COUNTER.fetch_add(0x9e37_79b9_7f4a_7c15, Ordering::Relaxed);
        z = (z ^ (z >> 30)).wrapping_mul(0xbf58_476d_1ce4_e5b9);
        z = (z ^ (z >> 27)).wrapping_mul(0x94d0_49bb_1331_11eb);
        Self::new(z ^ (z >> 31))
    }

    #[inline(always)]
    pub fn next_u64(&mut self) -> u64 {
        let mut x = self.state;
        x ^= x << 13;
        x ^= x >> 7;
        x ^= x << 17;
        self.state = x;
        x
    }

    #[inline(always)]
    pub fn random(&mut self) -> f32 {
        ((self.next_u64() >> 40) as f32) / ((1u64 << 24) as f32)
    }

    #[inline(always)]
    pub fn random_range(&mut self, min: f32, max: f32) -> f32 {
        min + (max - min) * self.random()
    }
}

impl Default for Rng {
    fn default() -> Self {
        Self::from_entropy()
    }
}

pub fn clear_screen() {
    print!("\x1b[2J");
    print!("\x1b[H");
    io::stdout().flush().unwrap();
}

pub fn hide_cursor() {
    print!("\x1b[?25l");
    io::stdout().flush().unwrap();
}

pub fn show_cursor() {
    print!("\x1b[?25h");
    io::stdout().flush().unwrap();
}

pub fn print_progress(
    done: usize,
    total: usize,
    x: usize,
    y: usize,
    sample: usize,
    samples: usize,
    start: Instant,
) {
    let percent = done * 100 / total.max(1);
    let elapsed = start.elapsed().as_secs_f32();
    let eta = if done > 0 {
        (total - done) as f32 * elapsed / done as f32
    } else {
        0.0
    };

    let bar_w = 30;
    let filled = done * bar_w / total.max(1);
    let bar = if filled >= bar_w {
        "=".repeat(bar_w)
    } else {
        "=".repeat(filled) + ">" + &" ".repeat(bar_w - filled - 1)
    };

    let elapsed_s = start.elapsed().as_secs();
    let eta_s = eta as u64;
    let eta_str = if done == 0 {
        "--".to_string()
    } else {
        format!("{}:{:02}", eta_s / 60, eta_s % 60)
    };

    let sample_str = format!(
        "[{:>width$}/{}]",
        sample,
        samples,
        width = samples.to_string().len(),
    );

    let line = format!(
        "[{}] {}% | ({},{}) {} | {}:{:02} / {}",
        bar,
        percent,
        x,
        y,
        sample_str,
        elapsed_s / 60,
        elapsed_s % 60,
        eta_str,
    );

    print!(
        "\r{}{}",
        line,
        " ".repeat(80usize.saturating_sub(line.len()))
    );
    io::stdout().flush().unwrap();
}

pub fn linear_to_gamma(linear: f32) -> f32 {
    if linear > 0.0 { linear.sqrt() } else { 0.0 }
}

pub fn to_u8(c: Color) -> (u8, u8, u8) {
    let intensity = Interval::new(0.000, 0.999);

    let r = linear_to_gamma(intensity.clamp(c.x));
    let g = linear_to_gamma(intensity.clamp(c.y));
    let b = linear_to_gamma(intensity.clamp(c.z));
    ((256.0 * r) as u8, (256.0 * g) as u8, (256.0 * b) as u8)
}
