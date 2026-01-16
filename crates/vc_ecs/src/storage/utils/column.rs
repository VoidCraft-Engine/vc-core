#![expect(unsafe_code, reason = "original implementation requires unsafe code.")]

use core::alloc::Layout;
use core::cell::UnsafeCell;
use core::num::NonZeroUsize;
use core::panic::Location;

use vc_ptr::{OwningPtr, Ptr, PtrMut};

use super::{BlobArray, ThinArray};

use crate::cfg;
use crate::tick::{CheckTicks, Tick};
use crate::utils::DebugLocation;

// -----------------------------------------------------------------------------
// Column

#[derive(Debug)]
pub struct Column {
    data: BlobArray,
    added_ticks: ThinArray<UnsafeCell<Tick>>,
    changed_ticks: ThinArray<UnsafeCell<Tick>>,
    changed_by: DebugLocation<ThinArray<UnsafeCell<&'static Location<'static>>>>,
    #[cfg(any(debug_assertions, feature = "debug"))]
    capacity: usize,
}

impl Column {
    #[inline(always)]
    pub fn empty(item_layout: Layout, drop_fn: Option<unsafe fn(OwningPtr<'_>)>) -> Self {
        Self {
            data: unsafe { BlobArray::empty(item_layout, drop_fn) },
            added_ticks: ThinArray::empty(),
            changed_ticks: ThinArray::empty(),
            changed_by: DebugLocation::new_with(ThinArray::empty),
            #[cfg(any(debug_assertions, feature = "debug"))]
            capacity: 0,
        }
    }

    pub fn with_capacity(
        item_layout: Layout,
        drop_fn: Option<unsafe fn(OwningPtr<'_>)>,
        capacity: usize,
    ) -> Self {
        Self {
            data: unsafe { BlobArray::with_capacity(item_layout, drop_fn, capacity) },
            added_ticks: ThinArray::with_capacity(capacity),
            changed_ticks: ThinArray::with_capacity(capacity),
            changed_by: DebugLocation::new_with(|| ThinArray::with_capacity(capacity)),
            #[cfg(any(debug_assertions, feature = "debug"))]
            capacity,
        }
    }

    pub unsafe fn alloc(&mut self, new_capacity: NonZeroUsize) {
        cfg::debug! {
            assert!(self.capacity == 0);
            self.capacity = new_capacity.get();
        }

        unsafe {
            self.data.alloc(new_capacity);
            self.added_ticks.alloc(new_capacity);
            self.changed_ticks.alloc(new_capacity);
            cfg::debug! {
                self.changed_by.as_mut().map(|cb| cb.alloc(new_capacity));
            }
        }
    }

    pub unsafe fn realloc(&mut self, current_capacity: NonZeroUsize, new_capacity: NonZeroUsize) {
        cfg::debug! {
            assert!(self.capacity == current_capacity.get());
            self.capacity = new_capacity.get();
        }

        unsafe {
            self.data.realloc(current_capacity, new_capacity);
            self.added_ticks.realloc(current_capacity, new_capacity);
            self.changed_ticks.realloc(current_capacity, new_capacity);
            cfg::debug! {
                self.changed_by.as_mut().map(|cb| cb.realloc(current_capacity, new_capacity));
            }
        }
    }

    pub unsafe fn dealloc(&mut self, current_capacity: usize, len: usize) {
        cfg::debug! {
            assert!(self.capacity == current_capacity);
            assert!(len <= self.capacity);
            self.capacity = 0;
        }

        unsafe {
            self.added_ticks.dealloc(current_capacity);
            self.changed_ticks.dealloc(current_capacity);
            self.data.dealloc(current_capacity, len);
            cfg::debug! {
                self.changed_by.as_mut().map(|cb| cb.dealloc(current_capacity));
            }
        }
    }

    pub unsafe fn clear(&mut self, len: usize) {
        cfg::debug! { assert!(len <= self.capacity); }
        unsafe {
            self.data.clear(len);
        }
    }

    pub unsafe fn drop_last(&mut self, last_index: usize) {
        cfg::debug! { assert!(last_index < self.capacity); }
        unsafe {
            self.data.drop_last(last_index);
        }
    }

    #[inline(always)]
    pub fn get_drop_fn(&self) -> Option<unsafe fn(OwningPtr<'_>)> {
        self.data.drop_fn()
    }

    #[inline(always)]
    pub unsafe fn get_data(&self, index: usize) -> Ptr<'_> {
        cfg::debug! { assert!(index < self.capacity); }
        unsafe { self.data.get_item(index) }
    }

    #[inline(always)]
    pub unsafe fn get_data_mut(&mut self, index: usize) -> PtrMut<'_> {
        cfg::debug! { assert!(index < self.capacity); }
        unsafe { self.data.get_item_mut(index) }
    }

