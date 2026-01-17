#![expect(unsafe_code, reason = "need ptr operation")]

use core::panic::Location;

use vc_ptr::{Ptr, PtrMut};

use super::{ComponentTicksMut, ComponentTicksRef};

use crate::change_detection::{DetectChanges, DetectChangesMut};
use crate::resource::Resource;
use crate::tick::Tick;
use crate::utils::DebugLocation;

// -----------------------------------------------------------------------------
// Res

pub struct Res<'w, T: ?Sized + Resource> {
    pub(crate) value: &'w T,
    pub(crate) ticks: ComponentTicksRef<'w>,
}

// -----------------------------------------------------------------------------
// ResMut

pub struct ResMut<'w, T: ?Sized + Resource> {
    pub(crate) value: &'w mut T,
    pub(crate) ticks: ComponentTicksMut<'w>,
}

// -----------------------------------------------------------------------------
// NonSend

pub struct NonSend<'w, T: ?Sized + 'static> {
    pub(crate) value: &'w T,
    pub(crate) ticks: ComponentTicksRef<'w>,
}

// -----------------------------------------------------------------------------
// NonSendMut

pub struct NonSendMut<'w, T: ?Sized + 'static> {
    pub(crate) value: &'w mut T,
    pub(crate) ticks: ComponentTicksMut<'w>,
}

// -----------------------------------------------------------------------------
// Ref

pub struct Ref<'w, T: ?Sized> {
    pub(crate) value: &'w T,
    pub(crate) ticks: ComponentTicksRef<'w>,
}

// -----------------------------------------------------------------------------
// Mut

pub struct Mut<'w, T: ?Sized> {
    pub(crate) value: &'w mut T,
    pub(crate) ticks: ComponentTicksMut<'w>,
}

// -----------------------------------------------------------------------------
// MutUntyped

pub struct MutUntyped<'w> {
    pub(crate) value: PtrMut<'w>,
    pub(crate) ticks: ComponentTicksMut<'w>,
}

// -----------------------------------------------------------------------------
// Res, Ref, NonSend : into_inner

impl<'w, T: Resource> Res<'w, T> {
    #[inline(always)]
    pub fn into_inner(self) -> &'w T {
        self.value
    }
}

impl<'w, T: ?Sized> Ref<'w, T> {
    #[inline(always)]
    pub fn into_inner(self) -> &'w T {
        self.value
    }

    #[inline]
    pub fn map<U: ?Sized>(self, f: impl FnOnce(&T) -> &U) -> Ref<'w, U> {
        Ref {
            value: f(self.value),
            ticks: self.ticks,
        }
    }
}

impl<'w, T: ?Sized> NonSend<'w, T> {
    #[inline(always)]
    pub fn into_inner(self) -> &'w T {
        self.value
    }
}

// -----------------------------------------------------------------------------
// MutUntyped : Method Implementation

impl<'w> MutUntyped<'w> {
    #[inline]
    pub fn into_inner(mut self) -> PtrMut<'w> {
        self.set_changed();
        self.value
    }

    #[inline]
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

    #[inline]
    pub fn has_changed_since(&self, tick: Tick) -> bool {
        self.ticks.changed.is_newer_than(tick, self.ticks.this_run)
    }

    #[inline]
    pub fn as_mut(&mut self) -> PtrMut<'_> {
        self.set_changed();
        self.value.reborrow()
    }

    #[inline]
    pub fn as_ref(&self) -> Ptr<'_> {
        self.value.borrow()
    }

    #[inline]
    pub fn map_unchanged<T: ?Sized>(self, f: impl FnOnce(PtrMut<'w>) -> &'w mut T) -> Mut<'w, T> {
        Mut {
            value: f(self.value),
            ticks: self.ticks,
        }
    }

    #[inline]
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

    #[inline]
    fn added_tick(&self) -> Tick {
        *self.ticks.added
    }

    #[inline]
    fn changed_tick(&self) -> Tick {
        *self.ticks.changed
    }

    #[inline]
    fn changed_by(&self) -> DebugLocation {
        self.ticks.changed_by.copied()
    }
}

impl<'w> DetectChangesMut for MutUntyped<'w> {
    type Inner = PtrMut<'w>;

