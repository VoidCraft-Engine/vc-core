#![expect(unsafe_code, reason = "UnsafeCell")]

use core::cell::UnsafeCell;
use core::fmt;
use core::marker::PhantomData;
use core::ptr;

use super::World;

// -----------------------------------------------------------------------------
// UnsafeWorldCell

#[derive(Copy, Clone)]
pub struct UnsafeWorldCell<'w> {
    _marker: PhantomData<(&'w World, &'w UnsafeCell<World>)>,
    ptr: *mut World,
    #[cfg(any(debug_assertions, feature = "debug"))]
    allows_mutable_access: bool,
}

unsafe impl Send for UnsafeWorldCell<'_> {}
unsafe impl Sync for UnsafeWorldCell<'_> {}

impl fmt::Debug for UnsafeWorldCell<'_> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        fmt::Debug::fmt(unsafe { self.world_metadata() }, f)
    }
}

impl<'w> From<&'w mut World> for UnsafeWorldCell<'w> {
    #[inline(always)]
    fn from(value: &'w mut World) -> Self {
        Self::new_mutable(value)
    }
}

impl<'w> From<&'w World> for UnsafeWorldCell<'w> {
    #[inline(always)]
    fn from(value: &'w World) -> Self {
        Self::new_readonly(value)
    }
}

impl<'w> UnsafeWorldCell<'w> {
    #[inline(always)]
    pub const fn new_readonly(world: &'w World) -> Self {
        Self {
            _marker: PhantomData,
            ptr: ptr::from_ref(world).cast_mut(),
            #[cfg(any(debug_assertions, feature = "debug"))]
            allows_mutable_access: false,
        }
    }

    #[inline(always)]
    pub const fn new_mutable(world: &'w mut World) -> Self {
        Self {
            _marker: PhantomData,
            ptr: ptr::from_mut(world),
            #[cfg(any(debug_assertions, feature = "debug"))]
            allows_mutable_access: true,
        }
    }

    #[inline(always)]
    #[cfg_attr(any(debug_assertions, feature = "debug"), track_caller)]
    pub const fn assert_allows_mutable_access(self) {
        #[cfg(any(debug_assertions, feature = "debug"))]
        assert!(
            self.allows_mutable_access,
            "mutating world data via `World::as_unsafe_world_cell_readonly` is forbidden"
        );
    }

    #[inline(always)]
    pub const unsafe fn world_mut(self) -> &'w mut World {
        self.assert_allows_mutable_access();
        unsafe { &mut *self.ptr }
    }

    #[inline(always)]
    pub const unsafe fn world_ref(self) -> &'w World {
        unsafe { &*self.ptr }
    }

    #[inline(always)]
    pub const unsafe fn world_metadata(self) -> &'w World {
        unsafe { &*self.ptr }
    }
}
