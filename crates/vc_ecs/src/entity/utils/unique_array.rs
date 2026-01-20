use alloc::boxed::Box;
use alloc::rc::Rc;
use core::array;
use core::borrow::{Borrow, BorrowMut};
use core::fmt::Debug;
use core::ops::{Bound, Deref, DerefMut, Index, IndexMut};
use core::ops::{Range, RangeFrom, RangeFull, RangeInclusive};
use core::ops::{RangeTo, RangeToInclusive};
use core::ptr;

use vc_os::sync::Arc;

use super::EntityEquivalent;
use super::unique_slice::UniqueEntityEquivalentSlice;
use crate::entity::Entity;

// -----------------------------------------------------------------------------
// UniqueEntityEquivalentArray

/// An array that contains only unique entities.
///
/// It can be obtained through certain methods on [`UniqueEntityEquivalentSlice`],
/// and some [`TryFrom`] implementations.
///
/// When `T` is [`Entity`], use [`UniqueEntityArray`].
#[derive(Clone, Copy, Debug, Hash, PartialEq, Eq, PartialOrd, Ord)]
#[repr(transparent)]
pub struct UniqueEntityEquivalentArray<T, const N: usize>([T; N]);

/// An array that contains only unique [`Entity`].
///
/// This is the default case of a [`UniqueEntityEquivalentArray`].
pub type UniqueEntityArray<const N: usize> = UniqueEntityEquivalentArray<Entity, N>;

// -----------------------------------------------------------------------------
// UniqueEntityEquivalentArray Implementation

impl<T: EntityEquivalent, const N: usize> UniqueEntityEquivalentArray<T, N> {
    /// A basic API for implementing operations
    /// such as comparison and hash calculation.
    #[inline(always)]
    pub const fn data_slice(&self) -> &[T] {
        &self.0
    }

    /// Return the inner array.
    #[inline(always)]
    pub fn into_inner(self) -> [T; N] {
        self.0
    }

    /// Returns a reference to the inner array.
    pub fn as_inner(&self) -> &[T; N] {
        &self.0
    }

    /// Returns a mutable reference to the inner `[T; N]`.
    ///
    /// # Safety
    ///
    /// The elements of this Array must always remain unique, even while
    /// this mutable reference is live.
    pub unsafe fn as_mut_inner(&mut self) -> &mut [T; N] {
        &mut self.0
    }

    /// Returns a slice containing the entire array. Equivalent to `&s[..]`.
    pub const fn as_slice(&self) -> &UniqueEntityEquivalentSlice<T> {
        // SAFETY: All elements in the original array are unique.
        unsafe { UniqueEntityEquivalentSlice::from_slice_unchecked(self.0.as_slice()) }
    }

    /// Returns a mutable slice containing the entire array. Equivalent to
    /// `&mut s[..]`.
    pub fn as_mut_slice(&mut self) -> &mut UniqueEntityEquivalentSlice<T> {
        // SAFETY: All elements in the original array are unique.
        unsafe { UniqueEntityEquivalentSlice::from_slice_unchecked_mut(self.0.as_mut_slice()) }
    }

    /// Constructs a `UniqueEntityEquivalentArray` from a [`[T; N]`] unsafely.
    ///
    /// # Safety
    ///
    /// `array` must contain only unique elements.
    pub const unsafe fn from_array_unchecked(array: [T; N]) -> Self {
        Self(array)
    }

    /// Constructs a `&UniqueEntityEquivalentArray` from a [`&[T; N]`] unsafely.
    ///
    /// # Safety
    ///
    /// `array` must contain only unique elements.
    pub const unsafe fn from_array_ref_unchecked(array: &[T; N]) -> &Self {
        // SAFETY: UniqueEntityEquivalentArray is a transparent wrapper around [T; N].
        unsafe { &*(ptr::from_ref(array).cast()) }
    }

