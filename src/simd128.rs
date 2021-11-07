/// use SSE 2
#[cfg(any(target_arch = "x86_64"))]
use core::arch::x86_64::{__m128i, __m256i};

union Inner {
    mb: __m128i,
    norm: [u32; 4],
}

pub struct Simd128 {
    data: Inner,
}

impl From<[u32; 4]> for Simd128 {
    fn from(d: [u32; 4]) -> Self {
        Simd128 {
            data: Inner { norm: d },
        }
    }
}

impl From<Simd128> for [u32; 4] {
    fn from(it: Simd128) -> Self {
        unsafe { it.data.norm }
    }
}

impl Simd128 {
    pub fn get_m128(&self) -> __m128i {
        unsafe { self.data.mb }
    }

    pub fn inc_each(&self) {
        unsafe {}
    }
}
