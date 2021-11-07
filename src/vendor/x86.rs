#[cfg(any(target_arch = "x86"))]
use core::arch::x86::*;

#[cfg(target_arch = "x86_64")]
use core::arch::x86_64::*;

macro_rules! simd_layout {
    ($name: ident, $ty: ty, $num: literal) => {
        union $name {
            mm: $ty,
            m8: [u8; $num / 8],
            m8i: [i8; $num / 8],
            m16: [u16; $num / 16],
            m16i: [i16; $num / 16],
            m32: [u32; $num / 32],
            m32i: [i32; $num / 32],
            m64: [u64; $num / 64],
            m64i: [i64; $num / 64],
        }
    };
}

simd_layout!(SimdX86Layout128, __m128i, 128);

simd_layout!(SimdX86Layout256, __m256i, 256);

// TODO(CGQAQ): Uncomment this line to enable 512bit SIMD as soon as it becomes stable.
// simd_layout!(SimdX86Layout512, __m512i, 128);