    /// Constructs a `Box<UniqueEntityEquivalentArray>` from a [`Box<[T; N]>`] unsafely.
    ///
    /// # Safety
    ///
    /// `array` must contain only unique elements.
    pub unsafe fn from_boxed_array_unchecked(array: Box<[T; N]>) -> Box<Self> {
        // SAFETY: UniqueEntityEquivalentArray is a transparent wrapper around [T; N].
        unsafe { Box::from_raw(Box::into_raw(array).cast()) }
    }

    /// Constructs a `Arc<UniqueEntityEquivalentArray>` from a [`Arc<[T; N]>`] unsafely.
    ///
    /// # Safety
    ///
    /// `slice` must contain only unique elements.
    pub unsafe fn from_arc_array_unchecked(slice: Arc<[T; N]>) -> Arc<Self> {
        // SAFETY: UniqueEntityEquivalentArray is a transparent wrapper around [T; N].
        unsafe { Arc::from_raw(Arc::into_raw(slice).cast()) }
    }

    // Constructs a `Rc<UniqueEntityEquivalentArray>` from a [`Rc<[T; N]>`] unsafely.
    ///
    /// # Safety
    ///
    /// `slice` must contain only unique elements.
    pub unsafe fn from_rc_array_unchecked(slice: Rc<[T; N]>) -> Rc<Self> {
        // SAFETY: UniqueEntityEquivalentArray is a transparent wrapper around [T; N].
        unsafe { Rc::from_raw(Rc::into_raw(slice).cast()) }
    }

    /// Casts `self` into the inner array.
    pub fn into_boxed_inner(self: Box<Self>) -> Box<[T; N]> {
        // SAFETY: UniqueEntityEquivalentArray is a transparent wrapper around [T; N].
        unsafe { Box::from_raw(Box::into_raw(self).cast()) }
    }

    /// Casts `self` to the inner array.
    pub fn into_arc_inner(this: Arc<Self>) -> Arc<[T; N]> {
        // SAFETY: UniqueEntityEquivalentArray is a transparent wrapper around [T; N].
        unsafe { Arc::from_raw(Arc::into_raw(this).cast()) }
    }

    /// Casts `self` to the inner array.
    pub fn into_rc_inner(self: Rc<Self>) -> Rc<[T; N]> {
        // SAFETY: UniqueEntityEquivalentArray is a transparent wrapper around [T; N].
        unsafe { Rc::from_raw(Rc::into_raw(self).cast()) }
    }

    /// Borrows each element and returns an array of references with the same
    /// size as `self`.
    ///
    /// Equivalent to [`[T; N]::as_ref`](array::each_ref).
    pub fn each_ref(&self) -> UniqueEntityEquivalentArray<&T, N> {
        UniqueEntityEquivalentArray(self.0.each_ref())
    }
}

// -----------------------------------------------------------------------------
// Deref AsRef and Borrow

impl<T: EntityEquivalent, const N: usize> Deref for UniqueEntityEquivalentArray<T, N> {
    type Target = UniqueEntityEquivalentSlice<T>;

    fn deref(&self) -> &Self::Target {
        // SAFETY: All elements in the original array are unique.
        unsafe { UniqueEntityEquivalentSlice::from_slice_unchecked(&self.0) }
    }
}

impl<T: EntityEquivalent, const N: usize> DerefMut for UniqueEntityEquivalentArray<T, N> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        // SAFETY: All elements in the original array are unique.
        unsafe { UniqueEntityEquivalentSlice::from_slice_unchecked_mut(&mut self.0) }
    }
}

impl<T: EntityEquivalent, const N: usize> AsRef<UniqueEntityEquivalentSlice<T>>
    for UniqueEntityEquivalentArray<T, N>
{
    fn as_ref(&self) -> &UniqueEntityEquivalentSlice<T> {
        self
    }
}

