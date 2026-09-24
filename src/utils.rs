use std::{
    io::{self, Write},
    sync::atomic::{AtomicU64, Ordering},
    time::Instant,
};

use crate::{interval::Interval, vec3::Color};

// xorshift64
static SEED: AtomicU64 = AtomicU64::new(0x853c_49e6_748f_ea9b);

pub fn random() -> f32 {
    let mut x = SEED.load(Ordering::Relaxed);
    x ^= x << 13;
    x ^= x >> 7;
    x ^= x << 17;
    SEED.store(x, Ordering::Relaxed);
    ((x >> 40) as f32) / ((1u64 << 24) as f32)
}

pub fn random_range(min: f32, max: f32) -> f32 {
    min + (max - min) * random()
}

pub fn clear_screen() {
    print!("\x1b[2J");
    print!("\x1b[H");
    io::stdout().flush().unwrap();
}

pub fn hide_cursor() {
    print!("\x1b[?25l")
}

pub fn show_cursor() {
    print!("\x1b[?25h")
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

    print!("\r{}{}", line, " ".repeat(80usize.saturating_sub(line.len())));
    io::stdout().flush().unwrap();
}

pub fn to_u8(c: Color) -> (u8, u8, u8) {
    let intensity = Interval::new(0.000, 0.999);
    (
        (256.0 * intensity.clamp(c.x)) as u8,
        (256.0 * intensity.clamp(c.y)) as u8,
        (256.0 * intensity.clamp(c.z)) as u8,
    )
}
