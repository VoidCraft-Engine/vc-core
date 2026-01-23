#![expect(unsafe_code, reason = "raw pointer is unsafe")]
#![expect(
    clippy::mut_from_ref,
    reason = "`PagePool` copies the data instead of returning the original reference."
)]

use alloc::alloc as malloc;
use alloc::vec::Vec;
use core::alloc::Layout;
use core::cell::UnsafeCell;
use core::marker::PhantomData;
use core::panic::{RefUnwindSafe, UnwindSafe};
use core::ptr::{self, NonNull};

use crate::UnsafeCellDeref;

// -----------------------------------------------------------------------------
// Page

const PAGE_SIZE: usize = 1024;

/// A memory page managed by the `PagePool`.
#[derive(Debug)]
struct Page {
    layout: Layout,
    data: NonNull<u8>,
    span: NonNull<u8>,
}

impl Drop for Page {
    fn drop(&mut self) {
        unsafe {
            malloc::dealloc(self.data.as_ptr(), self.layout);
        }
    }
}

// -----------------------------------------------------------------------------
// PagePool

/// A simple, thread-unsafe memory pool for fast allocations.
///
/// `PagePool` allocates memory in fixed-size pages and bump-allocates within each page.
/// It does not call `Drop::drop` on allocated objects, requiring manual resource
/// management when necessary.
///
/// # Safety
///
/// - **Not thread-safe**: Can only be used on the thread that created it.
/// - **No automatic cleanup**: Does not call destructors; users must manually
///   manage resources for types that require cleanup.
///
/// # Performance Characteristics
///
/// - **Fast allocations**: Bump allocation within pages is O(1).
/// - **Memory overhead**: Each page has some overhead for alignment padding.
/// - **Fragmentation**: Memory is not reused until the entire pool is cleared.
///
/// # Examples
///
/// ```
/// use vc_utils::extra::PagePool;
///
/// let pool = PagePool::new();
///
/// // Allocate an integer
/// let x = pool.alloc(42);
/// assert_eq!(*x, 42);
///
/// // Allocate a string slice
/// let s = pool.alloc_str("hello");
/// assert_eq!(s, "hello");
/// ```
#[derive(Debug)]
pub struct PagePool {
    pages: UnsafeCell<Vec<Page>>,
    _marker: PhantomData<*mut u8>,
}

impl UnwindSafe for PagePool {}
impl RefUnwindSafe for PagePool {}

impl Default for PagePool {
    #[inline(always)]
    fn default() -> Self {
        Self::new()
    }
}

impl PagePool {
    /// Creates a new, empty `PagePool`.
    ///
    /// The pool starts with no allocated pages. Pages are allocated
    /// on-demand when the first allocation request is made.
    #[inline(always)]
    pub const fn new() -> Self {
        Self {
            pages: UnsafeCell::new(Vec::new()),
            _marker: PhantomData,
        }
    }

    /// Allocates a value of type `T` in the pool and returns a mutable reference.
    ///
    /// The value is moved into the pool's memory. The returned reference is valid
    /// until the pool is cleared or destroyed.
    ///
    /// This is safe because `T` implements `Copy` and does not require `Drop`.
    ///
    /// # Panics
    ///
    /// This method may panic if the system allocator fails to allocate memory.
    ///
    /// # Examples
    ///
    /// ```
    /// use vc_utils::extra::PagePool;
    ///
    /// let pool = PagePool::new();
    /// let v1 = pool.alloc(123);
    /// let v2 = pool.alloc([1, 2, 3, 4]);
    ///
    /// assert_eq!(*v1, 123);
    /// assert_eq!(*v2, [1, 2, 3, 4]);
    /// ```
    #[inline(always)]
    pub fn alloc<T: Copy>(&self, val: T) -> &mut T {
        let layout = Layout::new::<T>();

        let p = self.alloc_layout(layout).cast::<T>();

        unsafe {
            ptr::write(p.as_ptr(), val);
            &mut *p.as_ptr()
        }
    }

    /// Allocates a value of type `T` in the pool and returns a mutable reference.
    ///
    /// The value is moved into the pool's memory. The returned reference is valid
    /// until the pool is cleared or destroyed.
    ///
    /// # Panics
    ///
    /// This method may panic if the system allocator fails to allocate memory.
    ///
    /// # Safety
    ///
    /// If `T` implements `Drop`, the caller must manually call the destructor
    /// before the `PagePool` is cleared or destroyed.
    #[inline(always)]
    pub unsafe fn alloc_unchecked<T>(&self, val: T) -> &mut T {
        let layout = Layout::new::<T>();

        let p = self.alloc_layout(layout).cast::<T>();

        unsafe {
            ptr::write(p.as_ptr(), val);
            &mut *p.as_ptr()
        }
    }