impl<T: EntityEquivalent, const N: usize> AsMut<UniqueEntityEquivalentSlice<T>>
    for UniqueEntityEquivalentArray<T, N>
{
    fn as_mut(&mut self) -> &mut UniqueEntityEquivalentSlice<T> {
        self
    }
}

impl<T: EntityEquivalent, const N: usize> Borrow<UniqueEntityEquivalentSlice<T>>
    for UniqueEntityEquivalentArray<T, N>
{
    fn borrow(&self) -> &UniqueEntityEquivalentSlice<T> {
        self
    }
}

impl<T: EntityEquivalent, const N: usize> BorrowMut<UniqueEntityEquivalentSlice<T>>
    for UniqueEntityEquivalentArray<T, N>
{
    fn borrow_mut(&mut self) -> &mut UniqueEntityEquivalentSlice<T> {
        self
    }
}

impl<T: EntityEquivalent, const N: usize> AsRef<[T]> for UniqueEntityEquivalentArray<T, N> {
    fn as_ref(&self) -> &[T] {
        &self.0
    }
}

impl<T: EntityEquivalent, const N: usize> Borrow<[T]> for UniqueEntityEquivalentArray<T, N> {
    fn borrow(&self) -> &[T] {
        &self.0
    }
}

impl<T: EntityEquivalent, const N: usize> AsRef<Self> for UniqueEntityEquivalentArray<T, N> {
    fn as_ref(&self) -> &Self {
        self
    }
}

impl<T: EntityEquivalent, const N: usize> AsMut<Self> for UniqueEntityEquivalentArray<T, N> {
    fn as_mut(&mut self) -> &mut Self {
        self
    }
}

// -----------------------------------------------------------------------------
// IntoIterator

impl<'a, T, const N: usize> IntoIterator for &'a UniqueEntityEquivalentArray<T, N>
where
    T: EntityEquivalent,
{
    type Item = &'a T;
    type IntoIter = super::unique_slice::Iter<'a, T>;

    fn into_iter(self) -> Self::IntoIter {
        // SAFETY: All elements in the original array are unique.
        unsafe { super::UniqueEntityIter::new_unchecked(self.0.iter()) }
    }
}

impl<T, const N: usize> IntoIterator for UniqueEntityEquivalentArray<T, N>
where
    T: EntityEquivalent,
{
    type Item = T;
    type IntoIter = IntoIter<N, T>;

    fn into_iter(self) -> Self::IntoIter {
        // SAFETY: All elements in the original array are unique.
        unsafe { super::UniqueEntityIter::new_unchecked(self.0.into_iter()) }
    }
}

// -----------------------------------------------------------------------------
// From

// impl<T: EntityEquivalent + Clone> From<&[T; 1]> for UniqueEntityEquivalentArray<T, 1> {
//     fn from(value: &[T; 1]) -> Self {
//         Self(value.clone())
//     }
// }

// impl<T: EntityEquivalent + Clone> From<&[T; 0]> for UniqueEntityEquivalentArray<T, 0> {
//     fn from(value: &[T; 0]) -> Self {
//         Self(value.clone())
//     }
// }

// impl<T: EntityEquivalent + Clone> From<&mut [T; 1]> for UniqueEntityEquivalentArray<T, 1> {
//     fn from(value: &mut [T; 1]) -> Self {
//         Self(value.clone())
//     }
// }

// impl<T: EntityEquivalent + Clone> From<&mut [T; 0]> for UniqueEntityEquivalentArray<T, 0> {
//     fn from(value: &mut [T; 0]) -> Self {
//         Self(value.clone())
//     }
// }

// impl<T: EntityEquivalent> From<[T; 1]> for UniqueEntityEquivalentArray<T, 1> {
//     fn from(value: [T; 1]) -> Self {
//         Self(value)
//     }
// }

// impl<T: EntityEquivalent> From<[T; 0]> for UniqueEntityEquivalentArray<T, 0> {
//     fn from(value: [T; 0]) -> Self {
//         Self(value)
//     }
// }

