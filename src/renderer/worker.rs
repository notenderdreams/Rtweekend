use std::sync::atomic::{AtomicBool, Ordering};

use crate::{
    gui::Preview,
    renderer::{buffer::AccumBuffer, tile::Tile},
    utils::{Rng, to_u8},
    vec3::Color,
};

const MARKER_COLOR: (u8, u8, u8) = (180, 0, 0);

#[inline(always)]
#[allow(clippy::too_many_arguments)]
pub fn render_tile<F>(
    tile: Tile,
    w: usize,
    s_start: usize,
    s_end: usize,
    rng: &mut Rng,
    shader: &F,
    accum_buf: &AccumBuffer,
    preview: Option<&Preview>,
    cancelled: &AtomicBool,
) where
    F: Fn(usize, usize, usize, &mut Rng) -> Color + Sync,
{
    let show_marker =
        preview.is_some() && (s_end > s_start) && tile.width() >= 8 && tile.height() >= 8;

    if show_marker && let Some(p) = preview {
        tile.for_each_corner_pixel(|x, y| {
            p.set_pixel(x, y, MARKER_COLOR.0, MARKER_COLOR.1, MARKER_COLOR.2);
        });
    }

    // Records how many samples have been fully finished across this tile:
    //
    // Why `s_start - 1`:
    //    * Stage 1 starts at sample 1 -> 0 samples were done beforehand (1 - 1 = 0).
    //    * Stage 2 starts at sample 2 -> 1 sample was already finished in Stage 1 (2 - 1 = 1).
    //
    // Why we need it:
    //      If rendering finishes or gets cancelled midway through a sample, this gives
    //      us the last valid sample count to average and erase the red corner markers
    //      without dividing by zero.
    let mut last_complete_sample = s_start.saturating_sub(1);

    'outer: for s in s_start..=s_end {
        let inv_s = 1.0 / s as f32;

        for y in tile.y0..tile.y1 {
            for x in tile.x0..tile.x1 {
                if cancelled.load(Ordering::Relaxed) {
                    break 'outer;
                }

                let idx = y * w + x;
                let px_color = shader(x, y, s, rng);

                let total_accum = unsafe { accum_buf.add_sample(idx, px_color) };

                if let Some(p) = preview
                    && (!show_marker || !tile.is_corner(x, y))
                {
                    let (r, g, b) = to_u8(total_accum * inv_s);
                    p.set_pixel(x, y, r, g, b);
                }
            }
        }
        last_complete_sample = s;
    }

    if show_marker
        && let Some(p) = preview
        && last_complete_sample > 0
    {
        let inv_s = 1.0 / last_complete_sample as f32;
        tile.for_each_corner_pixel(|x, y| {
            let idx = y * w + x;
            let total_accum = unsafe { accum_buf.get(idx) };
            let (r, g, b) = to_u8(total_accum * inv_s);
            p.set_pixel(x, y, r, g, b);
        });
    }
}
