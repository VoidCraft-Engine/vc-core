#![expect(unsafe_code, reason = "need ptr operation")]

use core::panic::Location;

use vc_ptr::{Ptr, PtrMut};

use super::{ComponentTicksMut, ComponentTicksRef};

use crate::cfg;
use crate::change_detection::{DetectChanges, DetectChangesMut};
use crate::resource::Resource;
use crate::tick::Tick;
use crate::utils::DebugLocation;

// -----------------------------------------------------------------------------
// Res

/// A shared reference to a [`Resource`].
///
/// Implements [`Deref`](core::ops::Deref) and can be used as a regular reference.
///
/// Consumes itself and returns a Rust reference `&T` with the same lifetime via
/// [`into_inner`](Self::into_inner).
///
/// Creates a copy with unchanged lifetime via [`reborrow`](Self::reborrow).
///
/// Obtains a shorter-lived inner reference `&T` via [`Deref::deref`] or [`AsRef::as_ref`].
///
/// Transforms the contained type via [`map`], [`try_map`], or [`filter_map`],
/// e.g., from `Res<'a, (i32, String)>` to `Res<'a, String>`.
///
/// Converts to the generic component reference [`Ref`] via [`From`] or [`Into`].
///
/// [`map`]: Self::map
/// [`try_map`]: Self::try_map
/// [`filter_map`]: Self::filter_map
/// [`Deref::deref`]: core::ops::Deref::deref
pub struct Res<'w, T: ?Sized + Resource> {
    pub(crate) value: &'w T,
    pub(crate) ticks: ComponentTicksRef<'w>,
}

// -----------------------------------------------------------------------------
// ResMut

/// A unique mutable reference to a [`Resource`].
///
/// Implements [`Deref`](core::ops::Deref) and [`DerefMut`](core::ops::DerefMut),
/// and can be used as a regular reference.
///
/// Since we cannot determine which operations modify data, any acquisition of
/// a **mutable reference** sets the internal change flag, marking the resource as
/// changed for subsequent change events.
///
/// Consumes itself and returns a Rust reference `&mut T` with the same lifetime
/// via [`into_inner`](Self::into_inner).
///
/// Creates a shorter-lived copy via [`reborrow`](Self::reborrow). Rust's borrow
/// checker ensures the original reference is inaccessible while the new one exists.
/// This function does not set the change flag.
///
/// Obtains a shorter-lived inner reference `&T` via [`Deref::deref`]/[`AsRef::as_ref`],
/// or `&mut T` via [`DerefMut::deref_mut`]/[`AsMut::as_mut`]. Rust's borrow checker ensures
/// the original reference is inaccessible while the new one exists.
///
/// Transforms the contained type via [`map_unchanged`], [`try_map_unchanged`], or
/// [`filter_map_unchanged`], e.g., from `ResMut<'a, (i32, String)>` to `ResMut<'a, String>`.
/// These functions are assumed to only change the type, not modify data, so they do
/// not set the change flag. Users must ensure they do not modify data within the closure.
/// (Data may be modified through the returned reference, but not within the transformation
/// closure itself.)
///
/// Converts to the shared reference [`Res`] or the generic mutable component reference [`Mut`]
/// via [`From`] or [`Into`].
///
/// [`map_unchanged`]: Self::map_unchanged
/// [`try_map_unchanged`]: Self::try_map_unchanged
/// [`filter_map_unchanged`]: Self::filter_map_unchanged
/// [`Deref::deref`]: core::ops::Deref::deref
/// [`DerefMut::deref_mut`]: core::ops::DerefMut::deref_mut
pub struct ResMut<'w, T: ?Sized + Resource> {
    pub(crate) value: &'w mut T,
    pub(crate) ticks: ComponentTicksMut<'w>,
}

// -----------------------------------------------------------------------------
// NonSend