// impl<T: EntityEquivalent> From<UniqueEntityEquivalentArray<T, 1>> for (T,) {
//     fn from(array: UniqueEntityEquivalentArray<T, 1>) -> Self {
//         Self::from(array.into_inner())
//     }
// }

// impl<T: EntityEquivalent> From<UniqueEntityEquivalentArray<T, 2>> for (T, T) {
//     fn from(array: UniqueEntityEquivalentArray<T, 2>) -> Self {
//         Self::from(array.into_inner())
//     }
// }

// impl<T: EntityEquivalent> From<UniqueEntityEquivalentArray<T, 3>> for (T, T, T) {
//     fn from(array: UniqueEntityEquivalentArray<T, 3>) -> Self {
//         Self::from(array.into_inner())
//     }
// }

// impl<T: EntityEquivalent> From<UniqueEntityEquivalentArray<T, 4>> for (T, T, T, T) {
//     fn from(array: UniqueEntityEquivalentArray<T, 4>) -> Self {
//         Self::from(array.into_inner())
//     }
// }

// impl<T: EntityEquivalent> From<UniqueEntityEquivalentArray<T, 5>> for (T, T, T, T, T) {
//     fn from(array: UniqueEntityEquivalentArray<T, 5>) -> Self {
//         Self::from(array.into_inner())
//     }
// }

// impl<T: EntityEquivalent> From<UniqueEntityEquivalentArray<T, 6>> for (T, T, T, T, T, T) {
//     fn from(array: UniqueEntityEquivalentArray<T, 6>) -> Self {
//         Self::from(array.into_inner())
//     }
// }

// impl<T: EntityEquivalent> From<UniqueEntityEquivalentArray<T, 7>> for (T, T, T, T, T, T, T) {
//     fn from(array: UniqueEntityEquivalentArray<T, 7>) -> Self {
//         Self::from(array.into_inner())
//     }
// }

// impl<T: EntityEquivalent> From<UniqueEntityEquivalentArray<T, 8>> for (T, T, T, T, T, T, T, T) {
//     fn from(array: UniqueEntityEquivalentArray<T, 8>) -> Self {
//         Self::from(array.into_inner())
//     }
// }

// impl<T: EntityEquivalent> From<UniqueEntityEquivalentArray<T, 9>> for (T, T, T, T, T, T, T, T, T) {
//     fn from(array: UniqueEntityEquivalentArray<T, 9>) -> Self {
//         Self::from(array.into_inner())
//     }
// }

// impl<T: EntityEquivalent> From<UniqueEntityEquivalentArray<T, 10>>
//     for (T, T, T, T, T, T, T, T, T, T)
// {
//     fn from(array: UniqueEntityEquivalentArray<T, 10>) -> Self {
//         Self::from(array.into_inner())
//     }
// }

// impl<T: EntityEquivalent> From<UniqueEntityEquivalentArray<T, 11>>
//     for (T, T, T, T, T, T, T, T, T, T, T)
// {
//     fn from(array: UniqueEntityEquivalentArray<T, 11>) -> Self {
//         Self::from(array.into_inner())
//     }
// }

// impl<T: EntityEquivalent> From<UniqueEntityEquivalentArray<T, 12>>
//     for (T, T, T, T, T, T, T, T, T, T, T, T)
// {
//     fn from(array: UniqueEntityEquivalentArray<T, 12>) -> Self {
//         Self::from(array.into_inner())
//     }
// }

// impl<T: EntityEquivalent + Ord, const N: usize> From<UniqueEntityEquivalentArray<T, N>>
//     for BTreeSet<T>
// {
//     fn from(value: UniqueEntityEquivalentArray<T, N>) -> Self {
//         BTreeSet::from(value.0)
//     }
// }

