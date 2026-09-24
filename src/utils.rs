use std::{
    io::{self, Write},
    time::Instant,
};

pub fn clear_screen() {
    print!("\x1b[2J"); // clear screen
    print!("\x1b[H"); // move cursor to 0,0
    io::stdout().flush().unwrap();
}

pub fn hide_cursor() {
    print!("\x1b[?25l")
}

pub fn show_cursor() {
    print!("\x1b[?25h")
}

pub fn print_progress(done: usize, total: usize, start: Instant) {
    let percent = done * 100 / total;
    let elapsed = start.elapsed().as_secs_f32();
    let pps = done as f32 / elapsed.max(0.001);
    let eta = (total - done) as f32 / pps.max(0.001);

    let bar_w = 40;
    let filled = percent * bar_w / 100;
    let bar = if filled >= bar_w {
        "=".repeat(bar_w)
    } else {
        "=".repeat(filled) + ">" + &" ".repeat(bar_w - filled - 1)
    };

    let elapsed_s = start.elapsed().as_secs();
    let eta_s = eta as u64;

    let line = format!(
        "\r[{}] {}% | {:.1}k px/s | {}:{:02} / {}:{:02}",
        bar,
        percent,
        pps / 1000.0,
        elapsed_s / 60,
        elapsed_s % 60,
        eta_s / 60,
        eta_s % 60
    );

    print!("\r{}{}", line, " ".repeat(80 - line.len().min(80)));
    io::stdout().flush().unwrap();
}