/// A shared reference to a non-[`Send`] resource/component.
///
/// Implements [`Deref`](core::ops::Deref) and can be used as a regular reference.
///
/// Consumes itself and returns a Rust reference `&T` with the same lifetime via
/// [`into_inner`](Self::into_inner).
///
/// Creates a copy with unchanged lifetime via [`reborrow`](Self::reborrow).
///
/// Obtains a shorter-lived inner reference `&T` via [`Deref::deref`] or [`AsRef::as_ref`].
///
/// Transforms the contained type via [`map`], [`try_map`], or [`filter_map`],
/// e.g., from `NonSend<'a, (i32, String)>` to `NonSend<'a, String>`.
///
/// Converts to the generic component reference [`Ref`] via [`From`] or [`Into`].
/// Note: Thread allocation is handled internally by the ECS scheduler based on function signatures,
/// making this conversion safe within functions.
///
/// [`map`]: Self::map
/// [`try_map`]: Self::try_map
/// [`filter_map`]: Self::filter_map
/// [`Deref::deref`]: core::ops::Deref::deref
pub struct NonSend<'w, T: ?Sized + 'static> {
    pub(crate) value: &'w T,
    pub(crate) ticks: ComponentTicksRef<'w>,
}

// -----------------------------------------------------------------------------
// NonSendMut

/// A unique mutable reference to a non-[`Send`] resource/component.
///
/// Implements [`Deref`](core::ops::Deref) and [`DerefMut`](core::ops::DerefMut),
/// and can be used as a regular reference.
///
/// Since we cannot determine which operations modify data, any acquisition of
/// a **mutable reference** sets the internal change flag, marking the resource as
/// changed for subsequent change events.
///
/// Consumes itself and returns a Rust reference `&mut T` with the same lifetime
/// via [`into_inner`](Self::into_inner).
///
/// Creates a shorter-lived copy via [`reborrow`](Self::reborrow). Rust's borrow
/// checker ensures the original reference is inaccessible while the new one exists.
/// This function does not set the change flag.
///
/// Obtains a shorter-lived inner reference `&T` via [`Deref::deref`]/[`AsRef::as_ref`],
/// or `&mut T` via [`DerefMut::deref_mut`]/[`AsMut::as_mut`]. Rust's borrow checker ensures
/// the original reference is inaccessible while the new one exists.
///
/// Transforms the contained type via [`map_unchanged`], [`try_map_unchanged`], or
/// [`filter_map_unchanged`], e.g., from `NonSendMut<'a, (i32, String)>` to `NonSendMut<'a, String>`.
/// These functions are assumed to only change the type, not modify data, so they do
/// not set the change flag. Users must ensure they do not modify data within the closure.
/// (Data may be modified through the returned reference, but not within the transformation
/// closure itself.)
///
/// Converts to the shared reference [`NonSend`] or the generic mutable component reference [`Mut`]
/// via [`From`] or [`Into`].
/// Note: Thread allocation is handled internally by the ECS scheduler based on function signatures,
/// making this conversion safe within functions.
///
/// [`map_unchanged`]: Self::map_unchanged
/// [`try_map_unchanged`]: Self::try_map_unchanged
/// [`filter_map_unchanged`]: Self::filter_map_unchanged
/// [`Deref::deref`]: core::ops::Deref::deref
/// [`DerefMut::deref_mut`]: core::ops::DerefMut::deref_mut
pub struct NonSendMut<'w, T: ?Sized + 'static> {
    pub(crate) value: &'w mut T,
    pub(crate) ticks: ComponentTicksMut<'w>,
}

// -----------------------------------------------------------------------------
// Ref

/// A shared reference to a resource/component with change detection.
///
/// Implements [`Deref`](core::ops::Deref) and can be used as a regular reference.
///
/// Consumes itself and returns a Rust reference `&T` with the same lifetime via
/// [`into_inner`](Self::into_inner).
///
/// Creates a copy with unchanged lifetime via [`reborrow`](Self::reborrow).
///
/// Obtains a shorter-lived inner reference `&T` via [`Deref::deref`] or [`AsRef::as_ref`].
///
/// Transforms the contained type via [`map`], [`try_map`], or [`filter_map`],
/// e.g., from `Ref<'a, (i32, String)>` to `Ref<'a, String>`.
///
/// [`map`]: Self::map
/// [`try_map`]: Self::try_map
/// [`filter_map`]: Self::filter_map
/// [`Deref::deref`]: core::ops::Deref::deref
pub struct Ref<'w, T: ?Sized> {
    pub(crate) value: &'w T,
    pub(crate) ticks: ComponentTicksRef<'w>,
}

// -----------------------------------------------------------------------------
// Mut

