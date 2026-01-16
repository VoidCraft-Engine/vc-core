#![expect(unsafe_code, reason = "UnsafeCell")]

use core::cell::UnsafeCell;
use core::marker::PhantomData;

use super::World;

// -----------------------------------------------------------------------------
// PhantomData

#[derive(Copy, Clone)]
pub struct UnsafeWorldCell<'w> {
    _marker: PhantomData<(&'w World, &'w UnsafeCell<World>)>,
    _ptr: *mut World,
    #[cfg(any(debug_assertions, feature = "debug"))]
    _allows_mutable_access: bool,
}

// SAFETY: `&World` and `&mut World` are both `Send` and `Sync`
unsafe impl Send for UnsafeWorldCell<'_> {}
unsafe impl Sync for UnsafeWorldCell<'_> {}
