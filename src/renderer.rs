use std::sync::atomic::Ordering;
use std::time::Instant;

use crate::gui::Preview;
use crate::image::Image;
use crate::utils::*;
use crate::vec3::Color;

pub fn render<F>(
    img: &mut Image,
    samples_per_pixel: usize,
    preview: Option<&Preview>,
    mut shader: F,
) -> bool
where
    F: FnMut(usize, usize, usize, &mut Rng) -> Color,
{
    let w = img.w();
    let h = img.h();
    let total_samples = w * h * samples_per_pixel;
    let update_every = (total_samples / 500).max(1);

    let mut accum = vec![Color::zero(); w * h];
    let mut done = 0usize;
    let mut rng = Rng::default();
    let start = Instant::now();
    hide_cursor();

    let mut completed = true;

    'outer: for s in 1..=samples_per_pixel {
        let inv_s = 1.0 / s as f32;

        for y in 0..h {
            for x in 0..w {
                if let Some(p) = preview {
                    if p.abort.load(Ordering::Relaxed) || p.restart.load(Ordering::Relaxed) {
                        completed = false;
                        break 'outer;
                    }
                }

                let idx = y * w + x;
                accum[idx] = accum[idx] + shader(x, y, s, &mut rng);
                done += 1;

                if done % update_every == 0 {
                    print_progress(done, total_samples, x, y, s, samples_per_pixel, start);
                }

                let (r, g, b) = to_u8(accum[idx] * inv_s);
                img.set_px(x, y, r, g, b);
                if let Some(p) = preview {
                    p.set_pixel(x, y, r, g, b);
                }
            }
        }
    }

    show_cursor();

    if completed {
        print_progress(
            done,
            total_samples,
            w.saturating_sub(1),
            h.saturating_sub(1),
            samples_per_pixel,
            samples_per_pixel,
            start,
        );
        println!("\nDone in {:.2}s", start.elapsed().as_secs_f32());
    }

    completed
}
