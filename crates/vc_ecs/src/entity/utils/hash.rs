use core::hash::{BuildHasher, Hasher};

/// A [`BuildHasher`] that results in a [`EntityHasher`].
#[derive(Debug, Default, Clone)]
pub struct EntityHash;

impl BuildHasher for EntityHash {
    type Hasher = EntityHasher;
    #[inline(always)]
    fn build_hasher(&self) -> Self::Hasher {
        Self::Hasher::default()
    }
}

/// A very fast hash that is only designed to work on generational indices
/// like [`Entity`](super::Entity). It will panic if attempting to hash a type containing
/// non-u64 fields.
///
/// This is heavily optimized for typical cases, where you have mostly live
/// entities, and works particularly well for contiguous indices.
///
/// If you have an unusual case -- say all your indices are multiples of 256
/// or most of the entities are dead generations -- then you might want also to
/// try [`DefaultHasher`](bevy_platform::hash::DefaultHasher) for a slower hash
/// computation but fewer lookup conflicts.
#[derive(Debug, Default)]
pub struct EntityHasher {
    hash: u64,
}

impl Hasher for EntityHasher {
    #[inline]
    fn finish(&self) -> u64 {
        self.hash
    }

    fn write(&mut self, _bytes: &[u8]) {
        panic!("EntityHasher can only hash u64 fields.");
    }

    #[inline]
    fn write_u64(&mut self, bits: u64) {
        const UPPER_PHI: u64 = 0x9e37_79b9_0000_0001;

        self.hash = bits.wrapping_mul(UPPER_PHI);
    }
}
