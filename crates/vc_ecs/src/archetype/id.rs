use core::fmt;
use core::hash;
use core::num::NonZeroU32;

// -----------------------------------------------------------------------------
// ArchetypeId

#[derive(Clone, Copy, PartialOrd, Ord, Debug)]
#[repr(transparent)]
pub struct ArchetypeId(NonZeroU32);

impl ArchetypeId {
    #[inline(always)]
    pub const fn new(index: NonZeroU32) -> Self {
        Self(index)
    }

    #[inline(always)]
    pub const fn from_u32(index: u32) -> Self {
        Self(NonZeroU32::new(index).unwrap())
    }

    #[inline(always)]
    pub const fn index(self) -> u32 {
        self.0.get()
    }
}

impl fmt::Display for ArchetypeId {
    #[inline(always)]
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        fmt::Display::fmt(&self.0, f)
    }
}

impl hash::Hash for ArchetypeId {
    #[inline(always)]
    fn hash<H: hash::Hasher>(&self, state: &mut H) {
        state.write_u32(self.0.get());
    }
}

impl PartialEq for ArchetypeId {
    #[inline(always)]
    fn eq(&self, other: &Self) -> bool {
        self.0.get() == other.0.get()
    }
}

impl Eq for ArchetypeId {}