/// A unique mutable reference to a resource/component with change detection.
///
/// Implements [`Deref`](core::ops::Deref) and [`DerefMut`](core::ops::DerefMut),
/// and can be used as a regular reference.
///
/// Since we cannot determine which operations modify data, any acquisition of
/// a **mutable reference** sets the internal change flag, marking the resource as
/// changed for subsequent change events.
///
/// Consumes itself and returns a Rust reference `&mut T` with the same lifetime
/// via [`into_inner`](Self::into_inner).
///
/// Creates a shorter-lived copy via [`reborrow`](Self::reborrow). Rust's borrow
/// checker ensures the original reference is inaccessible while the new one exists.
/// This function does not set the change flag.
///
/// Obtains a shorter-lived inner reference `&T` via [`Deref::deref`]/[`AsRef::as_ref`],
/// or `&mut T` via [`DerefMut::deref_mut`]/[`AsMut::as_mut`]. Rust's borrow checker ensures
/// the original reference is inaccessible while the new one exists.
///
/// Transforms the contained type via [`map_unchanged`], [`try_map_unchanged`], or
/// [`filter_map_unchanged`], e.g., from `Mut<'a, (i32, String)>` to `Mut<'a, String>`.
/// These functions are assumed to only change the type, not modify data, so they do
/// not set the change flag. Users must ensure they do not modify data within the closure.
/// (Data may be modified through the returned reference, but not within the transformation
/// closure itself.)
///
/// Converts to the shared reference [`Ref`] via [`From`] or [`Into`].
///
/// [`map_unchanged`]: Self::map_unchanged
/// [`try_map_unchanged`]: Self::try_map_unchanged
/// [`filter_map_unchanged`]: Self::filter_map_unchanged
/// [`Deref::deref`]: core::ops::Deref::deref
/// [`DerefMut::deref_mut`]: core::ops::DerefMut::deref_mut
pub struct Mut<'w, T: ?Sized> {
    pub(crate) value: &'w mut T,
    pub(crate) ticks: ComponentTicksMut<'w>,
}

// -----------------------------------------------------------------------------
// MutUntyped

/// A type-erased unique mutable reference.
///
/// Must be converted to a typed [`Mut`] via [`with_type`] or [`map_unchanged`].
///
/// Since we cannot determine which operations modify data, acquiring the inner mutable pointer
/// triggers change detection, even if data is not actually modified through the pointer.
///
/// [`with_type`]: Self::with_type
/// [`map_unchanged`]: Self::map_unchanged
pub struct MutUntyped<'w> {
    pub(crate) value: PtrMut<'w>,
    pub(crate) ticks: ComponentTicksMut<'w>,
}

// -----------------------------------------------------------------------------
// From

impl<'w, T: ?Sized + Resource> From<ResMut<'w, T>> for Mut<'w, T> {
    #[inline(always)]
    fn from(other: ResMut<'w, T>) -> Mut<'w, T> {
        Mut {
            value: other.value,
            ticks: other.ticks,
        }
    }
}

impl<'w, T: ?Sized + Resource> From<ResMut<'w, T>> for Res<'w, T> {
    #[inline(always)]
    fn from(res: ResMut<'w, T>) -> Self {
        Self {
            value: res.value,
            ticks: res.ticks.into(),
        }
    }
}

impl<'w, T: ?Sized + Resource> From<Res<'w, T>> for Ref<'w, T> {
    #[inline(always)]
    fn from(res: Res<'w, T>) -> Self {
        Self {
            value: res.value,
            ticks: res.ticks,
        }
    }
}

impl<'w, T: ?Sized + 'static> From<NonSendMut<'w, T>> for Mut<'w, T> {
    #[inline(always)]
    fn from(other: NonSendMut<'w, T>) -> Mut<'w, T> {
        Mut {
            value: other.value,
            ticks: other.ticks,
        }
    }
}

impl<'w, T: ?Sized + 'static> From<NonSendMut<'w, T>> for NonSend<'w, T> {
    #[inline(always)]
    fn from(other: NonSendMut<'w, T>) -> Self {
        Self {
            value: other.value,
            ticks: other.ticks.into(),
        }
    }
}

impl<'w, T: ?Sized + 'static> From<NonSend<'w, T>> for Ref<'w, T> {
    #[inline(always)]
    fn from(other: NonSend<'w, T>) -> Ref<'w, T> {
        Ref {
            value: other.value,
            ticks: other.ticks,
        }
    }
}

impl<'w, T: ?Sized> From<Mut<'w, T>> for Ref<'w, T> {
    #[inline(always)]
    fn from(mut_ref: Mut<'w, T>) -> Self {
        Self {
            value: mut_ref.value,
            ticks: mut_ref.ticks.into(),
        }
    }
}

impl<'w, T: ?Sized> From<Mut<'w, T>> for MutUntyped<'w> {
    #[inline(always)]
    fn from(value: Mut<'w, T>) -> Self {
        MutUntyped {
            value: value.value.into(),
            ticks: value.ticks,
        }
    }
}

