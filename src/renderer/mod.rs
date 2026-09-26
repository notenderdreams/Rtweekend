use std::sync::atomic::{AtomicBool, AtomicUsize, Ordering};
use std::thread;
use std::time::Instant;

use crate::gui::Preview;
use crate::image::Image;
use crate::utils::*;
use crate::vec3::Color;

mod buffer;
mod tile;
mod worker;

use buffer::AccumBuffer;
pub use tile::Tile;
use worker::render_tile;

pub const TILE_SIZE: usize = 32;

pub fn render<F>(
    img: &mut Image,
    samples_per_pixel: usize,
    preview: Option<&Preview>,
    shader: F,
) -> bool
where
    F: Fn(usize, usize, usize, &mut Rng) -> Color + Sync,
{
    let w = img.w();
    let h = img.h();
    let total_samples = w * h * samples_per_pixel;
    let update_every = (total_samples / 500).max(1);

    let tiles = Tile::generate(w, h, TILE_SIZE, true);
    let tile_count = tiles.len();

    let mut accum = vec![Color::zero(); w * h];
    let accum_buf = AccumBuffer::new(&mut accum);

    let thread_count = thread::available_parallelism()
        .map(|n| n.get())
        .unwrap_or(1);

    let done = AtomicUsize::new(0);
    let cancelled = AtomicBool::new(false);
    let start = Instant::now();

    hide_cursor();

    'passes: for s in 1..=samples_per_pixel {
        let next_tile = AtomicUsize::new(0);

        thread::scope(|scope| {
            for _ in 0..thread_count {
                scope.spawn(|| {
                    let mut rng = Rng::default();
                    loop {
                        if cancelled.load(Ordering::Relaxed) {
                            break;
                        }

                        if preview.is_some_and(|p| {
                            p.abort.load(Ordering::Relaxed) || p.restart.load(Ordering::Relaxed)
                        }) {
                            cancelled.store(true, Ordering::Relaxed);
                            break;
                        }

                        let tile_idx = next_tile.fetch_add(1, Ordering::Relaxed);
                        if tile_idx >= tile_count {
                            break;
                        }

                        let tile = tiles[tile_idx];

                        render_tile(
                            tile,
                            w,
                            s,
                            samples_per_pixel,
                            &mut rng,
                            &shader,
                            &accum_buf,
                            preview,
                            &done,
                            total_samples,
                            update_every,
                            start,
                            &cancelled,
                        );
                    }
                });
            }
        });

        if cancelled.load(Ordering::Relaxed) {
            break 'passes;
        }
    }

    show_cursor();

    let completed = !cancelled.load(Ordering::Relaxed);

    let inv_samples = 1.0 / samples_per_pixel as f32;
    for y in 0..h {
        for x in 0..w {
            let idx = y * w + x;
            let color = unsafe { accum_buf.get(idx) };
            let (r, g, b) = to_u8(color * inv_samples);
            img.set_px(x, y, r, g, b);
        }
    }

    if completed {
        let d = done.load(Ordering::Relaxed);
        print_progress(
            d,
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
