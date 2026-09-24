use std::time::Instant;

use crate::image::Image;
use crate::utils::*;

pub fn render<F>(img: &mut Image, mut shader: F)
where
    F: FnMut(usize, usize, usize, usize) -> (u8, u8, u8),
{
    let total = img.w() * img.h();
    let mut done = 0;
    let start = Instant::now();
    hide_cursor();

    for y in 0..img.h() {
        for x in 0..img.w() {
            let (r, g, b) = shader(x, y, img.w(), img.h());
            img.set_px(x, y, r, g, b);
            done += 1;
            if done % 400 == 0 {
                print_progress(done, total, start);
            }
        }
    }
    print_progress(done, total, start);
    println!("\nDone in {:.2}s", start.elapsed().as_secs_f32());
    show_cursor();
}
