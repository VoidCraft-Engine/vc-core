#![expect(unsafe_code, reason = "unchecked is unsafe")]

pub(crate) trait VecSwapRemove<T> {
    unsafe fn swap_remove_nonoverlapping(&mut self, index: usize) -> T;

    unsafe fn remove_last(&mut self, index: usize) -> T;
}

impl<T> VecSwapRemove<T> for alloc::vec::Vec<T> {
    #[inline(always)]
    unsafe fn swap_remove_nonoverlapping(&mut self, index: usize) -> T {
        let new_len = self.len() - 1;
        let base_ptr = self.as_mut_ptr();

        unsafe {
            let removal = base_ptr.add(index);
            let last = base_ptr.add(new_len);

            let value = removal.read();
            core::ptr::copy_nonoverlapping(last, removal, 1);

            self.set_len(new_len);

            value
        }
    }

    #[inline(always)]
    unsafe fn remove_last(&mut self, last_index: usize) -> T {
        unsafe {
            let value = self.as_ptr().add(last_index).read();
            self.set_len(last_index);
            value
        }
    }
}

pub(crate) trait VecCopyRemove<T> {
    unsafe fn copy_remove_nonoverlapping(&mut self, index: usize) -> T;
}

impl<T: Copy> VecCopyRemove<T> for alloc::vec::Vec<T> {
    /// 注意返回的是原先的末尾元素的备份，而非被删除的元素。
    #[inline(always)]
    unsafe fn copy_remove_nonoverlapping(&mut self, index: usize) -> T {
        let new_len = self.len() - 1;
        let base_ptr = self.as_mut_ptr();

        unsafe {
            let dst = base_ptr.add(index);
            core::ptr::copy_nonoverlapping(base_ptr.add(new_len), dst, 1);

            self.set_len(new_len);

            core::ptr::read(dst)
        }
    }
}
