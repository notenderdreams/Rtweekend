use std::{
    sync::atomic::{AtomicBool, AtomicUsize, Ordering},
    time::Instant,
};

use crate::{
    gui::Preview,
    renderer::{buffer::AccumBuffer, tile::Tile},
    utils::{Rng, print_progress, to_u8},
    vec3::Color,
};

#[inline(always)]
#[allow(clippy::too_many_arguments)]
pub fn render_tile<F>(
    tile: Tile,
    w: usize,
    s_start: usize,
    s_end: usize,
    spp: usize,
    rng: &mut Rng,
    shader: &F,
    accum_buf: &AccumBuffer,
    preview: Option<&Preview>,
    done: &AtomicUsize,
    total_samples: usize,
    update_every: usize,
    start: Instant,
    cancelled: &AtomicBool,
) where
    F: Fn(usize, usize, usize, &mut Rng) -> Color + Sync,
{
    for s in s_start..=s_end {
        let inv_s = 1.0 / s as f32;

        for y in tile.y0..tile.y1 {
            for x in tile.x0..tile.x1 {
                if cancelled.load(Ordering::Relaxed) {
                    return;
                }

                let idx = y * w + x;
                let px_color = shader(x, y, s, rng);

                let total_accum = unsafe { accum_buf.add_sample(idx, px_color) };

                let (r, g, b) = to_u8(total_accum * inv_s);
                if let Some(p) = preview {
                    p.set_pixel(x, y, r, g, b);
                }

                let d = done.fetch_add(1, Ordering::Relaxed) + 1;
                if d.is_multiple_of(update_every) {
                    print_progress(d, total_samples, x, y, s, spp, start);
                }
            }
        }
    }
}