    #[inline]
    #[track_caller]
    fn set_changed(&mut self) {
        *self.ticks.changed = self.ticks.this_run;
        self.ticks.changed_by.assign(DebugLocation::caller());
    }

    #[inline]
    #[track_caller]
    fn set_added(&mut self) {
        *self.ticks.changed = self.ticks.this_run;
        *self.ticks.added = self.ticks.this_run;
        self.ticks.changed_by.assign(DebugLocation::caller());
    }

    #[inline]
    #[track_caller]
    fn set_changed_with(&mut self, last_changed: Tick) {
        *self.ticks.changed = last_changed;
        self.ticks.changed_by.assign(DebugLocation::caller());
    }

    #[inline]
    #[track_caller]
    fn set_added_with(&mut self, last_added: Tick) {
        *self.ticks.added = last_added;
        *self.ticks.changed = last_added;
        self.ticks.changed_by.assign(DebugLocation::caller());
    }

    #[inline]
    #[track_caller]
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

impl<'w, T> From<Mut<'w, T>> for MutUntyped<'w> {
    #[inline(always)]
    fn from(value: Mut<'w, T>) -> Self {
        MutUntyped {
            value: value.value.into(),
            ticks: value.ticks,
        }
    }
}

// -----------------------------------------------------------------------------
// Res: IntoIterator and From

impl<'w, T: Resource> From<ResMut<'w, T>> for Res<'w, T> {
    fn from(res: ResMut<'w, T>) -> Self {
        Self {
            value: res.value,
            ticks: res.ticks.into(),
        }
    }
}

