#![expect(unsafe_code, reason = "deref ptr is unsafe.")]

use core::cell::UnsafeCell;

mod private {
    pub trait Seal {}
}

impl<T> private::Seal for &'_ UnsafeCell<T> {}

/// Extension trait for helper methods on [`UnsafeCell`]
pub trait UnsafeCellDeref<'a, T>: private::Seal {
    /// # Safety
    /// - The returned value must be unique and not alias any mutable
    ///   or immutable references to the contents of the [`UnsafeCell`].
    /// - At all times, you must avoid data races. If multiple threads
    ///   have access to the same [`UnsafeCell`], then any writes must
    ///   have a proper happens-before relation to all other accesses
    ///   or use atomics ([`UnsafeCell`] docs for reference).
    unsafe fn deref_mut(self) -> &'a mut T;

    /// # Safety
    /// - For the lifetime `'a` of the returned value you must not construct
    ///   a mutable reference to the contents of the [`UnsafeCell`].
    /// - At all times, you must avoid data races. If multiple threads have
    ///   access to the same [`UnsafeCell`], then any writes must have a
    ///   proper happens-before relation to all other accesses or use atomics
    ///   ([`UnsafeCell`] docs for reference).
    unsafe fn deref(self) -> &'a T;

    /// Returns a copy of the contained value.
    ///
    /// # Safety
    /// - The [`UnsafeCell`] must not currently have a mutable reference to
    ///   its content.
    /// - At all times, you must avoid data races. If multiple threads have
    ///   access to the same [`UnsafeCell`], then any writes must have a proper
    ///   happens-before relation to all other accesses or use atomics
    ///   ([`UnsafeCell`] docs for reference).
    unsafe fn read(self) -> T
    where
        T: Copy;
}

impl<'a, T> UnsafeCellDeref<'a, T> for &'a UnsafeCell<T> {
    #[inline(always)]
    unsafe fn deref_mut(self) -> &'a mut T {
        // SAFETY: The caller upholds the alias rules.
        unsafe { &mut *self.get() }
    }

    #[inline(always)]
    unsafe fn deref(self) -> &'a T {
        // SAFETY: The caller upholds the alias rules.
        unsafe { &*self.get() }
    }

    #[inline(always)]
    unsafe fn read(self) -> T
    where
        T: Copy,
    {
        // SAFETY: The caller upholds the alias rules.
        unsafe { self.get().read() }
    }
}
