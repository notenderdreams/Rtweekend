use std::{io::BufReader, sync::Arc};

use zune_jpeg::JpegDecoder;

use crate::{perlin::Perlin, vec3::{Color, Point}};

pub trait Texture: Send + Sync {
    fn value(&self, u: f32, v: f32, p: &Point) -> Color;
}

pub struct SolidColor {
    albedo: Color,
}

impl SolidColor {
    pub fn new(albedo: Color) -> Self {
        Self { albedo }
    }

    pub fn from_rgb(r: f32, g: f32, b: f32) -> Self {
        Self::new(Color::new(r, g, b))
    }
}

impl Texture for SolidColor {
    fn value(&self, _u: f32, _v: f32, _p: &Point) -> Color {
        self.albedo
    }
}

pub struct CheckerTexture {
    inv_scale: f32,
    even: Arc<dyn Texture>,
    odd: Arc<dyn Texture>,
}

impl CheckerTexture {
    pub fn new(scale: f32, even: Arc<dyn Texture>, odd: Arc<dyn Texture>) -> Self {
        Self {
            inv_scale: 1.0 / scale,
            even,
            odd,
        }
    }

    pub fn from_colors(scale: f32, even: Color, odd: Color) -> Self {
        Self::new(
            scale,
            Arc::new(SolidColor::new(even)),
            Arc::new(SolidColor::new(odd)),
        )
    }
}

impl Texture for CheckerTexture {
    fn value(&self, u: f32, v: f32, p: &Point) -> Color {
        let x_int = (self.inv_scale * p.x).floor() as i32;
        let y_int = (self.inv_scale * p.y).floor() as i32;
        let z_int = (self.inv_scale * p.z).floor() as i32;

        // rem_euclid to correctly handle negative coordinate remainders
        let is_even = (x_int + y_int + z_int).rem_euclid(2) == 0;
        if is_even {
            self.even.value(u, v, p)
        } else {
            self.odd.value(u, v, p)
        }
    }
}

pub struct RtwImage {
    width: usize,
    height: usize,
    bytes_per_scanline: usize,
    data: Vec<u8>,
}

impl RtwImage {
    pub fn new(path: &str) -> Self {
        let load = || -> Result<(usize, usize, Vec<u8>), ()> {
            let file = std::fs::File::open(path).map_err(|_| ())?;
            let mut decoder = JpegDecoder::new(BufReader::new(file));
            let pixels = decoder.decode().map_err(|_| ())?;
            let info = decoder.info().ok_or(())?;
            let (w, h) = (info.width as usize, info.height as usize);
            Ok((w, h, pixels))
        };

        if let Ok((width, height, data)) = load() {
            return Self {
                width,
                height,
                bytes_per_scanline: width * 3,
                data,
            };
        }

        eprintln!("ERROR: Could not load image file '{}'.", path);
        Self {
            width: 0,
            height: 0,
            bytes_per_scanline: 0,
            data: Vec::new(),
        }
    }
    pub fn width(&self) -> usize {
        self.width
    }
    pub fn height(&self) -> usize {
        self.height
    }

    pub fn pixel_data(&self, mut x: usize, mut y: usize) -> (u8, u8, u8) {
        if self.data.is_empty() {
            return (255, 0, 255);
        }
        x = x.clamp(0, self.width.saturating_sub(1));
        y = y.clamp(0, self.height.saturating_sub(1));
        let index = y * self.bytes_per_scanline + x * 3;
        (self.data[index], self.data[index + 1], self.data[index + 2])
    }
}

pub struct ImageTexture {
    image: RtwImage,
}

impl ImageTexture {
    pub fn new(path: &str) -> Self {
        Self {
            image: RtwImage::new(path),
        }
    }
}

impl Texture for ImageTexture {
    fn value(&self, u: f32, v: f32, _p: &Point) -> Color {
        if self.image.height() == 0 {
            return Color::new(0.0, 1.0, 1.0);
        }

        let u = u.clamp(0.0, 1.0);
        let v = 1.0 - v.clamp(0.0, 1.0); // Flip v to match image coordinates

        let mut i = (u * self.image.width() as f32) as usize;
        let mut j = (v * self.image.height() as f32) as usize;

        if i >= self.image.width() {
            i = self.image.width() - 1;
        }
        if j >= self.image.height() {
            j = self.image.height() - 1;
        }
        let (r, g, b) = self.image.pixel_data(i, j);
        let color_scale = 1.0 / 255.0;
        Color::new(
            r as f32 * color_scale,
            g as f32 * color_scale,
            b as f32 * color_scale,
        )
    }
}

pub struct NoiseTexture {
    noise: Perlin,
    scale: f32,
}

impl NoiseTexture {
    pub fn new(scale: f32) -> Self {
        Self {
            noise: Perlin::new(),
            scale,
        }
    }
}

impl Default for NoiseTexture {
    fn default() -> Self {
        Self::new(1.0)
    }
}

impl Texture for NoiseTexture {
    fn value(&self, _u: f32, _v: f32, p: &Point) -> Color {
        Color::new(0.5, 0.5, 0.5) * (1.0 + (self.scale * p.z + 10.0 * self.noise.turb(p, 7)).sin())
    }
}
