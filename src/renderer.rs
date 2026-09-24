use std::time::Instant;

use crate::image::Image;
use crate::utils::*;
use crate::vec3::Color;

pub fn render<F>(img: &mut Image, samples_per_pixel: usize, mut shader: F)
where
    F: FnMut(usize, usize, usize) -> Color,
{
    let w = img.w();
    let h = img.h();
    let total_samples = w * h * samples_per_pixel;
    let inv_samples = 1.0 / samples_per_pixel as f32;

    let update_every = (total_samples / 500).max(1);

    let mut done = 0usize;
    let start = Instant::now();
    hide_cursor();

    for y in 0..h {
        for x in 0..w {
            let mut color = Color::zero();
            for s in 0..samples_per_pixel {
                color = color + shader(x, y, s);
                done += 1;
                if done % update_every == 0 {
                    print_progress(done, total_samples, x, y, s + 1, samples_per_pixel, start);
                }
            }
            let (r, g, b) = to_u8(color * inv_samples);
            img.set_px(x, y, r, g, b);
        }
    }

    print_progress(done, total_samples, w - 1, h - 1, samples_per_pixel, samples_per_pixel, start);
    println!("\nDone in {:.2}s", start.elapsed().as_secs_f32());
    show_cursor();
}