// -----------------------------------------------------------------------------
// IntoIterator

impl<'w, 'a, T: Resource> IntoIterator for &'a mut ResMut<'w, T>
where
    &'a mut T: IntoIterator,
{
    type Item = <&'a mut T as IntoIterator>::Item;
    type IntoIter = <&'a mut T as IntoIterator>::IntoIter;

    #[inline(always)]
    fn into_iter(self) -> Self::IntoIter {
        self.set_changed();
        self.value.into_iter()
    }
}

impl<'w, 'a, T: Resource> IntoIterator for &'a ResMut<'w, T>
where
    &'a T: IntoIterator,
{
    type Item = <&'a T as IntoIterator>::Item;
    type IntoIter = <&'a T as IntoIterator>::IntoIter;

    #[inline(always)]
    fn into_iter(self) -> Self::IntoIter {
        self.value.into_iter()
    }
}

impl<'w, 'a, T: Resource> IntoIterator for &'a Res<'w, T>
where
    &'a T: IntoIterator,
{
    type Item = <&'a T as IntoIterator>::Item;
    type IntoIter = <&'a T as IntoIterator>::IntoIter;

    #[inline(always)]
    fn into_iter(self) -> Self::IntoIter {
        self.value.into_iter()
    }
}

impl<'w, 'a, T> IntoIterator for &'a mut Mut<'w, T>
where
    &'a mut T: IntoIterator,
{
    type Item = <&'a mut T as IntoIterator>::Item;
    type IntoIter = <&'a mut T as IntoIterator>::IntoIter;

    #[inline(always)]
    fn into_iter(self) -> Self::IntoIter {
        self.set_changed();
        self.value.into_iter()
    }
}

impl<'w, 'a, T> IntoIterator for &'a Mut<'w, T>
where
    &'a T: IntoIterator,
{
    type Item = <&'a T as IntoIterator>::Item;
    type IntoIter = <&'a T as IntoIterator>::IntoIter;
    #[inline(always)]
    fn into_iter(self) -> Self::IntoIter {
        self.value.into_iter()
    }
}

impl<'w, 'a, T> IntoIterator for &'a Ref<'w, T>
where
    &'a T: IntoIterator,
{
    type Item = <&'a T as IntoIterator>::Item;
    type IntoIter = <&'a T as IntoIterator>::IntoIter;
    #[inline(always)]
    fn into_iter(self) -> Self::IntoIter {
        self.value.into_iter()
    }
}

// -----------------------------------------------------------------------------
// impl_debug

macro_rules! impl_debug {
    ($name:ident < $( $generics:tt ),+ > $($traits:ident)?) => {
        impl<$($generics),* : ?Sized $(+ $traits)?> ::core::fmt::Debug for $name<$($generics),*>
            where T: ::core::fmt::Debug
        {
            fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
                f.debug_tuple(stringify!($name))
                    .field(&self.value)
                    .finish()
            }
        }
    };
}

