#![expect(unsafe_code, reason = "unchecked is unsafe")]

use core::ptr;

// -----------------------------------------------------------------------------
// VecSwapRemove

pub(crate) trait VecSwapRemove<T> {
    unsafe fn swap_remove_nonoverlapping(&mut self, index: usize, last_index: usize) -> T;
    unsafe fn remove_last(&mut self, last_index: usize) -> T;
}

impl<T> VecSwapRemove<T> for alloc::vec::Vec<T> {
    #[inline(always)]
    unsafe fn swap_remove_nonoverlapping(&mut self, index: usize, last_index: usize) -> T {
        let base_ptr = self.as_mut_ptr();

        unsafe {
            let removal = base_ptr.add(index);
            let last = base_ptr.add(last_index);

            let value = ptr::read(removal);

            ptr::copy_nonoverlapping(last, removal, 1);

            self.set_len(last_index);

            value
        }
    }

    #[inline(always)]
    unsafe fn remove_last(&mut self, last_index: usize) -> T {
        unsafe {
            let value = ptr::read(self.as_ptr().add(last_index));
            self.set_len(last_index);
            value
        }
    }
}

// -----------------------------------------------------------------------------
// VecCopyRemove

pub(crate) trait VecCopyRemove<T> {
    unsafe fn copy_last_and_return_nonoverlapping(&mut self, index: usize, last_index: usize) -> T;
}

impl<T: Copy> VecCopyRemove<T> for alloc::vec::Vec<T> {
    #[inline(always)]
    unsafe fn copy_last_and_return_nonoverlapping(&mut self, index: usize, last_index: usize) -> T {
        let base_ptr = self.as_mut_ptr();

        unsafe {
            let src = base_ptr.add(last_index);
            let dst = base_ptr.add(index);

            ptr::copy_nonoverlapping(src, dst, 1);

            self.set_len(last_index);

            ptr::read(dst)
        }
    }
}
