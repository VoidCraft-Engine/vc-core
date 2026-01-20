use alloc::boxed::Box;
use alloc::collections::{btree_map, btree_set};
use core::array;
use core::hash::{BuildHasher, Hash};
use core::iter;
use core::{option, result};

use crate::entity::EntityEquivalent;

// -----------------------------------------------------------------------------
// Traits

pub unsafe trait EntitySetIterator: Iterator<Item: EntityEquivalent> {
    #[inline(always)]
    fn collect_entity_set<B: FromEntitySet<Self::Item>>(self) -> B
    where
        Self: Sized,
    {
        FromEntitySet::from_entity_set(self)
    }
}

pub trait EntitySet: IntoIterator<IntoIter: EntitySetIterator> {}

impl<T: IntoIterator<IntoIter: EntitySetIterator>> EntitySet for T {}

pub trait FromEntitySet<A: EntityEquivalent> {
    fn from_entity_set<T: EntitySet<Item = A>>(set_iter: T) -> Self;
}

// -----------------------------------------------------------------------------
// EntitySetIterator Implementation

unsafe impl<K: EntityEquivalent, V> EntitySetIterator for btree_map::Keys<'_, K, V> {}
unsafe impl<K: EntityEquivalent, V> EntitySetIterator for btree_map::IntoKeys<K, V> {}

unsafe impl<T: EntityEquivalent> EntitySetIterator for btree_set::Range<'_, T> {}
unsafe impl<T: EntityEquivalent + Ord> EntitySetIterator for btree_set::Intersection<'_, T> {}
unsafe impl<T: EntityEquivalent + Ord> EntitySetIterator for btree_set::Union<'_, T> {}
unsafe impl<T: EntityEquivalent + Ord> EntitySetIterator for btree_set::Difference<'_, T> {}
unsafe impl<T: EntityEquivalent + Ord> EntitySetIterator for btree_set::SymmetricDifference<'_, T> {}
unsafe impl<T: EntityEquivalent> EntitySetIterator for btree_set::Iter<'_, T> {}
unsafe impl<T: EntityEquivalent> EntitySetIterator for btree_set::IntoIter<T> {}

unsafe impl<T: EntityEquivalent> EntitySetIterator for option::Iter<'_, T> {}
unsafe impl<T: EntityEquivalent> EntitySetIterator for option::IntoIter<T> {}

unsafe impl<T: EntityEquivalent> EntitySetIterator for result::Iter<'_, T> {}
unsafe impl<T: EntityEquivalent> EntitySetIterator for result::IntoIter<T> {}

unsafe impl<T: EntityEquivalent> EntitySetIterator for array::IntoIter<T, 0> {}
unsafe impl<T: EntityEquivalent> EntitySetIterator for array::IntoIter<T, 1> {}

unsafe impl<T: EntityEquivalent, F: FnOnce() -> T> EntitySetIterator for iter::OnceWith<F> {}
unsafe impl<T: EntityEquivalent> EntitySetIterator for iter::Once<T> {}
unsafe impl<T: EntityEquivalent> EntitySetIterator for iter::Empty<T> {}
unsafe impl<I: EntitySetIterator> EntitySetIterator for iter::Fuse<I> {}
unsafe impl<I: DoubleEndedIterator + EntitySetIterator> EntitySetIterator for iter::Rev<I> {}
unsafe impl<I: EntitySetIterator> EntitySetIterator for iter::Skip<I> {}
unsafe impl<I: EntitySetIterator> EntitySetIterator for iter::Take<I> {}
unsafe impl<I: EntitySetIterator> EntitySetIterator for iter::StepBy<I> {}
unsafe impl<'a, T: 'a + EntityEquivalent + Copy, I: EntitySetIterator<Item = &'a T>>
    EntitySetIterator for iter::Copied<I>
{
}
unsafe impl<'a, T: 'a + EntityEquivalent + Clone, I: EntitySetIterator<Item = &'a T>>
    EntitySetIterator for iter::Cloned<I>
{
}
unsafe impl<I: EntitySetIterator, P: FnMut(&<I as Iterator>::Item) -> bool> EntitySetIterator
    for iter::Filter<I, P>
{
}
unsafe impl<I: EntitySetIterator, F: FnMut(&<I as Iterator>::Item)> EntitySetIterator
    for iter::Inspect<I, F>
{
}
unsafe impl<I: EntitySetIterator, P: FnMut(&<I as Iterator>::Item) -> bool> EntitySetIterator
    for iter::SkipWhile<I, P>
{
}
unsafe impl<I: EntitySetIterator, P: FnMut(&<I as Iterator>::Item) -> bool> EntitySetIterator
    for iter::TakeWhile<I, P>
{
}

