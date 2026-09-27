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

    let tiles = Tile::generate(w, h, TILE_SIZE, true);

    let mut accum = vec![Color::zero(); w * h];
    let accum_buf = AccumBuffer::new(&mut accum);

    let thread_count = thread::available_parallelism()
        .map(|n| n.get())
        .unwrap_or(1);

    let tiles_done = AtomicUsize::new(0);
    let cancelled = AtomicBool::new(false);
    let start = Instant::now();

    hide_cursor();

    // Stage 1: Full-scene initial draft (1 sample across all tiles)
    let stage1_progress = if samples_per_pixel == 1 {
        Some((&tiles_done, tiles.len()))
    } else {
        None
    };
    dispatch_tiles(
        &tiles,
        w,
        1,
        1,
        thread_count,
        &shader,
        &accum_buf,
        preview,
        stage1_progress,
        start,
        &cancelled,
    );

    // Stage 2: Tiled cleanup (samples 2..=spp per tile, center-out)
    if samples_per_pixel > 1 && !cancelled.load(Ordering::Relaxed) {
        dispatch_tiles(
            &tiles,
            w,
            2,
            samples_per_pixel,
            thread_count,
            &shader,
            &accum_buf,
            preview,
            Some((&tiles_done, tiles.len())),
            start,
            &cancelled,
        );
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
        let msg = format!("Done in {:.2}s", start.elapsed().as_secs_f32());
        println!("\r{}{}", msg, " ".repeat(85usize.saturating_sub(msg.len())));
    }

    completed
}

#[allow(clippy::too_many_arguments)]
fn dispatch_tiles<F>(
    tiles: &[Tile],
    w: usize,
    s_start: usize,
    s_end: usize,
    thread_count: usize,
    shader: &F,
    accum_buf: &AccumBuffer,
    preview: Option<&Preview>,
    progress: Option<(&AtomicUsize, usize)>,
    start: Instant,
    cancelled: &AtomicBool,
) where
    F: Fn(usize, usize, usize, &mut Rng) -> Color + Sync,
{
    let tile_count = tiles.len();
    let next_tile = AtomicUsize::new(0);
    let active_threads = AtomicUsize::new(0);

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

                    active_threads.fetch_add(1, Ordering::Relaxed);
                    let tile = tiles[tile_idx];
                    render_tile(
                        tile,
                        w,
                        s_start,
                        s_end,
                        &mut rng,
                        shader,
                        accum_buf,
                        preview,
                        cancelled,
                    );
                    let active = active_threads.fetch_sub(1, Ordering::Relaxed);

                    if let Some((tiles_done, total_tiles)) = progress {
                        let d = tiles_done.fetch_add(1, Ordering::Relaxed) + 1;
                        print_progress(d, total_tiles, active, start);
                    }
                }
            });
        }
    });
}