    #[inline(always)]
    pub unsafe fn get_added_tick(&self, index: usize) -> &UnsafeCell<Tick> {
        cfg::debug! { assert!(index < self.capacity); }
        unsafe { self.added_ticks.get_item(index) }
    }

    #[inline(always)]
    pub unsafe fn get_changed_tick(&self, index: usize) -> &UnsafeCell<Tick> {
        cfg::debug! { assert!(index < self.capacity); }
        unsafe { self.changed_ticks.get_item(index) }
    }

    #[inline(always)]
    pub unsafe fn get_changed_by(
        &self,
        index: usize,
    ) -> DebugLocation<&UnsafeCell<&'static Location<'static>>> {
        cfg::debug! { assert!(index < self.capacity); }
        unsafe { self.changed_by.as_ref().map(|cb| cb.get_item(index)) }
    }

    #[inline(always)]
    pub unsafe fn get_data_slice<T>(&self, len: usize) -> &[UnsafeCell<T>] {
        cfg::debug! { assert!(len <= self.capacity); }
        unsafe { self.data.as_slice(len) }
    }

    #[inline(always)]
    pub unsafe fn get_added_ticks_slice(&self, len: usize) -> &[UnsafeCell<Tick>] {
        cfg::debug! { assert!(len <= self.capacity); }
        unsafe { self.added_ticks.as_slice(len) }
    }

    #[inline(always)]
    pub unsafe fn get_changed_ticks_slice(&self, len: usize) -> &[UnsafeCell<Tick>] {
        cfg::debug! { assert!(len <= self.capacity); }
        unsafe { self.changed_ticks.as_slice(len) }
    }

    #[inline(always)]
    pub unsafe fn get_changed_by_slice(
        &self,
        len: usize,
    ) -> DebugLocation<&[UnsafeCell<&'static Location<'static>>]> {
        cfg::debug! { assert!(len <= self.capacity); }
        unsafe { self.changed_by.as_ref().map(|cb| cb.as_slice(len)) }
    }

    #[inline(always)]
    pub unsafe fn reset_item(&mut self, index: usize) {
        cfg::debug! { assert!(index < self.capacity); }
        unsafe {
            self.added_ticks
                .init_item(index, UnsafeCell::new(Tick::new(0)));
            self.changed_ticks
                .init_item(index, UnsafeCell::new(Tick::new(0)));
            cfg::debug! {
                let caller = Location::caller();
                self.changed_by.as_mut().map(move |cb|
                    cb.init_item(index, UnsafeCell::new(caller))
                );
            }
        }
    }

    pub unsafe fn init_item(
        &mut self,
        index: usize,
        data: OwningPtr<'_>,
        tick: Tick,
        caller: DebugLocation,
    ) {
        cfg::debug! { assert!(index < self.capacity); }
        unsafe {
            self.data.init_item(index, data);
            self.added_ticks.init_item(index, UnsafeCell::new(tick));
            self.changed_ticks.init_item(index, UnsafeCell::new(tick));
            cfg::debug! {
                self.changed_by.as_mut()
                    .map(|cb| cb.get_item_mut(index).get_mut())
                    .assign(caller);
            }
        }
    }

    pub unsafe fn replace_item(
        &mut self,
        index: usize,
        data: OwningPtr<'_>,
        change_tick: Tick,
        caller: DebugLocation,
    ) {
        cfg::debug! { assert!(index < self.capacity); }
        unsafe {
            self.data.replace_item(index, data);
            self.changed_ticks
                .init_item(index, UnsafeCell::new(change_tick));
            cfg::debug! {
                self.changed_by.as_mut()
                    .map(|cb| cb.get_item_mut(index).get_mut())
                    .assign(caller);
            }
        }
    }

    #[must_use = "The returned pointer should be used to drop the removed element"]
    pub unsafe fn swap_remove_nonoverlapping(
        &mut self,
        index: usize,
        last_index: usize,
    ) -> OwningPtr<'_> {
        cfg::debug! {
            assert!(index < last_index && last_index < self.capacity);
        }

