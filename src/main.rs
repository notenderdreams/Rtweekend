mod image;
mod renderer;
mod utils;

use image::Image;

fn main() {
    let mut img = Image::new(256, 256);

    renderer::render(&mut img, gradient_shader);

    utils::clear_screen();
    println!("{}", img);
}

fn gradient_shader(x: usize, y: usize, w: usize, h: usize) -> (u8, u8, u8) {
    let r = (x * 255 / w) as u8;
    let g = (y * 255 / h) as u8;
    let b = 0;
    (r, g, b)
}