// impl<T: EntityEquivalent + Ord, const N: usize> From<UniqueEntityEquivalentArray<T, N>>
//     for BinaryHeap<T>
// {
//     fn from(value: UniqueEntityEquivalentArray<T, N>) -> Self {
//         BinaryHeap::from(value.0)
//     }
// }

// impl<T: EntityEquivalent, const N: usize> From<UniqueEntityEquivalentArray<T, N>>
//     for LinkedList<T>
// {
//     fn from(value: UniqueEntityEquivalentArray<T, N>) -> Self {
//         LinkedList::from(value.0)
//     }
// }

// impl<T: EntityEquivalent, const N: usize> From<UniqueEntityEquivalentArray<T, N>> for Vec<T> {
//     fn from(value: UniqueEntityEquivalentArray<T, N>) -> Self {
//         Vec::from(value.0)
//     }
// }

// impl<T: EntityEquivalent, const N: usize> From<UniqueEntityEquivalentArray<T, N>> for VecDeque<T> {
//     fn from(value: UniqueEntityEquivalentArray<T, N>) -> Self {
//         VecDeque::from(value.0)
//     }
// }

// -----------------------------------------------------------------------------
// PartialEq

impl<T: EntityEquivalent + PartialEq<U>, U: EntityEquivalent, const N: usize> PartialEq<[U]>
    for UniqueEntityEquivalentArray<T, N>
{
    fn eq(&self, other: &[U]) -> bool {
        self.data_slice().eq(other)
    }
}

// impl<T: EntityEquivalent + PartialEq<U>, U: EntityEquivalent, const N: usize>
//     PartialEq<&UniqueEntityEquivalentSlice<U>> for UniqueEntityEquivalentArray<T, N>
// {
//     fn eq(&self, other: &&UniqueEntityEquivalentSlice<U>) -> bool {
//         self.data_slice().eq(other.data_slice())
//     }
// }

// impl<T: EntityEquivalent + PartialEq<U>, U: EntityEquivalent, const N: usize>
//     PartialEq<UniqueEntityEquivalentSlice<U>> for UniqueEntityEquivalentArray<T, N>
// {
//     fn eq(&self, other: &UniqueEntityEquivalentSlice<U>) -> bool {
//         self.data_slice().eq(other.data_slice())
//     }
// }

// impl<T: PartialEq<U>, U: EntityEquivalent, const N: usize>
//     PartialEq<&UniqueEntityEquivalentArray<U, N>> for Vec<T>
// {
//     fn eq(&self, other: &&UniqueEntityEquivalentArray<U, N>) -> bool {
//         self.eq(&other.0)
//     }
// }

// impl<T: PartialEq<U>, U: EntityEquivalent, const N: usize>
//     PartialEq<&UniqueEntityEquivalentArray<U, N>> for VecDeque<T>
// {
//     fn eq(&self, other: &&UniqueEntityEquivalentArray<U, N>) -> bool {
//         self.eq(&other.0)
//     }
// }

// impl<T: PartialEq<U>, U: EntityEquivalent, const N: usize>
//     PartialEq<&mut UniqueEntityEquivalentArray<U, N>> for VecDeque<T>
// {
//     fn eq(&self, other: &&mut UniqueEntityEquivalentArray<U, N>) -> bool {
//         self.eq(&other.0)
//     }
// }

// impl<T: PartialEq<U>, U: EntityEquivalent, const N: usize>
//     PartialEq<UniqueEntityEquivalentArray<U, N>> for Vec<T>
// {
//     fn eq(&self, other: &UniqueEntityEquivalentArray<U, N>) -> bool {
//         self.eq(&other.0)
//     }
// }

// impl<T: PartialEq<U>, U: EntityEquivalent, const N: usize>
//     PartialEq<UniqueEntityEquivalentArray<U, N>> for VecDeque<T>
// {
//     fn eq(&self, other: &UniqueEntityEquivalentArray<U, N>) -> bool {
//         self.eq(&other.0)
//     }
// }