impl_debug!(Mut<'w, T>);
impl_debug!(Ref<'w, T>);
impl_debug!(NonSendMut<'w, T>);
impl_debug!(NonSend<'w, T>);
impl_debug!(ResMut<'w, T> Resource);
impl_debug!(Res<'w, T> Resource);

// -----------------------------------------------------------------------------
// impl_ref_methods

macro_rules! impl_ref_methods {
    ($name:ident < $( $generics:tt ),+ >, $target:ty, $($traits:ident)?) => {
        impl<$($generics),* : ?Sized $(+ $traits)?> $name<$($generics),*> {
            /// Consumes self and returns the inner reference `&T` with the same lifetime.
            #[inline(always)]
            pub fn into_inner(self) -> &'w $target {
                self.value
            }

            /// Creates a copy with the same lifetime.
            ///
            /// Since this is a shared reference, the original and copy do not interfere.
            #[inline]
            pub fn reborrow(&self) -> Self {
                Self {
                    value: self.value,
                    ticks: self.ticks.clone(),
                }
            }

            /// Transforms the reference type via a function, preserving the lifetime.
            ///
            /// Returns the generic [`Ref`] container.
            #[inline(always)]
            pub fn map<U: ?Sized>(
                self,
                f: impl FnOnce(&$target) -> &U,
            ) -> Ref<'w, U> {
                Ref {
                    value: f(self.value),
                    ticks: self.ticks,
                }
            }

            /// Transforms the reference type via a function, preserving the lifetime.
            ///
            /// Returns the generic [`Ref`] container, or `None` if the transformation fails.
            #[inline]
            pub fn filter_map<U: ?Sized>(
                self,
                f: impl FnOnce(&$target) -> Option<&U>,
            ) -> Option<Ref<'w, U>> {
                let value = f(self.value);
                value.map(|value| Ref {
                    value,
                    ticks: self.ticks,
                })
            }

            /// Transforms the reference type via a function, preserving the lifetime.
            ///
            /// Returns the generic [`Ref`] container, or an error if the transformation fails.
            #[inline]
            pub fn try_map<U: ?Sized, E>(
                self,
                f: impl FnOnce(&$target) -> Result<&U, E>,
            ) -> Result<Ref<'w, U>, E> {
                let value = f(self.value);
                value.map(|value| Ref {
                    value,
                    ticks: self.ticks,
                })
            }

            /// Dereferences the inner type, e.g., converts `Ref<'a, Box<T>>` to `Ref<'a, T>`.
            ///
            /// Returns the generic [`Ref`] container.
            #[inline]
            pub fn into_deref(self) -> Ref<'w, <$target as ::core::ops::Deref>::Target>
                where $target: ::core::ops::Deref
            {
                self.map(|v| v.deref())
            }
        }
    };
}

impl_ref_methods!(Res<'w, T>, T, Resource);
impl_ref_methods!(NonSend<'w, T>, T,);
impl_ref_methods!(Ref<'w, T>, T,);

// -----------------------------------------------------------------------------
// impl_mut_methods

macro_rules! impl_mut_methods {
    ($name:ident < $( $generics:tt ),+ >, $target:ty, $($traits:ident)?) => {
        impl<$($generics),* : ?Sized $(+ $traits)?> $name<$($generics),*> {
            /// Consumes self and returns the inner reference `&mut T` with the
            /// same lifetime, marking the target as changed.
            #[inline]
            pub fn into_inner(mut self) -> &'w mut $target {
                self.set_changed();
                self.value
            }

            /// Returns a shorter-lived version of self, with borrow checker guarantees.
            ///
            /// This function does not mark the target as changed.
            pub fn reborrow(&mut self) -> $name<'_, $target> {
                $name {
                    value: self.value,
                    ticks: ComponentTicksMut {
                        added: self.ticks.added,
                        changed: self.ticks.changed,
                        changed_by: self.ticks.changed_by.as_deref_mut(),
                        last_run: self.ticks.last_run,
                        this_run: self.ticks.this_run,
                    },
                }
            }

            /// Transforms the reference type via a function, preserving the lifetime.
            ///
            /// Returns the generic [`Mut`] container.
            ///
            /// This function is assumed to only change the type, not modify data.
            /// Modifying data through the mutable reference in the closure is undefined behavior
            /// (data may be modified without triggering change events).
            #[inline(always)]
            pub fn map_unchanged<U: ?Sized>(
                self,
                f: impl FnOnce(&mut $target) -> &mut U,
            ) -> Mut<'w, U> {
                Mut {
                    value: f(self.value),
                    ticks: self.ticks,
                }
            }

            /// Transforms the reference type via a function, preserving the lifetime.
            ///
            /// Returns the generic [`Mut`] container, or `None` if the transformation fails.
            ///
            /// This function is assumed to only change the type, not modify data.
            /// Modifying data through the mutable reference in the closure is undefined behavior
            /// (data may be modified without triggering change events).
            #[inline]
            pub fn filter_map_unchanged<U: ?Sized>(
                self,
                f: impl FnOnce(&mut $target) -> Option<&mut U>,
            ) -> Option<Mut<'w, U>> {
                let value = f(self.value);
                value.map(|value| Mut {
                    value,
                    ticks: self.ticks,
                })
            }

            /// Transforms the reference type via a function, preserving the lifetime.
            ///
            /// Returns the generic [`Mut`] container, or an error if the transformation fails.
            ///
            /// This function is assumed to only change the type, not modify data.
            /// Modifying data through the mutable reference in the closure is undefined behavior
            /// (data may be modified without triggering change events).
            #[inline]
            pub fn try_map_unchanged<U: ?Sized, E>(
                self,
                f: impl FnOnce(&mut $target) -> Result<&mut U, E>,
            ) -> Result<Mut<'w, U>, E> {
                let value = f(self.value);
                value.map(|value| Mut {
                    value,
                    ticks: self.ticks,
                })
            }

            /// Dereferences the inner type, e.g., converts `Mut<'a, Box<T>>` to `Mut<'a, T>`.
            ///
            /// Returns the generic [`Mut`] container.
            ///
            /// This function does not set the change flag.
            #[inline]
            pub fn into_deref_mut(self) -> Mut<'w, <$target as ::core::ops::Deref>::Target>
                where $target: ::core::ops::DerefMut
            {
                self.map_unchanged(|v| v.deref_mut())
            }
        }
    };
}

