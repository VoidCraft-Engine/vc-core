pub(crate) struct AbortOnDrop;

impl Drop for AbortOnDrop {
    #[cold]
    #[inline(never)]
    fn drop(&mut self) {
        crate::cfg::std! {
            if {
                std::eprintln!("Aborting due to allocator error.");
                std::process::abort();
            } else {
                panic!("Aborting due to allocator error.");
            }
        }
    }
}
