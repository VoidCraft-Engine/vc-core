use core::fmt;
use core::hash;

// -----------------------------------------------------------------------------
// ComponentId

#[derive(Debug, Clone, Copy, Ord, PartialOrd)]
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

impl PartialEq for ComponentId {
    #[inline(always)]
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}

impl Eq for ComponentId {}

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

// -----------------------------------------------------------------------------
// ComponentIdGenerator

use vc_os::sync::atomic::{AtomicU32, Ordering};

#[derive(Debug)]
pub struct ComponentIdGenerator {
    next: AtomicU32,
}

impl ComponentIdGenerator {
    pub fn peek(&self) -> ComponentId {
        ComponentId(self.next.load(Ordering::Relaxed))
    }

    pub fn next(&self) -> ComponentId {
        let next = self.next.fetch_add(1, Ordering::Relaxed);
        assert!(next < u32::MAX, "too many components");
        ComponentId(next)
    }

    pub fn peek_mut(&mut self) -> ComponentId {
        ComponentId(*self.next.get_mut())
    }

    pub fn next_mut(&mut self) -> ComponentId {
        let next = self.next.get_mut();
        assert!(*next < u32::MAX, "too many components");
        let result = ComponentId(*next);
        *next += 1;
        result
    }
}
