use core::{
    fmt::Debug,
    hash::{BuildHasher, Hasher},
};

use foldhash::fast::{FixedState, FoldHasher};

/// A fixed hash seed(randomly generated)
///
/// Internally, `with_seed` will also XOR with another fixed seed once to obtain the final fixed seed.
const FIXED_HASH_STATE: FixedState = FixedState::with_seed(0x95EE04C4F326B271);

/// See [`foldhash::fast::FoldHasher`]
pub type FixedHasher = FoldHasher<'static>;

/// ### Fixed Hash State based upon a random but fixed seed.
///
/// Internally used [`foldhash::fast::FixedState`], but changed the fixed seed.
#[derive(Copy, Clone, Default, Debug)]
pub struct FixedHashState;

impl BuildHasher for FixedHashState {
    type Hasher = FixedHasher;

    #[inline(always)]
    fn build_hasher(&self) -> Self::Hasher {
        FIXED_HASH_STATE.build_hasher()
    }
}

/// ### A no-op hash that only works on `u64`s.
///
/// See [`NoOpHashState`]
#[derive(Copy, Clone, Default, Debug)]
pub struct NoOpHasher {
    hash: u64,
}

impl Hasher for NoOpHasher {
    #[inline]
    fn finish(&self) -> u64 {
        self.hash
    }

    fn write(&mut self, bytes: &[u8]) {
        // Usually recommended to use `write_u64` directly
        for byte in bytes.iter().rev() {
            self.hash = self.hash.rotate_left(8).wrapping_add(*byte as u64);
        }
    }

    #[inline]
    fn write_u64(&mut self, i: u64) {
        self.hash = i;
    }
}

/// ### A fixed hasher without any additional operations.
///
/// Only storing one `u64` and assigning values directly when `writa_u64` (recommended to only use this method).
///
/// Other method will call `write`, which will add the input bytes in reverse order to `u64`, and make it rotate left.
/// Ensure that the results of `write_u64(1234)` and `write_i32(1234)` are the same **if only called once**.
#[derive(Copy, Clone, Default, Debug)]
pub struct NoOpHashState;

impl BuildHasher for NoOpHashState {
    type Hasher = NoOpHasher;

    #[inline]
    fn build_hasher(&self) -> Self::Hasher {
        // manually inline
        NoOpHasher { hash: 0 }
    }
}
