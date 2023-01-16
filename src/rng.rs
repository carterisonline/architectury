use std::ops::Range;
use std::time::SystemTime;

use once_cell::sync::Lazy;
use oorandom::Rand32;

static mut RNG: Lazy<Rand32> = Lazy::new(|| {
    Rand32::new(
        (SystemTime::now()
            .duration_since(SystemTime::UNIX_EPOCH)
            .unwrap()
            .as_millis()
            & 0x0000_0000_0000_0000_FFFF_FFFF_FFFF_FFFF) as u64,
    )
});

/// Produces a random f32 in the range `[0.0, 1.0)]`.
pub fn rand_f32() -> f32 {
    unsafe { RNG.rand_float() }
}

/// Produces a random within the given bounds.  Like any `Range`,
/// it includes the lower bound and excludes the upper one.
///
/// This should be faster than `Self::rand() % end + start`, but the
/// real advantage is it's more convenient.  Requires that
/// `range.end <= range.start`.
pub fn rand_range_u32(r: Range<u32>) -> u32 {
    unsafe { RNG.rand_range(r) }
}

/// Produces a random bool, with a 1/2 chance of either false or true.
pub fn rand_bool() -> bool {
    unsafe { RNG.rand_float() >= 0.5 }
}
