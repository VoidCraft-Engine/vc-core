#![expect(unsafe_code, reason = "original implementation requires unsafe code.")]

use alloc::alloc as malloc;
use core::alloc::Layout;
use core::num::NonZeroUsize;
use core::ptr::NonNull;

use vc_ptr::{OwningPtr, Ptr, PtrMut};

// -----------------------------------------------------------------------------
// BlobArray

struct AbortOnPanic;

impl Drop for AbortOnPanic {
    #[cold]
    #[inline(never)]
    fn drop(&mut self) {
        crate::cfg::std! {
            if {
                std::eprintln!("Aborting due to drop_fn panicked.");
                std::process::abort();
            } else {
                panic!("Aborting due to drop_fn panicked.");
            }
        }
    }
}

// -----------------------------------------------------------------------------
// BlobArray

#[derive(Debug)]
pub(crate) struct BlobArray {
    item_layout: Layout,
    data: NonNull<u8>,
    drop_fn: Option<unsafe fn(OwningPtr<'_>)>,
}

impl BlobArray {
    #[inline(always)]
    pub const fn is_zst(&self) -> bool {
        self.item_layout.size() == 0
    }

    #[inline(always)]
    pub const fn layout(&self) -> Layout {
        self.item_layout
    }

    #[inline(always)]
    pub const fn drop_fn(&self) -> Option<unsafe fn(OwningPtr<'_>)> {
        self.drop_fn
    }

    #[inline(always)]
    pub const unsafe fn empty(
        item_layout: Layout,
        drop_fn: Option<unsafe fn(OwningPtr<'_>)>,
    ) -> Self {
        let align = unsafe { NonZeroUsize::new_unchecked(item_layout.align()) };

        Self {
            item_layout,
            drop_fn,
            data: NonNull::without_provenance(align),
        }
    }

    #[inline]
    pub unsafe fn with_capacity(
        item_layout: Layout,
        drop_fn: Option<unsafe fn(OwningPtr<'_>)>,
        capacity: usize,
    ) -> Self {
        let mut arr = unsafe { Self::empty(item_layout, drop_fn) };

        if capacity > 0 {
            unsafe {
                arr.alloc(NonZeroUsize::new_unchecked(capacity));
            }
        }

        arr
    }

    pub unsafe fn alloc(&mut self, capacity: NonZeroUsize) {
        if !self.is_zst() {
            let new_layout = array_layout(&self.item_layout, capacity.get());

            self.data = NonNull::new(unsafe { malloc::alloc(new_layout) })
                .unwrap_or_else(|| malloc::handle_alloc_error(new_layout));
        }
    }

    pub unsafe fn realloc(&mut self, current_capacity: NonZeroUsize, new_capacity: NonZeroUsize) {
        if !self.is_zst() {
            let new_layout = array_layout(&self.item_layout, new_capacity.get());

            self.data = NonNull::new(unsafe {
                malloc::realloc(
                    self.data.as_ptr(),
                    array_layout_unchecked(&self.item_layout, current_capacity.get()),
                    new_layout.size(),
                )
            })
            .unwrap_or_else(|| malloc::handle_alloc_error(new_layout));
        }
    }

    pub unsafe fn dealloc(&mut self, current_capacity: usize, len: usize) {
        if current_capacity != 0 {
            unsafe {
                self.clear(len);
                if !self.is_zst() {
                    let layout = array_layout_unchecked(&self.item_layout, current_capacity);

                    alloc::alloc::dealloc(self.data.as_ptr(), layout);
                }
            }
        }
    }

    #[inline]
    pub unsafe fn clear(&mut self, len: usize) {
        if let Some(drop_fn) = self.drop_fn {
            let size = self.item_layout.size();
            let mut offset: usize = 0;

            let drop_guard = AbortOnPanic;

            for _ in 0..len {
                unsafe {
                    drop_fn(OwningPtr::new(self.data.byte_add(offset)));
                }
                offset += size;
            }

            ::core::mem::forget(drop_guard);
        }
    }

    #[inline(always)]
    pub const unsafe fn get_item(&self, index: usize) -> Ptr<'_> {
        let size = self.item_layout.size();
        unsafe { Ptr::new(self.data.byte_add(index * size)) }
    }

    #[inline(always)]
    pub const unsafe fn get_item_mut(&mut self, index: usize) -> PtrMut<'_> {
        let size = self.item_layout.size();
        unsafe { PtrMut::new(self.data.byte_add(index * size)) }
    }

