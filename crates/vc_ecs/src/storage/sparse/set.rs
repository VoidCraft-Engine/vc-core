#![expect(unsafe_code, reason = "original implementation need unsafe codes.")]

use alloc::vec::Vec;

use nonmax::NonMaxU32;

use crate::cfg;

use super::SparseIndex;
use super::SparseMap;

// -----------------------------------------------------------------------------
// SparseSet

#[derive(Debug)]
pub struct SparseSet<I, V> {
    dense: Vec<V>,
    indices: Vec<I>,
    sparse: SparseMap<I, NonMaxU32>,
}

// -----------------------------------------------------------------------------
// SparseSet Implementation

impl<I: SparseIndex, V> SparseSet<I, V> {
    #[inline]
    pub const fn empty() -> Self {
        Self {
            dense: Vec::new(),
            indices: Vec::new(),
            sparse: SparseMap::empty(),
        }
    }

    pub fn with_capacity(capacity: usize) -> Self {
        Self {
            dense: Vec::with_capacity(capacity),
            indices: Vec::with_capacity(capacity),
            sparse: SparseMap::empty(),
        }
    }

    #[inline]
    pub fn len(&self) -> usize {
        self.dense.len()
    }

    #[inline]
    pub fn is_empty(&self) -> bool {
        self.dense.len() == 0
    }

    #[inline]
    pub fn capacity(&self) -> usize {
        self.dense.capacity()
    }

    #[inline]
    pub fn contains(&self, index: I) -> bool {
        self.sparse.contains(index)
    }

    #[inline]
    pub fn get_raw_index(&self, index: I) -> Option<u32> {
        self.sparse.get_copied(index).map(|v| v.get())
    }

    #[inline]
    pub fn indices(&self) -> &[I] {
        &self.indices
    }

    #[inline]
    pub fn get(&self, index: I) -> Option<&V> {
        self.sparse
            .get_copied(index)
            .map(|dense_index| unsafe { self.dense.get_unchecked(dense_index.get() as usize) })
    }

    #[inline]
    pub fn get_mut(&mut self, index: I) -> Option<&mut V> {
        self.sparse
            .get_copied(index)
            .map(|dense_index| unsafe { self.dense.get_unchecked_mut(dense_index.get() as usize) })
    }

    #[inline(always)]
    pub unsafe fn get_raw(&self, raw_index: u32) -> &V {
        cfg::debug! { assert!((raw_index as usize) < self.dense.len()); }
        unsafe { self.dense.get_unchecked(raw_index as usize) }
    }

    #[inline(always)]
    pub unsafe fn get_mut_raw(&mut self, raw_index: u32) -> &mut V {
        cfg::debug! { assert!((raw_index as usize) < self.dense.len()); }
        unsafe { self.dense.get_unchecked_mut(raw_index as usize) }
    }

    pub fn values(&self) -> impl Iterator<Item = &V> {
        self.dense.iter()
    }

    pub fn values_mut(&mut self) -> impl Iterator<Item = &mut V> {
        self.dense.iter_mut()
    }

    pub fn iter(&self) -> impl Iterator<Item = (&I, &V)> {
        self.indices.iter().zip(self.dense.iter())
    }

    pub fn iter_mut(&mut self) -> impl Iterator<Item = (&I, &mut V)> {
        self.indices.iter().zip(self.dense.iter_mut())
    }

    pub fn insert(&mut self, index: I, value: V) -> u32 {
        if let Some(dense_index) = self.sparse.get_copied(index) {
            let raw_index = dense_index.get();
            unsafe {
                *self.dense.get_unchecked_mut(raw_index as usize) = value;
            }
            raw_index
        } else {
            let len = self.dense.len();

            cfg::debug! {
                assert!(len < u32::MAX as usize);
            }

            let raw_index = len as u32;

            self.sparse
                .insert(index, unsafe { NonMaxU32::new_unchecked(raw_index) });
            self.indices.push(index);
            self.dense.push(value);

            raw_index
        }
    }

    pub fn clear(&mut self) {
        self.dense.clear();
        self.indices.clear();
        self.sparse.clear();
    }

    pub fn get_or_insert_with(&mut self, index: I, func: impl FnOnce() -> V) -> &mut V {
        if let Some(dense_index) = self.sparse.get_copied(index) {
            // SAFETY: dense indices stored in self.sparse always exist
            unsafe { self.dense.get_unchecked_mut(dense_index.get() as usize) }
        } else {
            let dense_index = self.dense.len();

            cfg::debug! {
                assert!(dense_index < u32::MAX as usize);
            }

            let value = func();

            self.sparse.insert(index, unsafe {
                NonMaxU32::new_unchecked(dense_index as u32)
            });
            self.indices.push(index);
            self.dense.push(value);

            // SAFETY: dense index was just populated above
            unsafe { self.dense.get_unchecked_mut(dense_index) }
        }
    }
}