impl<'w, T: Resource> From<Res<'w, T>> for Ref<'w, T> {
    #[inline(always)]
    fn from(res: Res<'w, T>) -> Self {
        Self {
            value: res.value,
            ticks: res.ticks,
        }
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

// -----------------------------------------------------------------------------
// ResMut: IntoIterator

impl<'w, T: Resource> From<ResMut<'w, T>> for Mut<'w, T> {
    #[inline(always)]
    fn from(other: ResMut<'w, T>) -> Mut<'w, T> {
        Mut {
            value: other.value,
            ticks: other.ticks,
        }
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

// -----------------------------------------------------------------------------
// NonSend and NonSendMut : From

impl<'w, T> From<NonSendMut<'w, T>> for NonSend<'w, T> {
    fn from(other: NonSendMut<'w, T>) -> Self {
        Self {
            value: other.value,
            ticks: other.ticks.into(),
        }
    }
}

impl<'w, T: 'static> From<NonSendMut<'w, T>> for Mut<'w, T> {
    #[inline(always)]
    fn from(other: NonSendMut<'w, T>) -> Mut<'w, T> {
        Mut {
            value: other.value,
            ticks: other.ticks,
        }
    }
}

// -----------------------------------------------------------------------------
// Ref: IntoIterator

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
// RefMut: IntoIterator and From

impl<'w, T: ?Sized> From<Mut<'w, T>> for Ref<'w, T> {
    fn from(mut_ref: Mut<'w, T>) -> Self {
        Self {
            value: mut_ref.value,
            ticks: mut_ref.ticks.into(),
        }
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
// impl_mut_methods

macro_rules! impl_mut_methods {
    ($name:ident < $( $generics:tt ),+ >, $target:ty, $($traits:ident)?) => {
        impl<$($generics),* : ?Sized $(+ $traits)?> $name<$($generics),*> {
            /// Consume `self` and return a mutable reference to the
            /// contained value while marking `self` as "changed".
            #[inline]
            pub fn into_inner(mut self) -> &'w mut $target {
                self.set_changed();
                self.value
            }

            /// Returns a `Mut<>` with a smaller lifetime.
            /// This is useful if you have `&mut
            #[doc = stringify!($name)]
            /// <T>`, but you need a `Mut<T>`.
            pub fn reborrow(&mut self) -> Mut<'_, $target> {
                Mut {
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

            /// Maps to an inner value by applying a function to the contained reference,
            /// without flagging a change.
            ///
            /// You should never modify the argument passed to the closure --
            /// if you want to modify the data without flagging a change, consider
            /// using [`DetectChangesMut::bypass_change_detection`] to make your intent explicit.
            pub fn map_unchanged<U: ?Sized>(
                self,
                f: impl FnOnce(&mut $target) -> &mut U,
            ) -> Mut<'w, U> {
                Mut {
                    value: f(self.value),
                    ticks: self.ticks,
                }
            }

            /// Optionally maps to an inner value by applying a function to
            /// the contained reference.
            ///
            /// This is useful in a situation where you need to convert a
            /// `Mut<T>` to a `Mut<U>`, but only if `T` contains `U`.
            ///
            /// As with `map_unchanged`, you should never modify the argument passed to the closure.
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

            /// Optionally maps to an inner value by applying a function to
            /// the contained reference, returns an error on failure.
            ///
            /// This is useful in a situation where you need to convert a
            /// `Mut<T>` to a `Mut<U>`, but only if `T` contains `U`.
            ///
            /// As with `map_unchanged`, you should never modify the argument passed to the closure.
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

            /// Allows you access to the dereferenced value of this pointer without immediately
            /// triggering change detection.
            pub fn as_deref_mut(&mut self) -> Mut<'_, <$target as ::core::ops::Deref>::Target>
                where $target: ::core::ops::DerefMut
            {
                self.reborrow().map_unchanged(|v| v.deref_mut())
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

            #[inline]
            fn changed_tick(&self) -> Tick {
                *self.ticks.changed
            }

            #[inline]
            fn added_tick(&self) -> Tick {
                *self.ticks.added
            }

            #[inline]
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
                ::core::ops::Deref::deref(self)
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

            #[inline]
            #[track_caller]
            fn set_changed(&mut self) {
                *self.ticks.changed = self.ticks.this_run;
                self.ticks.changed_by.assign(DebugLocation::caller());
            }

            #[inline]
            #[track_caller]
            fn set_added(&mut self) {
                *self.ticks.changed = self.ticks.this_run;
                *self.ticks.added = self.ticks.this_run;
                self.ticks.changed_by.assign(DebugLocation::caller());
            }

            #[inline]
            #[track_caller]
            fn set_changed_with(&mut self, changed_tick: Tick) {
                *self.ticks.changed = changed_tick;
                self.ticks.changed_by.assign(DebugLocation::caller());
            }

            #[inline]
            #[track_caller]
            fn set_added_with(&mut self, added_tick: Tick) {
                *self.ticks.added = added_tick;
                *self.ticks.changed = added_tick;
                self.ticks.changed_by.assign(DebugLocation::caller());
            }

            #[inline]
            fn bypass_change_detection(&mut self) -> &mut Self::Inner {
                self.value
            }
        }

        impl<$($generics),* : ?Sized $(+ $traits)?> ::core::ops::DerefMut for $name<$($generics),*> {
            #[inline]
            #[track_caller]
            fn deref_mut(&mut self) -> &mut Self::Target {
                self.set_changed();
                self.ticks.changed_by.assign(DebugLocation::caller());
                self.value
            }
        }

        impl<$($generics),* $(: $traits)?> AsMut<$target> for $name<$($generics),*> {
            #[inline]
            fn as_mut(&mut self) -> &mut $target {
                ::core::ops::DerefMut::deref_mut(self)
            }
        }
    };
}

impl_change_detection_mut_and_deref_mut!(Mut<'w, T>, T,);
impl_change_detection_mut_and_deref_mut!(ResMut<'w, T>, T, Resource);
impl_change_detection_mut_and_deref_mut!(NonSendMut<'w, T>, T,);

// -----------------------------------------------------------------------------
// Internal API

impl<'w, T: ?Sized> Ref<'w, T> {
    // TODO
    #[inline]
    pub fn new(
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

    // TODO
    #[inline(always)]
    pub fn set_ticks(&mut self, last_run: Tick, this_run: Tick) {
        self.ticks.last_run = last_run;
        self.ticks.this_run = this_run;
    }
}

impl<'w, T: ?Sized> Mut<'w, T> {
    // TODO
    #[inline]
    pub fn new(
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

    // TODO
    #[inline(always)]
    pub fn set_ticks(&mut self, last_run: Tick, this_run: Tick) {
        self.ticks.last_run = last_run;
        self.ticks.this_run = this_run;
    }
}
