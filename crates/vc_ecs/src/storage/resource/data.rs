#![expect(unsafe_code, reason = "original implementation")]

use core::alloc::Layout;
use core::cell::UnsafeCell;
use core::panic::Location;

use vc_ptr::{OwningPtr, Ptr};
use vc_utils::UnsafeCellDeref;

use crate::cfg;
use crate::component::{ComponentTickCells, ComponentTicks, ComponentTicksMut, MutUntyped};
use crate::storage::BlobArray;
use crate::tick::{CheckTicks, Tick};
use crate::utils::{DebugLocation, DebugName};

cfg::std! {
    use std::thread::ThreadId;
}

cfg::std! {
    #[cfg(not(feature = "std"))]
    compile_error!("global std is enabled, but `vc_ecs/std` is disabled.");
}

// -----------------------------------------------------------------------------
// ResourceData

pub struct ResourceData<const SEND: bool> {
    name: DebugName,
    /// Capacity is 1, length is 1 if `present` and 0 otherwise.
    data: BlobArray,
    present: bool,
    added_tick: UnsafeCell<Tick>,
    changed_tick: UnsafeCell<Tick>,
    changed_by: DebugLocation<UnsafeCell<&'static Location<'static>>>,
    #[cfg(feature = "std")]
    thread_id: Option<ThreadId>,
}

impl<const SEND: bool> Drop for ResourceData<SEND> {
    fn drop(&mut self) {
        if !SEND && self.present {
            cfg::std! {
                if std::thread::panicking() { return; }
            }
            self.validate_access();
        }

        unsafe {
            self.data.dealloc(1, self.present as usize);
        }
    }
}

impl<const SEND: bool> ResourceData<SEND> {
    /// The only element in the underlying `BlobArray`.
    const INDEX: usize = 0;

    #[inline(always)]
    fn validate_access(&self) {
        #[cold]
        #[inline(never)]
        fn invalid_access<const S: bool>(this: &ResourceData<S>) -> ! {
            panic!(
                "Attempted to access or drop non-send resource {} from thread {:?} on a thread {:?}.",
                this.name,
                this.thread_id,
                std::thread::current().id()
            );
        }

        cfg::std! {
            if !SEND && self.thread_id != Some(std::thread::current().id()) {
                invalid_access(self);
            }
        }

        // Currently, no_std is single-threaded only, so this is safe to ignore.
    }

    #[inline(always)]
    fn init_thread_id(&mut self) {
        cfg::std! {
            if !SEND {
                self.thread_id = Some(std::thread::current().id());
            }
        }
    }

    #[inline]
    pub fn new(name: DebugName, layout: Layout, drop_fn: Option<unsafe fn(OwningPtr<'_>)>) -> Self {
        let data = unsafe { BlobArray::with_capacity(layout, drop_fn, 1) };

        ResourceData {
            name,
            data,
            present: false,
            added_tick: UnsafeCell::new(Tick::new(0)),
            changed_tick: UnsafeCell::new(Tick::new(0)),
            changed_by: DebugLocation::caller().map(UnsafeCell::new),
            #[cfg(feature = "std")]
            thread_id: None,
        }
    }

    #[inline(always)]
    pub fn present(&self) -> bool {
        self.present
    }

    #[inline]
    pub fn get_data(&self) -> Option<Ptr<'_>> {
        if self.present {
            self.validate_access();
            unsafe { Some(self.data.get_item(Self::INDEX)) }
        } else {
            None
        }
    }

    #[inline]
    pub fn get_component_ticks(&self) -> Option<ComponentTicks> {
        if self.present {
            self.validate_access();
            Some(ComponentTicks {
                added: unsafe { self.added_tick.read() },
                changed: unsafe { self.changed_tick.read() },
            })
        } else {
            None
        }
    }

    #[inline]
    pub fn get_data_with_ticks(&self) -> Option<(Ptr<'_>, ComponentTickCells<'_>)> {
        if self.present {
            self.validate_access();
            Some((
                unsafe { self.data.get_item(Self::INDEX) },
                ComponentTickCells {
                    added: &self.added_tick,
                    changed: &self.changed_tick,
                    changed_by: self.changed_by.as_ref(),
                },
            ))
        } else {
            None
        }
    }

    #[inline]
    pub fn get_mut(&mut self, last_run: Tick, this_run: Tick) -> Option<MutUntyped<'_>> {
        if self.present {
            self.validate_access();
            let value = unsafe { self.data.get_item_mut(Self::INDEX) };
            let cells = ComponentTickCells {
                added: &self.added_tick,
                changed: &self.changed_tick,
                changed_by: self.changed_by.as_ref(),
            };
            let ticks = unsafe { ComponentTicksMut::from_tick_cells(cells, last_run, this_run) };
            Some(MutUntyped { value, ticks })
        } else {
            None
        }
    }

    #[inline]
    pub unsafe fn insert(
        &mut self,
        value: OwningPtr<'_>,
        change_tick: Tick,
        _caller: DebugLocation,
    ) {
        if self.present {
            self.validate_access();
            unsafe {
                self.data.replace_item(Self::INDEX, value);
            }
        } else {
            self.init_thread_id();

            unsafe {
                self.data.init_item(Self::INDEX, value);
            }
            self.present = true;
        }

        unsafe {
            *self.changed_tick.deref_mut() = change_tick;
        }

        cfg::debug! {
            self.changed_by.as_ref()
                .map(|changed_by| unsafe{ changed_by.deref_mut() })
                .assign(_caller);
        }
    }

    #[inline]
    pub unsafe fn insert_with_ticks(
        &mut self,
        value: OwningPtr<'_>,
        change_ticks: ComponentTicks,
        _caller: DebugLocation,
    ) {
        if self.present {
            self.validate_access();
            unsafe {
                self.data.replace_item(Self::INDEX, value);
            }
        } else {
            self.init_thread_id();

            unsafe {
                self.data.init_item(Self::INDEX, value);
            }
            self.present = true;
        }

        unsafe {
            *self.added_tick.deref_mut() = change_ticks.added;
            *self.changed_tick.deref_mut() = change_ticks.changed;
        }

        cfg::debug! {
            self.changed_by.as_ref()
                .map(|changed_by| unsafe{ changed_by.deref_mut() })
                .assign(_caller);
        }
    }

    #[inline]
    #[must_use = "The returned pointer to the removed component should be used or dropped"]
    pub fn remove(&mut self) -> Option<(OwningPtr<'_>, ComponentTicks, DebugLocation)> {
        if !self.present {
            return None;
        }
        self.validate_access();
        self.present = false;

        unsafe {
            let ptr = self.data.get_item_mut(Self::INDEX).promote();
            let ticks = ComponentTicks {
                added: self.added_tick.read(),
                changed: self.changed_tick.read(),
            };
            let caller = self.changed_by.as_ref().map(|changed_by| changed_by.read());

            Some((ptr, ticks, caller))
        }
    }

    #[inline]
    pub fn remove_and_drop(&mut self) {
        if self.present {
            self.validate_access();
            unsafe {
                self.data.drop_last(Self::INDEX);
            }
            self.present = false;
        }
    }

    #[inline]
    pub fn check_ticks(&mut self, check: CheckTicks) {
        self.added_tick.get_mut().check_age(check.tick());
        self.changed_tick.get_mut().check_age(check.tick());
    }
}