// -----------------------------------------------------------------------------
// Index and IndexMut

impl<T: EntityEquivalent, const N: usize> Index<(Bound<usize>, Bound<usize>)>
    for UniqueEntityEquivalentArray<T, N>
{
    type Output = UniqueEntityEquivalentSlice<T>;
    fn index(&self, key: (Bound<usize>, Bound<usize>)) -> &Self::Output {
        // SAFETY: All elements in the original slice are unique.
        unsafe { UniqueEntityEquivalentSlice::from_slice_unchecked(self.0.index(key)) }
    }
}

impl<T: EntityEquivalent, const N: usize> Index<Range<usize>>
    for UniqueEntityEquivalentArray<T, N>
{
    type Output = UniqueEntityEquivalentSlice<T>;
    fn index(&self, key: Range<usize>) -> &Self::Output {
        // SAFETY: All elements in the original slice are unique.
        unsafe { UniqueEntityEquivalentSlice::from_slice_unchecked(self.0.index(key)) }
    }
}

impl<T: EntityEquivalent, const N: usize> Index<RangeFrom<usize>>
    for UniqueEntityEquivalentArray<T, N>
{
    type Output = UniqueEntityEquivalentSlice<T>;
    fn index(&self, key: RangeFrom<usize>) -> &Self::Output {
        // SAFETY: All elements in the original slice are unique.
        unsafe { UniqueEntityEquivalentSlice::from_slice_unchecked(self.0.index(key)) }
    }
}

impl<T: EntityEquivalent, const N: usize> Index<RangeFull> for UniqueEntityEquivalentArray<T, N> {
    type Output = UniqueEntityEquivalentSlice<T>;
    fn index(&self, key: RangeFull) -> &Self::Output {
        // SAFETY: All elements in the original slice are unique.
        unsafe { UniqueEntityEquivalentSlice::from_slice_unchecked(self.0.index(key)) }
    }
}

impl<T: EntityEquivalent, const N: usize> Index<RangeInclusive<usize>>
    for UniqueEntityEquivalentArray<T, N>
{
    type Output = UniqueEntityEquivalentSlice<T>;
    fn index(&self, key: RangeInclusive<usize>) -> &Self::Output {
        // SAFETY: All elements in the original slice are unique.
        unsafe { UniqueEntityEquivalentSlice::from_slice_unchecked(self.0.index(key)) }
    }
}

impl<T: EntityEquivalent, const N: usize> Index<RangeTo<usize>>
    for UniqueEntityEquivalentArray<T, N>
{
    type Output = UniqueEntityEquivalentSlice<T>;
    fn index(&self, key: RangeTo<usize>) -> &Self::Output {
        // SAFETY: All elements in the original slice are unique.
        unsafe { UniqueEntityEquivalentSlice::from_slice_unchecked(self.0.index(key)) }
    }
}

impl<T: EntityEquivalent, const N: usize> Index<RangeToInclusive<usize>>
    for UniqueEntityEquivalentArray<T, N>
{
    type Output = UniqueEntityEquivalentSlice<T>;
    fn index(&self, key: RangeToInclusive<usize>) -> &Self::Output {
        // SAFETY: All elements in the original slice are unique.
        unsafe { UniqueEntityEquivalentSlice::from_slice_unchecked(self.0.index(key)) }
    }
}

impl<T: EntityEquivalent, const N: usize> Index<usize> for UniqueEntityEquivalentArray<T, N> {
    type Output = T;
    fn index(&self, key: usize) -> &T {
        self.0.index(key)
    }
}

impl<T: EntityEquivalent, const N: usize> IndexMut<(Bound<usize>, Bound<usize>)>
    for UniqueEntityEquivalentArray<T, N>
{
    fn index_mut(&mut self, key: (Bound<usize>, Bound<usize>)) -> &mut Self::Output {
        // SAFETY: All elements in the original slice are unique.
        unsafe { UniqueEntityEquivalentSlice::from_slice_unchecked_mut(self.0.index_mut(key)) }
    }
}

