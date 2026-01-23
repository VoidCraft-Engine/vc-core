use core::fmt;
use core::hash::Hash;
use core::ops::{Deref, DerefMut};

use crate::cfg;

cfg::debug! {
    if { use core::panic::Location; }
    else { use core::marker::PhantomData; }
}

// -----------------------------------------------------------------------------
// DebugLocation

#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq, PartialOrd, Ord)]
pub struct DebugLocation<T: ?Sized = &'static Location<'static>>(
    #[cfg(any(debug_assertions, feature = "debug"))] T,
    #[cfg(not(any(debug_assertions, feature = "debug")))] PhantomData<T>,
);

// -----------------------------------------------------------------------------
// Traits

vc_reflect::derive::impl_reflect_opaque! {
    (in vc_ecs::utils as DebugLocation)
    DebugLocation<T: Clone + PartialEq + PartialOrd  + Hash + fmt::Debug>
    (clone, hash, debug, partial_eq, partial_cmp)
}

impl<T: fmt::Display> fmt::Display for DebugLocation<T> {
    #[inline(always)]
    fn fmt(&self, _f: &mut fmt::Formatter<'_>) -> fmt::Result {
        cfg::debug! { self.0.fmt(_f)?; }
        Ok(())
    }
}

// -----------------------------------------------------------------------------
// Implementation

impl DebugLocation {
    /// Returns the source location of the caller of this function.
    ///
    /// If that function's caller is annotated then its call location will be returned,
    /// and so on up the stack to the first call within a non-tracked function body.
    #[inline(always)]
    #[cfg_attr(any(debug_assertions, feature = "debug"), track_caller)]
    pub const fn caller() -> Self {
        cfg::debug! {
            if {  Self(Location::caller()) }
            else { Self(PhantomData) }
        }
    }
}

impl<T> DebugLocation<T> {
    #[inline(always)]
    pub fn new_with(_f: impl FnOnce() -> T) -> Self {
        cfg::debug! {
            if { Self(_f()) } else { Self(PhantomData) }
        }
    }

    #[inline(always)]
    pub fn map<U>(self, _f: impl FnOnce(T) -> U) -> DebugLocation<U> {
        cfg::debug! {
            if { DebugLocation(_f(self.0)) }
            else { DebugLocation(PhantomData) }
        }
    }

    #[inline(always)]
    pub fn zip<U>(self, _other: DebugLocation<U>) -> DebugLocation<(T, U)> {
        cfg::debug! {
            if { DebugLocation((self.0, _other.0)) }
            else { DebugLocation(PhantomData) }
        }
    }

    #[inline]
    pub fn unwrap_or_default(self) -> T
    where
        T: Default,
    {
        self.into_option().unwrap_or_default()
    }

    #[inline(always)]
    pub fn into_option(self) -> Option<T> {
        cfg::debug! {
            if { Some(self.0) } else { None }
        }
    }
}

impl<T> DebugLocation<Option<T>> {
    #[cfg_attr(not(any(debug_assertions, feature = "debug")), inline(always))]
    pub fn untranspose(_f: impl FnOnce() -> Option<DebugLocation<T>>) -> Self {
        cfg::debug! {
            if { Self(_f().map(|value| value.0)) }
            else { Self(PhantomData) }
        }
    }

    #[cfg_attr(not(any(debug_assertions, feature = "debug")), inline(always))]
    pub fn transpose(self) -> Option<DebugLocation<T>> {
        cfg::debug! {
            if { self.0.map(|v|DebugLocation(v)) }
            else { Some(DebugLocation(PhantomData)) }
        }
    }
}

impl<T> DebugLocation<&T> {
    /// Maps an `DebugLocation<&T>` to an `DebugLocation<T>` by copying the contents.
    #[inline(always)]
    pub const fn copied(&self) -> DebugLocation<T>
    where
        T: Copy,
    {
        cfg::debug! {
            if { DebugLocation(*self.0) }
            else { DebugLocation(PhantomData) }
        }
    }
}

impl<T> DebugLocation<&mut T> {
    /// Maps an `DebugLocation<&mut T>` to an `DebugLocation<T>` by copying the contents.
    #[inline(always)]
    pub const fn copied(&self) -> DebugLocation<T>
    where
        T: Copy,
    {
        cfg::debug! {
            if { DebugLocation(*self.0) }
            else { DebugLocation(PhantomData) }
        }
    }

    /// Assigns the contents of an `DebugLocation<T>` to an `DebugLocation<&mut T>`.
    #[inline(always)]
    pub fn assign(&mut self, _value: DebugLocation<T>) {
        cfg::debug! {
            *self.0 = _value.0;
        }
    }
}

impl<T: ?Sized> DebugLocation<T> {
    /// Converts from `&DebugLocation<T>` to `DebugLocation<&T>`.
    #[inline(always)]
    pub const fn as_ref(&self) -> DebugLocation<&T> {
        cfg::debug! {
            if { DebugLocation(&self.0) }
            else { DebugLocation(PhantomData) }
        }
    }

    /// Converts from `&mut DebugLocation<T>` to `DebugLocation<&mut T>`.
    #[inline(always)]
    pub const fn as_mut(&mut self) -> DebugLocation<&mut T> {
        cfg::debug! {
            if { DebugLocation(&mut self.0) }
            else { DebugLocation(PhantomData) }
        }
    }

    /// Converts from `&DebugLocation<T>` to `DebugLocation<&T::Target>`.
    #[inline(always)]
    pub fn as_deref(&self) -> DebugLocation<&T::Target>
    where
        T: Deref,
    {
        cfg::debug! {
            if {  DebugLocation(&*self.0) }
            else { DebugLocation(PhantomData) }
        }
    }

    /// Converts from `&mut DebugLocation<T>` to `DebugLocation<&mut T::Target>`.
    #[inline(always)]
    pub fn as_deref_mut(&mut self) -> DebugLocation<&mut T::Target>
    where
        T: DerefMut,
    {
        cfg::debug! {
            if {  DebugLocation(&mut *self.0) }
            else { DebugLocation(PhantomData) }
        }
    }
}
