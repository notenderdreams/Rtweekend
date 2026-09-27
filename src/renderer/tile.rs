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

    #[inline(always)]
    pub fn width(&self) -> usize {
        self.x1 - self.x0
    }

    #[inline(always)]
    pub fn height(&self) -> usize {
        self.y1 - self.y0
    }

    #[inline(always)]
    pub fn corner_arm(&self) -> usize {
        (self.width().min(self.height()) / 5).clamp(3, 7)
    }

    #[inline(always)]
    pub fn is_corner(&self, x: usize, y: usize) -> bool {
        let x_min = self.x0;
        let x_max = self.x1.saturating_sub(1);
        let y_min = self.y0;
        let y_max = self.y1.saturating_sub(1);

        if y > y_min && y < y_max && x > x_min && x < x_max {
            return false;
        }

        let arm = self.corner_arm();

        let on_top = y == y_min;
        let on_bottom = y == y_max;
        let on_left = x == x_min;
        let on_right = x == x_max;

        let in_left = x < x_min + arm;
        let in_right = x + arm > x_max;
        let in_top = y < y_min + arm;
        let in_bottom = y + arm > y_max;

        (on_top && (in_left || in_right))
            || (on_bottom && (in_left || in_right))
            || (on_left && (in_top || in_bottom))
            || (on_right && (in_top || in_bottom))
    }

    pub fn for_each_corner_pixel<F: FnMut(usize, usize)>(&self, mut f: F) {
        let arm = self.corner_arm();
        let x_min = self.x0;
        let x_max = self.x1 - 1;
        let y_min = self.y0;
        let y_max = self.y1 - 1;

        let x_right_start = x_max + 1 - arm;
        let y_bottom_start = y_max + 1 - arm;

        // Top-left
        for x in x_min..x_min + arm {
            f(x, y_min);
        }
        for y in (y_min + 1)..(y_min + arm) {
            f(x_min, y);
        }

        // Top-right
        for x in x_right_start..=x_max {
            f(x, y_min);
        }
        for y in (y_min + 1)..(y_min + arm) {
            f(x_max, y);
        }

        // Bottom-left
        for x in x_min..x_min + arm {
            f(x, y_max);
        }
        for y in y_bottom_start..y_max {
            f(x_min, y);
        }

        // Bottom-right
        for x in x_right_start..=x_max {
            f(x, y_max);
        }
        for y in y_bottom_start..y_max {
            f(x_max, y);
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_tile_corner_consistency() {
        for &(w, h) in &[(32, 32), (16, 16), (24, 32), (8, 8)] {
            let tile = Tile::new(0, w, 0, h);
            let arm = tile.corner_arm();

            let mut visited = std::collections::HashSet::new();
            tile.for_each_corner_pixel(|x, y| {
                assert!(x < w && y < h, "pixel ({x}, {y}) out of tile bounds");
                assert!(
                    tile.is_corner(x, y),
                    "for_each yielded ({x}, {y}) but is_corner returned false"
                );
                assert!(visited.insert((x, y)), "duplicate pixel ({x}, {y}) visited");
            });

            for y in 0..h {
                for x in 0..w {
                    if tile.is_corner(x, y) {
                        assert!(
                            visited.contains(&(x, y)),
                            "is_corner true for ({x}, {y}) but for_each did not yield it"
                        );
                    }
                }
            }

            assert_eq!(visited.len(), 4 * (2 * arm - 1));
        }
    }
}
