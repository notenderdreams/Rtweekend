#[derive(Debug, Copy, Clone)]
pub struct Tile {
    pub x0: usize,
    pub x1: usize,
    pub y0: usize,
    pub y1: usize,
}

impl Tile {
    pub fn new(x0: usize, x1: usize, y0: usize, y1: usize) -> Self {
        Self { x0, x1, y0, y1 }
    }

    pub fn generate(w: usize, h: usize, tile_size: usize, center_out: bool) -> Vec<Tile> {
        let mut tiles = Vec::new();

        for y in (0..h).step_by(tile_size) {
            for x in (0..w).step_by(tile_size) {
                tiles.push(Self::new(
                    x,
                    (x + tile_size).min(w),
                    y,
                    (y + tile_size).min(h),
                ));
            }
        }

        if center_out {
            let cx = (w / 2) as f32;
            let cy = (h / 2) as f32;

            tiles.sort_by(|a, b| {
                let a_cx = (a.x0 + a.x1) as f32 * 0.5;
                let a_cy = (a.y0 + a.y1) as f32 * 0.5;
                let b_cx = (b.x0 + b.x1) as f32 * 0.5;
                let b_cy = (b.y0 + b.y1) as f32 * 0.5;

                let dist_a = (a_cx - cx).powi(2) + (a_cy - cy).powi(2);
                let dist_b = (b_cx - cx).powi(2) + (b_cy - cy).powi(2);

                dist_a.partial_cmp(&dist_b).unwrap()
            });
        }

        tiles
    }
}
