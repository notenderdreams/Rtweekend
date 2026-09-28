use crate::vec3::Color;

/// A lock-free accumulation buffer for multithreaded ray tracing
//
/// Each sample is added to the pixel's running total:
///   accum[idx] += sample
///   preview    = accum[idx] / samples
///
/// The image is split into non-overlapping tiles.
///             ┌────────────┬────────────┐
///             │   Tile 0   │   Tile 1   │
///             │ (Thread 1) │ (Thread 2) │
///             ├────────────┼────────────┤
///             │   Tile 2   │   Tile 3   │
///             │ (Thread 3) │ (Thread 4) │
///             └────────────┴────────────┘
/// Tiles never overlap, so threads never touch the same pixel.
pub struct AccumBuffer(*mut Color);

unsafe impl Send for AccumBuffer {}
unsafe impl Sync for AccumBuffer {}

impl AccumBuffer {
    /// Stores a pointer to the first pixel of `slice`.
    /// The lifetime of `slice` must be longer than that of the `AccumBuffer`.
    pub fn new(slice: &mut [Color]) -> Self {
        Self(slice.as_mut_ptr())
    }

    #[inline(always)]
    pub unsafe fn get(&self, idx: usize) -> Color {
        unsafe { *self.0.add(idx) }
    }

    #[inline(always)]
    pub unsafe fn add_sample(&self, idx: usize, sample: Color) -> Color {
        unsafe {
            let ptr = self.0.add(idx);
            let sum = *ptr + sample;
            *ptr = sum;
            sum
        }
    }
}