    #[inline(always)]
    pub const unsafe fn as_slice<T>(&self, slice_len: usize) -> &[T] {
        unsafe { core::slice::from_raw_parts(self.data.as_ptr() as *const T, slice_len) }
    }

    #[inline(always)]
    pub const unsafe fn init_item(&mut self, index: usize, value: OwningPtr<'_>) {
        let size = self.item_layout.size();
        unsafe {
            let dst = self.data.as_ptr().byte_add(size * index);
            core::ptr::copy_nonoverlapping::<u8>(value.as_ptr(), dst, size);
        }
    }

    #[inline]
    pub unsafe fn replace_item(&mut self, index: usize, value: OwningPtr<'_>) {
        // SAFETY: The caller ensures that `index` fits in this vector.
        let size = self.item_layout.size();

        let src = value.as_ptr();
        let dst = unsafe { self.data.byte_add(size * index) };

        if let Some(drop_fn) = self.drop_fn {
            let drop_guard = AbortOnPanic;

            unsafe {
                drop_fn(OwningPtr::new(dst));
            }

            ::core::mem::forget(drop_guard);
        }

        // Overwriting the previous value.
        unsafe {
            core::ptr::copy_nonoverlapping::<u8>(src, dst.as_ptr(), size);
        }
    }

    #[inline(always)]
    #[must_use = "The returned pointer should be used to drop the removed element"]
    pub const unsafe fn remove_last(&mut self, last_index: usize) -> OwningPtr<'_> {
        let size = self.item_layout.size();
        unsafe { OwningPtr::new(self.data.byte_add(size * last_index)) }
    }

    #[inline(always)]
    pub unsafe fn drop_last(&mut self, last_index: usize) {
        if let Some(drop_fn) = self.drop_fn {
            let drop_guard = AbortOnPanic;

            let size = self.item_layout.size();
            unsafe {
                drop_fn(OwningPtr::new(self.data.byte_add(size * last_index)));
            }

            ::core::mem::forget(drop_guard);
        }
    }

    #[inline(always)]
    #[must_use = "The returned pointer should be used to drop the removed element"]
    pub const unsafe fn swap_remove_nonoverlapping(
        &mut self,
        index: usize,
        last_index: usize,
    ) -> OwningPtr<'_> {
        let size = self.item_layout.size();
        unsafe {
            let item = self.data.as_ptr().byte_add(size * index);
            let last = self.data.byte_add(size * last_index);
            core::ptr::swap_nonoverlapping::<u8>(item, last.as_ptr(), size);

            OwningPtr::new(last)
        }
    }

    #[inline]
    pub unsafe fn swap_remove_and_drop_nonoverlapping(&mut self, index: usize, last_index: usize) {
        let drop_fn = self.drop_fn;

        unsafe {
            let value = self.swap_remove_nonoverlapping(index, last_index);
            if let Some(drop_fn) = drop_fn {
                drop_fn(value);
            }
        }
    }
}

// -----------------------------------------------------------------------------
// alloc helper

#[cold]
#[inline(never)]
const fn invalid_size() -> ! {
    panic!("invalid size in `Layout::from_size_align`");
}

#[inline]
const fn array_layout(layout: &Layout, n: usize) -> Layout {
    let Some(alloc_size) = layout.size().checked_mul(n) else {
        invalid_size();
    };

    if alloc_size > isize::MAX as usize {
        invalid_size();
    }

    unsafe { Layout::from_size_align_unchecked(alloc_size, layout.align()) }
}

#[inline]
const unsafe fn array_layout_unchecked(layout: &Layout, n: usize) -> Layout {
    unsafe { Layout::from_size_align_unchecked(layout.size() * n, layout.align()) }
}