unsafe impl<I: EntitySetIterator + ?Sized> EntitySetIterator for &mut I {}
unsafe impl<I: EntitySetIterator + ?Sized> EntitySetIterator for Box<I> {}

// -----------------------------------------------------------------------------
// FromEntitySetIterator Implementation

use alloc::collections::BTreeSet;
use vc_utils::hash::{HashSet, NoOpHashSet, SparseHashSet};
use vc_utils::index::{IndexSet, SparseIndexSet};

impl<T: EntityEquivalent + Hash, S: BuildHasher + Default> FromEntitySet<T> for HashSet<T, S> {
    fn from_entity_set<I: EntitySet<Item = T>>(set_iter: I) -> Self {
        let iter = set_iter.into_iter();

        let mut set = HashSet::<T, S>::with_capacity_and_hasher(iter.size_hint().0, S::default());

        // Internal iteration `for_each` is known to result
        // in better code generation over a for loop.
        iter.for_each(|item| unsafe {
            set.insert_unique_unchecked(item);
        });

        set
    }
}

impl<T: EntityEquivalent + Hash> FromEntitySet<T> for SparseHashSet<T> {
    fn from_entity_set<I: EntitySet<Item = T>>(set_iter: I) -> Self {
        let iter = set_iter.into_iter();

        let mut set = SparseHashSet::<T>::with_capacity(iter.size_hint().0);

        // Internal iteration `for_each` is known to result
        // in better code generation over a for loop.
        iter.for_each(|item| unsafe {
            set.insert_unique_unchecked(item);
        });

        set
    }
}

impl<T: EntityEquivalent + Hash> FromEntitySet<T> for NoOpHashSet<T> {
    fn from_entity_set<I: EntitySet<Item = T>>(set_iter: I) -> Self {
        let iter = set_iter.into_iter();

        let mut set = NoOpHashSet::<T>::with_capacity(iter.size_hint().0);

        // Internal iteration `for_each` is known to result
        // in better code generation over a for loop.
        iter.for_each(|item| unsafe {
            set.insert_unique_unchecked(item);
        });

        set
    }
}

impl<T: EntityEquivalent + Hash, S: BuildHasher + Default> FromEntitySet<T> for IndexSet<T, S> {
    fn from_entity_set<I: EntitySet<Item = T>>(set_iter: I) -> Self {
        let iter = set_iter.into_iter();

        let mut set = IndexSet::<T, S>::with_capacity_and_hasher(iter.size_hint().0, S::default());

        // Internal iteration `for_each` is known to result
        // in better code generation over a for loop.
        iter.for_each(|item| {
            set.insert(item);
        });

        set
    }
}

impl<T: EntityEquivalent + Hash> FromEntitySet<T> for SparseIndexSet<T> {
    fn from_entity_set<I: EntitySet<Item = T>>(set_iter: I) -> Self {
        let iter = set_iter.into_iter();

        let mut set = SparseIndexSet::<T>::with_capacity(iter.size_hint().0);

        // Internal iteration `for_each` is known to result
        // in better code generation over a for loop.
        iter.for_each(|item| {
            set.insert(item);
        });

        set
    }
}

impl<T: EntityEquivalent + Ord> FromEntitySet<T> for BTreeSet<T> {
    fn from_entity_set<I: EntitySet<Item = T>>(set_iter: I) -> Self {
        let iter = set_iter.into_iter();

        let mut set = BTreeSet::<T>::new();

        // Internal iteration `for_each` is known to result
        // in better code generation over a for loop.
        iter.for_each(|item| {
            set.insert(item);
        });

        set
    }
}
