use core::fmt::Debug;
use core::iter::FusedIterator;

use super::unique_slice::UniqueEntityEquivalentSlice;
use super::{EntityEquivalent, EntitySetIterator};

// -----------------------------------------------------------------------------
// UniqueEntityIter

/// An iterator that yields unique entities.
///
/// This wrapper can provide an [`EntitySetIterator`] implementation
/// when an instance of `I` is known to uphold uniqueness.
#[repr(transparent)]
pub struct UniqueEntityIter<I>(I);

// -----------------------------------------------------------------------------
// Implementation

impl<I> UniqueEntityIter<I> {
    /// Create a `UniqueEntityIter` from an `EntitySetIterator`.
    ///
    /// # Safety
    ///
    /// We ensure that `EntitySetIterator` yield unique elements.
    #[inline(always)]
    pub const fn new(iter: I) -> Self
    where
        I: EntitySetIterator,
    {
        Self(iter)
    }

    /// Create a `UniqueEntityIter` from an iterator.
    ///
    /// # Safety
    /// `iter` must only yield unique elements.
    #[inline(always)]
    pub const unsafe fn new_unchecked(iter: I) -> Self
    where
        I: Iterator<Item: EntityEquivalent>,
    {
        Self(iter)
    }

    /// Returns the inner `I`.
    #[inline(always)]
    pub fn into_inner(self) -> I {
        self.0
    }

    /// Returns the reference of inner `I`.
    #[inline(always)]
    pub const fn as_inner(&self) -> &I {
        &self.0
    }

    /// Returns the mutable reference of inner `I`.
    ///
    /// # Safety
    ///
    /// Ensure that `iter` must only yield unique elements.
    #[inline(always)]
    pub const unsafe fn as_mut_inner(&mut self) -> &mut I {
        &mut self.0
    }
}

// -----------------------------------------------------------------------------
// Debug Default Clone

impl<I: Debug> Debug for UniqueEntityIter<I> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_struct("UniqueEntityIter")
            .field("iter", &self.0)
            .finish()
    }
}

impl<I: Default> Default for UniqueEntityIter<I>
where
    I: EntitySetIterator, // guarantee uniqueness.
{
    #[inline]
    fn default() -> Self {
        Self(Default::default())
    }
}

impl<I: Clone> Clone for UniqueEntityIter<I>
where
    I: EntitySetIterator, // guarantee uniqueness.
{
    #[inline]
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }

    #[inline]
    fn clone_from(&mut self, source: &Self) {
        self.0.clone_from(&source.0);
    }
}

// -----------------------------------------------------------------------------
// Iterator

unsafe impl<I> EntitySetIterator for UniqueEntityIter<I> where I: Iterator<Item: EntityEquivalent> {}

impl<I> Iterator for UniqueEntityIter<I>
where
    I: Iterator<Item: EntityEquivalent>,
{
    type Item = I::Item;

    #[inline(always)]
    fn next(&mut self) -> Option<Self::Item> {
        self.0.next()
    }

    #[inline(always)]
    fn size_hint(&self) -> (usize, Option<usize>) {
        self.0.size_hint()
    }
}

impl<I> DoubleEndedIterator for UniqueEntityIter<I>
where
    I: DoubleEndedIterator<Item: EntityEquivalent>,
{
    #[inline(always)]
    fn next_back(&mut self) -> Option<Self::Item> {
        self.0.next_back()
    }
}

impl<I: ExactSizeIterator<Item: EntityEquivalent>> ExactSizeIterator for UniqueEntityIter<I> {}
impl<I: FusedIterator<Item: EntityEquivalent>> FusedIterator for UniqueEntityIter<I> {}

// -----------------------------------------------------------------------------
// AsRef

impl<T, I: AsRef<[T]>> AsRef<[T]> for UniqueEntityIter<I> {
    #[inline(always)]
    fn as_ref(&self) -> &[T] {
        self.0.as_ref()
    }
}

impl<T, I> AsRef<UniqueEntityEquivalentSlice<T>> for UniqueEntityIter<I>
where
    T: EntityEquivalent,
    I: AsRef<[T]>,
{
    #[inline]
    fn as_ref(&self) -> &UniqueEntityEquivalentSlice<T> {
        // SAFETY: All elements in the original slice are unique.
        unsafe { UniqueEntityEquivalentSlice::from_slice_unchecked(self.0.as_ref()) }
    }
}

impl<T, I> AsMut<UniqueEntityEquivalentSlice<T>> for UniqueEntityIter<I>
where
    T: EntityEquivalent,
    I: AsMut<[T]>,
{
    #[inline]
    fn as_mut(&mut self) -> &mut UniqueEntityEquivalentSlice<T> {
        // SAFETY: All elements in the original slice are unique.
        unsafe { UniqueEntityEquivalentSlice::from_slice_unchecked_mut(self.0.as_mut()) }
    }
}