        unsafe {
            let data = self.data.swap_remove_nonoverlapping(index, last_index);
            self.added_ticks
                .swap_remove_nonoverlapping(index, last_index);
            self.changed_ticks
                .swap_remove_nonoverlapping(index, last_index);

            cfg::debug! {
                // Use `{ ..; }` to eliminate return values and reduce compilation workload.
                self.changed_by.as_mut().map(|cb| {
                    cb.swap_remove_nonoverlapping(index, last_index);
                });
            }

            data
        }
    }

    #[inline]
    #[must_use = "The returned pointer should be used to drop the removed element"]
    pub unsafe fn swap_remove(&mut self, index: usize, last_index: usize) -> OwningPtr<'_> {
        if index != last_index {
            return unsafe { self.swap_remove_nonoverlapping(index, last_index) };
        }

        cfg::debug! { assert!(last_index < self.capacity); }
        unsafe { self.data.remove_last(last_index) }
    }

    pub unsafe fn swap_remove_and_drop_nonoverlapping(&mut self, index: usize, last_index: usize) {
        cfg::debug! {
            assert!(index < last_index && last_index < self.capacity);
        }

        unsafe {
            self.data
                .swap_remove_and_drop_nonoverlapping(index, last_index);
            self.added_ticks
                .swap_remove_nonoverlapping(index, last_index);
            self.changed_ticks
                .swap_remove_nonoverlapping(index, last_index);

            cfg::debug! {
                // Use `{ ..; }` to eliminate return values and reduce compilation workload.
                self.changed_by.as_mut().map(|cb| {
                    cb.swap_remove_nonoverlapping(index, last_index);
                });
            }
        }
    }

    #[inline]
    pub unsafe fn swap_remove_and_drop(&mut self, index: usize, last_index: usize) {
        if index != last_index {
            return unsafe { self.swap_remove_and_drop_nonoverlapping(index, last_index) };
        }

        cfg::debug! { assert!(last_index < self.capacity); }
        unsafe { self.data.drop_last(last_index) }
    }

    pub unsafe fn init_item_from(
        &mut self,
        other: &mut Column,
        other_last_index: usize,
        src: usize,
        dst: usize,
    ) {
        cfg::debug! {
            assert_eq!(self.data.layout(), other.data.layout());
            assert!(dst < self.capacity);
            assert!(src <= other_last_index && other_last_index < other.capacity);
        }

        let src_val: OwningPtr<'_>;
        let added_tick: UnsafeCell<Tick>;
        let changed_tick: UnsafeCell<Tick>;

        unsafe {
            if src != other_last_index {
                src_val = other.data.swap_remove_nonoverlapping(src, other_last_index);
                added_tick = other
                    .added_ticks
                    .swap_remove_nonoverlapping(src, other_last_index);
                changed_tick = other
                    .changed_ticks
                    .swap_remove_nonoverlapping(src, other_last_index);
                cfg::debug! {
                    self.changed_by.as_mut().zip(other.changed_by.as_mut()).map(|(scb, ocb)| {
                        let changed_by = ocb.swap_remove_nonoverlapping(src, other_last_index);
                        scb.init_item(dst, changed_by);
                    });
                }
            } else {
                src_val = other.data.remove_last(src);
                added_tick = other.added_ticks.remove_last(src);
                changed_tick = other.changed_ticks.remove_last(src);
                cfg::debug! {
                    self.changed_by.as_mut().zip(other.changed_by.as_mut()).map(|(scb, ocb)| {
                        let changed_by = ocb.remove_last(src);
                        scb.init_item(dst, changed_by);
                    });
                }
            }

            self.data.init_item(dst, src_val);
            self.added_ticks.init_item(dst, added_tick);
            self.changed_ticks.init_item(dst, changed_tick);
        }
    }

    pub unsafe fn check_ticks(&mut self, len: usize, check: CheckTicks) {
        for i in 0..len {
            unsafe {
                self.added_ticks
                    .get_item_mut(i)
                    .get_mut()
                    .check_age(check.tick());
                self.changed_ticks
                    .get_item_mut(i)
                    .get_mut()
                    .check_age(check.tick());
            }
        }
    }

    #[inline]
    pub unsafe fn get_component_ticks(&self, index: usize) -> crate::component::ComponentTicks {
        unsafe {
            crate::component::ComponentTicks {
                added: self.added_ticks.copy_item(index),
                changed: self.changed_ticks.copy_item(index),
            }
        }
    }
}
