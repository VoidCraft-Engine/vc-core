#![expect(unsafe_code, reason = "unchecked non-zero is unsafe.")]

use core::fmt;
use core::hash;
use core::num::NonZeroU32;

// -----------------------------------------------------------------------------
// ComponentId

#[derive(Debug, Clone, Copy, PartialOrd, Ord)]
#[repr(transparent)]
pub struct ComponentId(NonZeroU32);

impl ComponentId {
    const _STATIC_ASSERT_: () = const {
        const VAL: u32 = 2026;
        #[allow(clippy::transmute_int_to_non_zero, reason = "static_assert")]
        const NON_ZERO: NonZeroU32 = unsafe { core::mem::transmute(VAL) };
        const ID: ComponentId = unsafe { core::mem::transmute(VAL) };
        assert!(VAL == NON_ZERO.get());
        assert!(VAL == ID.index_u32());
    };

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
        unsafe { core::mem::transmute(self) }
    }

    #[inline(always)]
    pub const fn index(self) -> usize {
        self.index_u32() as usize
    }
}

impl PartialEq for ComponentId {
    #[inline(always)]
    fn eq(&self, other: &Self) -> bool {
        self.index_u32() == other.index_u32()
    }
}

impl Eq for ComponentId {}

impl hash::Hash for ComponentId {
    #[inline(always)]
    fn hash<H: hash::Hasher>(&self, state: &mut H) {
        state.write_u32(self.index_u32());
    }
}

impl fmt::Display for ComponentId {
    #[inline(always)]
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        fmt::Display::fmt(&self.index_u32(), f)
    }
}

// -----------------------------------------------------------------------------
// ComponentIdGenerator

use vc_os::sync::atomic::{AtomicU32, Ordering};

#[derive(Debug)]
pub struct ComponentIdGenerator {
    next: AtomicU32,
}

impl Default for ComponentIdGenerator {
    #[inline(always)]
    fn default() -> Self {
        Self::new()
    }
}

impl ComponentIdGenerator {
    #[inline(always)]
    const unsafe fn get_unchecked(id: u32) -> ComponentId {
        unsafe { core::mem::transmute(id) }
    }

    #[inline(always)]
    pub const fn new() -> Self {
        // SAFETY: start from `1` instead of `0`.
        Self {
            next: AtomicU32::new(1),
        }
    }

    #[inline(always)]
    pub fn peek_mut(&mut self) -> ComponentId {
        unsafe { Self::get_unchecked(*self.next.get_mut()) }
    }

    #[inline]
    pub fn next_mut(&mut self) -> ComponentId {
        let next = self.next.get_mut();
        assert!(*next < u32::MAX, "too many components");
        let result = unsafe { Self::get_unchecked(*next) };
        *next += 1;
        result
    }

    #[inline]
    pub fn peek(&self) -> ComponentId {
        let next = self.next.fetch_add(1, Ordering::Relaxed);
        unsafe { Self::get_unchecked(next) }
    }

    #[inline]
    pub fn next(&self) -> ComponentId {
        let next = self.next.fetch_add(1, Ordering::Relaxed);
        assert!(next < u32::MAX, "too many components");
        unsafe { Self::get_unchecked(next) }
    }
}
