#![expect(unsafe_code, reason = "get_unchecked is unsafe.")]

use alloc::boxed::Box;
use alloc::vec::Vec;
use core::marker::PhantomData;

use super::SparseIndex;

// -----------------------------------------------------------------------------
// FixedSparseMap

#[derive(Debug)]
pub struct FixedSparseMap<I, V> {
    values: Box<[Option<V>]>,
    _marker: PhantomData<I>,
}

impl<I: SparseIndex, V> FixedSparseMap<I, V> {
    #[inline]
    pub fn contains(&self, index: I) -> bool {
        let index = index.sparse_index();
        self.values.get(index).is_some_and(Option::is_some)
    }

    #[inline]
    pub fn get_copied(&self, index: I) -> Option<V>
    where
        V: Copy,
    {
        let index = index.sparse_index();
        self.values.get(index).and_then(|&v| v)
    }
}

// -----------------------------------------------------------------------------
// SparseMap

#[derive(Debug)]
pub struct SparseMap<I, V = I> {
    values: Vec<Option<V>>,
    _marker: PhantomData<I>,
}

impl<I: SparseIndex, V> SparseMap<I, V> {
    #[inline]
    pub fn into_fixed(self) -> FixedSparseMap<I, V> {
        FixedSparseMap {
            values: self.values.into_boxed_slice(),
            _marker: PhantomData,
        }
    }

    #[inline]
    pub const fn empty() -> Self {
        Self {
            values: Vec::new(),
            _marker: PhantomData,
        }
    }

    #[inline]
    pub fn contains(&self, index: I) -> bool {
        let index = index.sparse_index();
        self.values.get(index).is_some_and(Option::is_some)
    }

    #[inline]
    pub fn get(&self, index: I) -> Option<&V> {
        let index = index.sparse_index();
        self.values.get(index).and_then(Option::as_ref)
    }

    #[inline]
    pub fn get_mut(&mut self, index: I) -> Option<&mut V> {
        let index = index.sparse_index();
        self.values.get_mut(index).and_then(Option::as_mut)
    }

    #[inline]
    pub fn get_copied(&self, index: I) -> Option<V>
    where
        V: Copy,
    {
        let index = index.sparse_index();
        self.values.get(index).and_then(|&v| v)
    }

    #[inline]
    pub fn insert(&mut self, index: I, value: V) {
        let index = index.sparse_index();
        if index >= self.values.len() {
            self.values.resize_with(index + 1, || None);
        }
        unsafe {
            *self.values.get_unchecked_mut(index) = Some(value);
        }
    }

    #[inline]
    pub fn remove(&mut self, index: I) -> Option<V> {
        let index = index.sparse_index();
        self.values.get_mut(index).and_then(Option::take)
    }

    #[inline]
    pub fn clear(&mut self) {
        self.values.clear();
    }
}