impl<T: EntityEquivalent, const N: usize> IndexMut<Range<usize>>
    for UniqueEntityEquivalentArray<T, N>
{
    fn index_mut(&mut self, key: Range<usize>) -> &mut Self::Output {
        // SAFETY: All elements in the original slice are unique.
        unsafe { UniqueEntityEquivalentSlice::from_slice_unchecked_mut(self.0.index_mut(key)) }
    }
}

impl<T: EntityEquivalent, const N: usize> IndexMut<RangeFrom<usize>>
    for UniqueEntityEquivalentArray<T, N>
{
    fn index_mut(&mut self, key: RangeFrom<usize>) -> &mut Self::Output {
        // SAFETY: All elements in the original slice are unique.
        unsafe { UniqueEntityEquivalentSlice::from_slice_unchecked_mut(self.0.index_mut(key)) }
    }
}

impl<T: EntityEquivalent, const N: usize> IndexMut<RangeFull>
    for UniqueEntityEquivalentArray<T, N>
{
    fn index_mut(&mut self, key: RangeFull) -> &mut Self::Output {
        // SAFETY: All elements in the original slice are unique.
        unsafe { UniqueEntityEquivalentSlice::from_slice_unchecked_mut(self.0.index_mut(key)) }
    }
}

impl<T: EntityEquivalent, const N: usize> IndexMut<RangeInclusive<usize>>
    for UniqueEntityEquivalentArray<T, N>
{
    fn index_mut(&mut self, key: RangeInclusive<usize>) -> &mut Self::Output {
        // SAFETY: All elements in the original slice are unique.
        unsafe { UniqueEntityEquivalentSlice::from_slice_unchecked_mut(self.0.index_mut(key)) }
    }
}

impl<T: EntityEquivalent, const N: usize> IndexMut<RangeTo<usize>>
    for UniqueEntityEquivalentArray<T, N>
{
    fn index_mut(&mut self, key: RangeTo<usize>) -> &mut Self::Output {
        // SAFETY: All elements in the original slice are unique.
        unsafe { UniqueEntityEquivalentSlice::from_slice_unchecked_mut(self.0.index_mut(key)) }
    }
}

impl<T: EntityEquivalent, const N: usize> IndexMut<RangeToInclusive<usize>>
    for UniqueEntityEquivalentArray<T, N>
{
    fn index_mut(&mut self, key: RangeToInclusive<usize>) -> &mut Self::Output {
        // SAFETY: All elements in the original slice are unique.
        unsafe { UniqueEntityEquivalentSlice::from_slice_unchecked_mut(self.0.index_mut(key)) }
    }
}

// -----------------------------------------------------------------------------
// IntoIter

/// A by-value array iterator.
///
/// Equivalent to [`array::IntoIter`].
pub type IntoIter<const N: usize, T = Entity> = super::UniqueEntityIter<array::IntoIter<T, N>>;

impl<T: EntityEquivalent, const N: usize> IntoIter<N, T> {
    /// Returns an immutable slice of all elements that have not been yielded yet.
    pub fn as_slice(&self) -> &UniqueEntityEquivalentSlice<T> {
        // SAFETY: All elements in the original slice are unique.
        unsafe { UniqueEntityEquivalentSlice::from_slice_unchecked(self.as_inner().as_slice()) }
    }

    /// Returns a mutable slice of all elements that have not been yielded yet.
    pub fn as_mut_slice(&mut self) -> &mut UniqueEntityEquivalentSlice<T> {
        // SAFETY: All elements in the original slice are unique.
        unsafe {
            UniqueEntityEquivalentSlice::from_slice_unchecked_mut(
                self.as_mut_inner().as_mut_slice(),
            )
        }
    }
}
