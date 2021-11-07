#[cfg(any(target_arch = "x86"))]
use core::arch::x86::*;

#[cfg(target_arch = "x86_64")]
use core::arch::x86_64::*;

union SimdX86Layout128 {
    mm: __m128i,
}
