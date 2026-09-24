use std::fmt;

pub struct Image {
    w: usize,
    h: usize,
    px: Vec<u8>,
}

impl Image {
    pub fn new(w: usize, h: usize) -> Self {
        Self {
            w,
            h,
            px: vec![0u8; w * h * 3],
        }
    }

    fn get_index(&self, x: usize, y: usize) -> usize {
        (y * self.w + x) * 3
    }

    pub fn set_px(&mut self, x: usize, y: usize, r: u8, g: u8, b: u8) {
        let i = self.get_index(x, y);
        self.px[i] = r;
        self.px[i + 1] = g;
        self.px[i + 2] = b;
    }

    pub fn get_px(&self, x: usize, y: usize) -> (u8, u8, u8) {
        let i = self.get_index(x, y);
        (self.px[i], self.px[i + 1], self.px[i + 2])
    }

    pub fn h(&self) -> usize {
        self.h
    }
    pub fn w(&self) -> usize {
        self.w
    }
}

impl fmt::Display for Image {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let h = self.h + self.h % 2;

        for y in (0..h).step_by(2) {
            for x in 0..self.w {
                let (r1, g1, b1) = if y < self.h {
                    self.get_px(x, y)
                } else {
                    (0, 0, 0)
                };
                let (r2, g2, b2) = if y + 1 < self.h {
                    self.get_px(x, y + 1)
                } else {
                    (0, 0, 0)
                };

                write!(
                    f,
                    "\x1b[38;2;{};{};{}m\x1b[48;2;{};{};{}m\u{2580}",
                    r1, g1, b1, r2, g2, b2
                )?;
            }
            writeln!(f, "\x1b[0m")?;
        }
        Ok(())
    }
}