impl_mut_methods!(ResMut<'w, T>, T, Resource);
impl_mut_methods!(NonSendMut<'w, T>, T,);
impl_mut_methods!(Mut<'w, T>, T,);

// -----------------------------------------------------------------------------
// impl_change_detection_and_deref

macro_rules! impl_change_detection_and_deref {
    ($name:ident < $( $generics:tt ),+ >, $target:ty, $($traits:ident)?) => {
        impl<$($generics),* : ?Sized $(+ $traits)?> DetectChanges for $name<$($generics),*> {
            #[inline]
            fn is_added(&self) -> bool {
                self.ticks
                    .added
                    .is_newer_than(self.ticks.last_run, self.ticks.this_run)
            }

            #[inline]
            fn is_changed(&self) -> bool {
                self.ticks
                    .changed
                    .is_newer_than(self.ticks.last_run, self.ticks.this_run)
            }

            #[inline(always)]
            fn changed_tick(&self) -> Tick {
                *self.ticks.changed
            }

            #[inline(always)]
            fn added_tick(&self) -> Tick {
                *self.ticks.added
            }

            #[inline(always)]
            fn changed_by(&self) -> DebugLocation {
                self.ticks.changed_by.copied()
            }
        }

        impl<$($generics),*: ?Sized $(+ $traits)?> ::core::ops::Deref for $name<$($generics),*> {
            type Target = $target;

            #[inline(always)]
            fn deref(&self) -> &Self::Target {
                self.value
            }
        }

        impl<$($generics),* $(: $traits)?> AsRef<$target> for $name<$($generics),*> {
            #[inline(always)]
            fn as_ref(&self) -> &$target {
                self.value
            }
        }
    }
}

impl_change_detection_and_deref!(Res<'w, T>, T, Resource);
impl_change_detection_and_deref!(ResMut<'w, T>, T, Resource);
impl_change_detection_and_deref!(NonSend<'w, T>, T,);
impl_change_detection_and_deref!(NonSendMut<'w, T>, T,);
impl_change_detection_and_deref!(Ref<'w, T>, T,);
impl_change_detection_and_deref!(Mut<'w, T>, T,);

// -----------------------------------------------------------------------------
// impl_change_detection_mut_and_deref_mut

macro_rules! impl_change_detection_mut_and_deref_mut {
    ($name:ident < $( $generics:tt ),+ >, $target:ty, $($traits:ident)?) => {
        impl<$($generics),* : ?Sized $(+ $traits)?> DetectChangesMut for $name<$($generics),*> {
            type Inner = $target;

            #[inline(always)]
            #[cfg_attr(any(debug_assertions, feature = "debug"), track_caller)]
            fn set_changed(&mut self) {
                *self.ticks.changed = self.ticks.this_run;
                cfg::debug!{ self.ticks.changed_by.assign(DebugLocation::caller()); }
            }

            #[inline(always)]
            #[cfg_attr(any(debug_assertions, feature = "debug"), track_caller)]
            fn set_added(&mut self) {
                *self.ticks.changed = self.ticks.this_run;
                *self.ticks.added = self.ticks.this_run;
                cfg::debug!{ self.ticks.changed_by.assign(DebugLocation::caller()); }
            }

            #[inline(always)]
            #[cfg_attr(any(debug_assertions, feature = "debug"), track_caller)]
            fn set_changed_with(&mut self, changed_tick: Tick) {
                *self.ticks.changed = changed_tick;
                cfg::debug!{ self.ticks.changed_by.assign(DebugLocation::caller()); }
            }

            #[inline(always)]
            #[cfg_attr(any(debug_assertions, feature = "debug"), track_caller)]
            fn set_added_with(&mut self, added_tick: Tick) {
                *self.ticks.added = added_tick;
                *self.ticks.changed = added_tick;
                cfg::debug!{ self.ticks.changed_by.assign(DebugLocation::caller()); }
            }

            #[inline(always)]
            fn bypass_change_detection(&mut self) -> &mut Self::Inner {
                self.value
            }
        }

        impl<$($generics),* : ?Sized $(+ $traits)?> ::core::ops::DerefMut for $name<$($generics),*> {
            #[inline(always)]
            #[cfg_attr(any(debug_assertions, feature = "debug"), track_caller)]
            fn deref_mut(&mut self) -> &mut Self::Target {
                *self.ticks.changed = self.ticks.this_run;
                cfg::debug!{ self.ticks.changed_by.assign(DebugLocation::caller()); }
                self.value
            }
        }

        impl<$($generics),* $(: $traits)?> AsMut<$target> for $name<$($generics),*> {
            #[inline(always)]
            #[cfg_attr(any(debug_assertions, feature = "debug"), track_caller)]
            fn as_mut(&mut self) -> &mut $target {
                *self.ticks.changed = self.ticks.this_run;
                cfg::debug!{ self.ticks.changed_by.assign(DebugLocation::caller()); }
                self.value
            }
        }
    };
}

impl_change_detection_mut_and_deref_mut!(Mut<'w, T>, T,);
impl_change_detection_mut_and_deref_mut!(ResMut<'w, T>, T, Resource);
impl_change_detection_mut_and_deref_mut!(NonSendMut<'w, T>, T,);

// -----------------------------------------------------------------------------
// MutUntyped : Method Implementation

impl<'w> MutUntyped<'w> {
    /// Consumes self and returns the inner [`PtrMut`].
    ///
    /// Marks the target as `changed` since a mutable handle is returned.
    #[inline(always)]
    pub fn into_inner(mut self) -> PtrMut<'w> {
        self.set_changed();
        self.value
    }

    /// Returns a shorter-lived version of self.
    ///
    /// This function does not set the change flag.
    #[inline(always)]
    pub fn reborrow(&mut self) -> MutUntyped<'_> {
        MutUntyped {
            value: self.value.reborrow(),
            ticks: ComponentTicksMut {
                added: self.ticks.added,
                changed: self.ticks.changed,
                changed_by: self.ticks.changed_by.as_deref_mut(),
                last_run: self.ticks.last_run,
                this_run: self.ticks.this_run,
            },
        }
    }

    /// Checks whether this value has changed since the given tick.
    #[inline]
    pub fn has_changed_since(&self, tick: Tick) -> bool {
        self.ticks.changed.is_newer_than(tick, self.ticks.this_run)
    }

    /// Returns a shorter-lived mutable pointer to the inner value.
    ///
    /// Marks the target as `changed` since a mutable handle is returned.
    #[inline(always)]
    pub fn as_ptr_mut(&mut self) -> PtrMut<'_> {
        self.set_changed();
        self.value.reborrow()
    }

    /// Returns a shorter-lived immutable pointer to the inner value.
    ///
    /// This function does not set the change flag.
    #[inline(always)]
    pub fn as_ptr(&self) -> Ptr<'_> {
        self.value.borrow()
    }

    /// Converts self to a [`Mut`] by specifying the reference type via a function.
    ///
    /// This function is assumed to only change the type, not modify data.
    /// Modifying data through the mutable pointer in the closure is undefined behavior
    /// (data may be modified without triggering change events).
    ///
    /// Consider using [`with_type`](Self::with_type) instead for `Sized` types without
    /// complex operations.
    #[inline(always)]
    pub fn map_unchanged<T: ?Sized>(self, f: impl FnOnce(PtrMut<'w>) -> &'w mut T) -> Mut<'w, T> {
        Mut {
            value: f(self.value),
            ticks: self.ticks,
        }
    }

    /// Specifies the reference type and converts self to a [`Mut`].
    ///
    /// This function does not set the change flag.
    ///
    /// Only works for `Sized` types. Use [`map_unchanged`](Self::map_unchanged) for
    /// `!Sized` types.
    ///
    /// # Safety
    ///
    /// `T` must be the erased pointee type for this [`MutUntyped`].
    #[inline(always)]
    pub unsafe fn with_type<T>(self) -> Mut<'w, T> {
        self.value.debug_assert_aligned::<T>();
        Mut {
            value: unsafe { self.value.consume() },
            ticks: self.ticks,
        }
    }
}

impl<'w> DetectChanges for MutUntyped<'w> {
    #[inline]
    fn is_added(&self) -> bool {
        self.ticks
            .added
            .is_newer_than(self.ticks.last_run, self.ticks.this_run)
    }

    #[inline]
    fn is_changed(&self) -> bool {
        self.ticks
            .changed
            .is_newer_than(self.ticks.last_run, self.ticks.this_run)
    }

    #[inline(always)]
    fn added_tick(&self) -> Tick {
        *self.ticks.added
    }

    #[inline(always)]
    fn changed_tick(&self) -> Tick {
        *self.ticks.changed
    }

    #[inline(always)]
    fn changed_by(&self) -> DebugLocation {
        self.ticks.changed_by.copied()
    }
}

impl<'w> DetectChangesMut for MutUntyped<'w> {
    type Inner = PtrMut<'w>;

    #[inline(always)]
    #[cfg_attr(any(debug_assertions, feature = "debug"), track_caller)]
    fn set_changed(&mut self) {
        *self.ticks.changed = self.ticks.this_run;
        cfg::debug! { self.ticks.changed_by.assign(DebugLocation::caller()); }
    }

    #[inline(always)]
    #[cfg_attr(any(debug_assertions, feature = "debug"), track_caller)]
    fn set_added(&mut self) {
        *self.ticks.changed = self.ticks.this_run;
        *self.ticks.added = self.ticks.this_run;
        cfg::debug! { self.ticks.changed_by.assign(DebugLocation::caller()); }
    }

    #[inline(always)]
    #[cfg_attr(any(debug_assertions, feature = "debug"), track_caller)]
    fn set_changed_with(&mut self, last_changed: Tick) {
        *self.ticks.changed = last_changed;
        cfg::debug! { self.ticks.changed_by.assign(DebugLocation::caller()); }
    }

    #[inline(always)]
    #[cfg_attr(any(debug_assertions, feature = "debug"), track_caller)]
    fn set_added_with(&mut self, last_added: Tick) {
        *self.ticks.added = last_added;
        *self.ticks.changed = last_added;
        cfg::debug! { self.ticks.changed_by.assign(DebugLocation::caller()); }
    }

    #[inline(always)]
    fn bypass_change_detection(&mut self) -> &mut Self::Inner {
        &mut self.value
    }
}

impl core::fmt::Debug for MutUntyped<'_> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("MutUntyped")
            .field(&self.value.as_ptr())
            .finish()
    }
}

// -----------------------------------------------------------------------------
// Internal API

impl<'w, T: ?Sized> Ref<'w, T> {
    /// Internal function to create a new [`Ref`].
    ///
    /// Not intended for public use.
    #[inline(always)]
    pub const fn new(
        value: &'w T,
        added: &'w Tick,
        changed: &'w Tick,
        last_run: Tick,
        this_run: Tick,
        caller: DebugLocation<&'w &'static Location<'static>>,
    ) -> Ref<'w, T> {
        Ref {
            value,
            ticks: ComponentTicksRef {
                added,
                changed,
                changed_by: caller,
                last_run,
                this_run,
            },
        }
    }

    /// Internal function to set the ticks when this [`Ref`] is used by a system.
    ///
    /// Not intended for public use.
    #[inline(always)]
    pub const fn set_ticks(&mut self, last_run: Tick, this_run: Tick) {
        self.ticks.last_run = last_run;
        self.ticks.this_run = this_run;
    }
}

impl<'w, T: ?Sized> Mut<'w, T> {
    /// Internal function to create a new [`Mut`].
    ///
    /// Not intended for public use.
    #[inline(always)]
    pub const fn new(
        value: &'w mut T,
        added: &'w mut Tick,
        last_changed: &'w mut Tick,
        last_run: Tick,
        this_run: Tick,
        caller: DebugLocation<&'w mut &'static Location<'static>>,
    ) -> Self {
        Self {
            value,
            ticks: ComponentTicksMut {
                added,
                changed: last_changed,
                changed_by: caller,
                last_run,
                this_run,
            },
        }
    }

    /// Internal function to set the ticks when this [`Mut`] is used by a system.
    ///
    /// Not intended for public use.
    #[inline(always)]
    pub const fn set_ticks(&mut self, last_run: Tick, this_run: Tick) {
        self.ticks.last_run = last_run;
        self.ticks.this_run = this_run;
    }
}
