use crate::vec3::Color;

pub struct AccumBuffer(*mut Color);

unsafe impl Send for AccumBuffer {}
unsafe impl Sync for AccumBuffer {}

impl AccumBuffer {
    pub fn new(slice: &mut [Color]) -> Self {
        Self(slice.as_mut_ptr())
    }

    #[inline(always)]
    #[allow(dead_code)]
    pub unsafe fn add(&self, idx: usize) -> *mut Color {
        unsafe { self.0.add(idx) }
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
