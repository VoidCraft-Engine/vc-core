use core::fmt;
use core::hash;
use core::num::NonZeroU32;

// -----------------------------------------------------------------------------
// EntityId

#[derive(Clone, Copy, PartialOrd, Ord, Debug)]
#[repr(transparent)]
pub struct EntityId(NonZeroU32);

impl EntityId {
    pub const PLACEHOLDER: Self = Self(NonZeroU32::MAX);

    #[inline(always)]
    pub const fn new(index: NonZeroU32) -> Self {
        Self(index)
    }

    #[inline(always)]
    pub const fn from_u32(index: u32) -> Self {
        Self(NonZeroU32::new(index).unwrap())
    }

    #[inline(always)]
    pub const fn index_u32(self) -> u32 {
        self.0.get()
    }

    #[inline(always)]
    pub const fn index(self) -> usize {
        self.0.get() as usize
    }
}

impl fmt::Display for EntityId {
    #[inline(always)]
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        fmt::Display::fmt(&self.0, f)
    }
}

impl hash::Hash for EntityId {
    #[inline(always)]
    fn hash<H: hash::Hasher>(&self, state: &mut H) {
        state.write_u32(self.0.get());
    }
}

impl PartialEq for EntityId {
    #[inline(always)]
    fn eq(&self, other: &Self) -> bool {
        self.0.get() == other.0.get()
    }
}

impl Eq for EntityId {}

// -----------------------------------------------------------------------------
// EntityGeneration

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
#[repr(transparent)]
pub struct EntityGeneration(u32);

impl EntityGeneration {
    /// Represents the first generation of an [`EntityIndex`].
    pub const FIRST: Self = Self(0);

    /// Non-wrapping difference between two generations after which a signed interpretation becomes negative.
    const DIFF_MAX: u32 = 1u32 << 31;

    #[inline]
    pub const fn after(self, versions: u32) -> Self {
        Self(self.0.wrapping_add(versions))
    }

    #[inline]
    pub const fn after_check_alias(self, versions: u32) -> (Self, bool) {
        let raw = self.0.overflowing_add(versions);
        (Self(raw.0), raw.1)
    }

    #[inline]
    pub const fn cmp_approx(&self, other: &Self) -> core::cmp::Ordering {
        use core::cmp::Ordering;
        match self.0.wrapping_sub(other.0) {
            0 => Ordering::Equal,
            1..Self::DIFF_MAX => Ordering::Greater,
            _ => Ordering::Less,
        }
    }
}

impl fmt::Display for EntityGeneration {
    #[inline(always)]
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        fmt::Display::fmt(&self.0, f)
    }
}

impl hash::Hash for EntityGeneration {
    #[inline(always)]
    fn hash<H: hash::Hasher>(&self, state: &mut H) {
        state.write_u32(self.0);
    }
}
