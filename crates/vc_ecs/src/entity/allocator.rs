use alloc::vec::Vec;
use core::num::NonZeroU32;
use core::sync::atomic::Ordering;

use vc_os::sync::atomic::{AtomicU32, AtomicUsize};

use super::{Entity, EntityId};

// -----------------------------------------------------------------------------
// EntityAllocator

#[derive(Debug)]
pub struct EntityAllocator {
    free: Vec<Entity>,
    free_len: AtomicUsize,
    next_index: AtomicU32,
}

impl Default for EntityAllocator {
    #[inline(always)]
    fn default() -> Self {
        const { Self::new() }
    }
}

impl EntityAllocator {
    pub const fn new() -> Self {
        Self {
            free: Vec::new(),
            free_len: AtomicUsize::new(0),
            // SAFETY: start from `1`, instead of `0`.
            next_index: AtomicU32::new(1),
        }
    }

    /// Restarts the allocator.
    pub fn restart(&mut self) {
        self.free.clear();
        *self.free_len.get_mut() = 0;
        // SAFETY: start from `1`, instead of `0`.
        *self.next_index.get_mut() = 1;
    }

    pub fn free(&mut self, freed: Entity) {
        let expected_len = *self.free_len.get_mut();
        if expected_len > self.free.len() {
            self.free.clear();
        } else {
            self.free.truncate(expected_len);
        }
        self.free.push(freed);
        *self.free_len.get_mut() = self.free.len();
    }

    pub fn alloc(&self) -> Entity {
        let index = self
            .free_len
            .fetch_sub(1, Ordering::Relaxed)
            .wrapping_sub(1);

        self.free.get(index).copied().unwrap_or_else(|| {
            let index = self.next_index.fetch_add(1, Ordering::Relaxed);
            assert!(index < u32::MAX, "too many entities");

            #[expect(unsafe_code, reason = "1 <= index < u32::MAX")]
            let index = unsafe { NonZeroU32::new_unchecked(index) };

            Entity::from_id(EntityId::new(index))
        })
    }

    pub fn alloc_many(&self, count: u32) -> AllocatedEntities<'_> {
        // Ensure that count <= u32::MAX.
        let count = count as usize;

        let current_len = self.free_len.fetch_sub(count, Ordering::Relaxed);

        let current_len = if current_len <= self.free.len() {
            current_len
        } else {
            0
        };

        let start = current_len.saturating_sub(count);
        let reuse = start..current_len;

        let still_need = (count + start - current_len) as u32;
        let new = if still_need == 0 {
            0..0
        } else {
            let start_new = self.next_index.fetch_add(still_need, Ordering::Relaxed);
            assert!(start_new <= u32::MAX - still_need, "too many entities");
            let end_new = start_new + still_need;
            start_new..end_new
        };

        AllocatedEntities {
            reuse: self.free[reuse].iter(),
            new,
        }
    }
}

// -----------------------------------------------------------------------------
// AllocatedEntities

pub struct AllocatedEntities<'a> {
    reuse: core::slice::Iter<'a, Entity>,
    new: core::ops::Range<u32>,
}

impl<'a> Iterator for AllocatedEntities<'a> {
    type Item = Entity;

    fn next(&mut self) -> Option<Self::Item> {
        self.reuse.next().copied().or_else(|| {
            self.new.next().map(|index| {
                #[expect(unsafe_code, reason = "1 <= index < u32::MAX")]
                let index = unsafe { NonZeroU32::new_unchecked(index) };
                Entity::from_id(EntityId::new(index))
            })
        })
    }

    #[inline]
    fn size_hint(&self) -> (usize, Option<usize>) {
        let len = self.reuse.len() + self.new.len();
        (len, Some(len))
    }
}

impl<'a> ExactSizeIterator for AllocatedEntities<'a> {}

impl<'a> core::iter::FusedIterator for AllocatedEntities<'a> {}
