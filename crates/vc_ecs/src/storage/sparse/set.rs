#![expect(unsafe_code, reason = "For better performance.")]

use alloc::vec::Vec;

use nonmax::NonMaxU32;

use crate::cfg;
use crate::storage::utils::VecCopyRemove;

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

    pub fn insert(&mut self, index: I, value: V) {
        if let Some(dense_index) = self.sparse.get_copied(index) {
            unsafe {
                *self.dense.get_unchecked_mut(dense_index.get() as usize) = value;
            }
        } else {
            let len = self.dense.len();
            cfg::debug! {
                assert!(len < u32::MAX as usize);
            }

            self.sparse
                .insert(index, unsafe { NonMaxU32::new_unchecked(len as u32) });
            self.indices.push(index);
            self.dense.push(value);
        }
    }

    pub fn remove(&mut self, index: I) -> Option<V> {
        use crate::storage::VecSwapRemove;

        self.sparse.remove(index).map(|dense_index| {
            let index = dense_index.get() as usize;
            let last_index = self.indices.len() - 1;

            if index == last_index {
                unsafe {
                    let value = self.dense.remove_last(index);
                    self.indices.set_len(index);
                    value
                }
            } else {
                unsafe {
                    let value = self.dense.swap_remove_nonoverlapping(index, last_index);
                    let swapped_index = self
                        .indices
                        .copy_last_and_return_nonoverlapping(index, last_index);

                    *self.sparse.get_mut(swapped_index).unwrap_unchecked() = dense_index;
                    value
                }
            }
        })
    }

    pub fn clear(&mut self) {
        self.dense.clear();
        self.indices.clear();
        self.sparse.clear();
    }
}