    /// Allocates a slice by copying its contents into the pool.
    ///
    /// Returns a mutable reference to the copied slice. The slice elements
    /// must be `Copy`.
    ///
    /// This is safe because `T` implements `Copy` and does not require `Drop`.
    ///
    /// # Panics
    ///
    /// This method may panic if the system allocator fails to allocate memory.
    ///
    /// # Examples
    ///
    /// ```
    /// use vc_utils::extra::PagePool;
    ///
    /// let pool = PagePool::new();
    /// let original = [1, 2, 3, 4, 5];
    /// let slice = pool.alloc_slice(&original);
    ///
    /// assert_eq!(*slice, original);
    /// ```
    #[inline(always)]
    pub fn alloc_slice<T: Copy>(&self, slice: &[T]) -> &mut [T] {
        let layout = Layout::for_value(slice);
        let ptr = self.alloc_layout(layout).cast::<T>();

        unsafe {
            // Copy the slice contents
            ptr::copy_nonoverlapping(slice.as_ptr(), ptr.as_ptr(), slice.len());
            core::slice::from_raw_parts_mut(ptr.as_ptr(), slice.len())
        }
    }

    /// Allocates a slice by copying its contents into the pool.
    ///
    /// Returns a mutable reference to the copied slice.
    ///
    /// # Panics
    ///
    /// This method may panic if the system allocator fails to allocate memory.
    ///
    /// # Safety
    ///
    /// If `T` implements `Drop`, the caller must manually call the destructor
    /// before the `PagePool` is cleared or destroyed.
    #[inline(always)]
    pub unsafe fn alloc_slice_unchecked<T>(&self, slice: &[T]) -> &'_ mut [T] {
        let layout = Layout::for_value(slice);
        let ptr = self.alloc_layout(layout).cast::<T>();

        unsafe {
            // Copy the slice contents
            ptr::copy_nonoverlapping(slice.as_ptr(), ptr.as_ptr(), slice.len());
            core::slice::from_raw_parts_mut(ptr.as_ptr(), slice.len())
        }
    }

    /// Allocates a string slice by copying its contents into the pool.
    ///
    /// Returns a reference to the copied string. The input must be
    /// valid UTF-8.
    ///
    /// # Panics
    ///
    /// This method may panic if the system allocator fails to allocate memory.
    ///
    /// # Examples
    ///
    /// ```
    /// use vc_utils::extra::PagePool;
    ///
    /// let pool = PagePool::new();
    /// let s = pool.alloc_str("Hello, world!");
    /// assert_eq!(s, "Hello, world!");
    /// ```
    #[inline(always)]
    pub fn alloc_str(&self, s: &str) -> &str {
        let bytes = self.alloc_slice(s.as_bytes());

        unsafe {
            // SAFETY: The input is valid UTF-8, and we're copying it verbatim
            core::str::from_utf8_unchecked_mut(bytes)
        }
    }

    /// Allocates memory with the given layout and returns a pointer to it.
    ///
    /// The returned pointer is aligned according to the layout's alignment
    /// requirement. The memory is uninitialized and should be initialized
    /// by the caller.
    ///
    /// # Panics
    ///
    /// This method may panic if the system allocator fails to allocate memory.
    ///
    /// # Examples
    ///
    /// ```
    /// use vc_utils::extra::PagePool;
    /// use core::alloc::Layout;
    ///
    /// let pool = PagePool::new();
    /// let layout = Layout::new::<i32>();
    /// let ptr = pool.alloc_layout(layout);
    ///
    /// unsafe {
    ///     // Initialize the memory
    ///     ptr.cast::<i32>().as_ptr().write(42);
    /// }
    /// ```
    #[inline]
    pub fn alloc_layout(&self, layout: Layout) -> NonNull<u8> {
        let pages = unsafe { self.pages.deref_mut() };

        if let Some(page) = pages.last_mut() {
            unsafe {
                let span = page.span;

                // Ensure aligned
                let align_mask = layout.align() - 1;
                let current_addr = span.as_ptr().addr();
                let aligned_addr = (current_addr + align_mask) & !align_mask;
                let aligned_ptr = NonNull::new_unchecked(aligned_addr as *mut u8);

                // get new span
                let new_span = aligned_ptr.byte_add(layout.size());
                let page_end = page.data.byte_add(page.layout.size());

                // Ensure the memory is enough.
                if new_span <= page_end {
                    page.span = new_span;
                    return span;
                }
            }
        }
        self.alloc_layout_slow(layout)
    }

    #[cold]
    #[inline(never)]
    fn alloc_layout_slow(&self, layout: Layout) -> NonNull<u8> {
        let align = layout.align();
        let size = layout.size();

        let page_size = if PAGE_SIZE >= size { PAGE_SIZE } else { size };

        // Ensure that page_size if aligned.
        let align_mask = layout.align() - 1;
        let page_size = (align_mask + page_size) & !align_mask;

        unsafe {
            let page_layout = Layout::from_size_align_unchecked(page_size, align);
            let data = NonNull::new(malloc::alloc(page_layout))
                .unwrap_or_else(|| malloc::handle_alloc_error(page_layout));

            let page = Page {
                layout: page_layout,
                data,
                span: data.add(size),
            };

            self.pages.deref_mut().push(page);

            data
        }
    }
}
