#![expect(unsafe_code, reason = "unchecked non-zero is unsafe.")]

use core::fmt;
use core::hash;
use core::num::NonZeroU32;

// -----------------------------------------------------------------------------
// EntityId

/// This represents the index of an [`Entity`] within the [`Entities`] array.
///
/// This is a unique identifier for an entity in the world, a lighter weight
/// version of [`Entity`].
///
/// This differs from [`Entity`] in that [`Entity`] is unique for all entities
/// total (unless the [`EntityGeneration`] wraps), but this is only unique for
/// entities that are active.
///
/// The valid range is `1..u32::MAX`, not including `u32::MAX`.
///
/// [`Entity`]: crate::entity::Entity
/// [`Entities`]: crate::entity::Entities
#[derive(Debug, Clone, Copy, PartialOrd, Ord)]
#[repr(transparent)]
pub struct EntityId(NonZeroU32);

impl EntityId {
    const _STATIC_ASSERT_: () = const {
        const VAL: u32 = 2026;
        const ID: EntityId = unsafe { core::mem::transmute(VAL) };
        assert!(VAL == ID.0.get());
        assert!(VAL == ID.index_u32());
    };

    pub const PLACEHOLDER: Self = Self(NonZeroU32::MAX);

    /// Constructs a new [`EntityId`] from its index.
    #[inline(always)]
    pub const fn new(index: NonZeroU32) -> Self {
        Self(index)
    }

    /// Equivalent to [`new`](Self::new) except it's non-zero.
    ///
    /// # Panic
    /// Panic if `index == 0` .
    #[inline(always)]
    pub const fn from_u32(index: u32) -> Self {
        Self(NonZeroU32::new(index).unwrap())
    }

    /// Gets the index of the entity.
    #[inline(always)]
    pub const fn index_u32(self) -> u32 {
        unsafe { core::mem::transmute(self) }
    }

    /// Gets the index of the entity.
    #[inline(always)]
    pub const fn index(self) -> usize {
        self.index_u32() as usize
    }
}

impl PartialEq for EntityId {
    #[inline(always)]
    fn eq(&self, other: &Self) -> bool {
        self.index_u32() == other.index_u32()
    }
}

impl Eq for EntityId {}

impl hash::Hash for EntityId {
    #[inline(always)]
    fn hash<H: hash::Hasher>(&self, state: &mut H) {
        state.write_u32(self.index_u32());
    }
}

impl fmt::Display for EntityId {
    #[inline(always)]
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        fmt::Display::fmt(&self.index_u32(), f)
    }
}

// -----------------------------------------------------------------------------
// EntityGeneration

/// This tracks different versions or generations of an [`EntityId`].
///
/// Importantly, this can wrap, meaning each generation is not necessarily
/// unique per [`EntityId`].
///
/// # Aliasing
///
/// Internally [`EntityGeneration`] wraps a `u32`, so it can't represent *every*
/// possible generation. Eventually, generations can (and do) wrap or alias.
///
/// This can cause [`Entity`] and [`EntityGeneration`] values to be equal while
/// still referring to different conceptual entities. Therefore, users should not
/// hold an `Entity` for a long time.
///
/// [`Entity`]: crate::entity::Entity
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(transparent)]
pub struct EntityGeneration(u32);

impl EntityGeneration {
    /// Represents the first generation of an [`EntityId`].
    pub const FIRST: Self = Self(0);

    /// Non-wrapping difference between two generations after which a
    /// signed interpretation becomes negative.
    const DIFF_MAX: u32 = 1u32 << 31;

    /// Returns the [`EntityGeneration`] that would result from this many
    /// more `versions` of the corresponding [`EntityId`] from passing.
    #[inline(always)]
    pub const fn after(self, versions: u32) -> Self {
        Self(self.0.wrapping_add(versions))
    }

    /// Identical to [`after`](Self::after) but also returns a `bool` indicating if,
    /// after these `versions`, one such version could conflict with a previous one.
    ///
    /// If this happens, this will no longer uniquely identify a version of an
    /// [`EntityId`]. This is called entity aliasing.
    #[inline]
    pub const fn after_check_alias(self, versions: u32) -> (Self, bool) {
        let raw = self.0.overflowing_add(versions);
        (Self(raw.0), raw.1)
    }

    /// Compares two generations.
    ///
    /// Generations that are later will be [`Greater`](core::cmp::Ordering::Greater)
    /// than earlier ones.
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

impl hash::Hash for EntityGeneration {
    #[inline(always)]
    fn hash<H: hash::Hasher>(&self, state: &mut H) {
        state.write_u32(self.0);
    }
}

impl fmt::Display for EntityGeneration {
    #[inline(always)]
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        fmt::Display::fmt(&self.0, f)
    }
}
