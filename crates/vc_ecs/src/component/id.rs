use core::fmt;
use core::hash;

use vc_reflect::derive::Reflect;

// -----------------------------------------------------------------------------
// ComponentId

#[derive(Reflect, Copy, Clone, Ord, PartialOrd, Debug)]
#[reflect(mini, debug, hash, partial_eq)]
#[repr(transparent)]
pub struct ComponentId(u32);

impl ComponentId {
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

impl hash::Hash for ComponentId {
    #[inline(always)]
    fn hash<H: hash::Hasher>(&self, state: &mut H) {
        state.write_u32(self.0);
    }
}

impl fmt::Display for ComponentId {
    #[inline(always)]
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        fmt::Display::fmt(&self.0, f)
    }
}

impl PartialEq for ComponentId {
    #[inline(always)]
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}

impl Eq for ComponentId {}
