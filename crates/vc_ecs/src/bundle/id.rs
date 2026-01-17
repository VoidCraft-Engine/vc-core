use core::fmt;
use core::hash;

// -----------------------------------------------------------------------------
// BundleId

#[derive(Debug, Clone, Copy, Ord, PartialOrd)]
#[repr(transparent)]
pub struct BundleId(u32);

impl BundleId {
    #[inline(always)]
    pub const fn new(index: u32) -> Self {
        Self(index)
    }

    #[inline(always)]
    pub const fn index_u32(self) -> u32 {
        self.0
    }

    #[inline(always)]
    pub const fn index(self) -> usize {
        self.0 as usize
    }
}

impl PartialEq for BundleId {
    #[inline(always)]
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}

impl Eq for BundleId {}

impl hash::Hash for BundleId {
    #[inline(always)]
    fn hash<H: hash::Hasher>(&self, state: &mut H) {
        state.write_u32(self.0);
    }
}

impl fmt::Display for BundleId {
    #[inline(always)]
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        fmt::Display::fmt(&self.0, f)
    }
}
