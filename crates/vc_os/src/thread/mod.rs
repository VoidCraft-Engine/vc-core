//! Provides `sleep` for all platforms.

pub use thread_impl::*;

crate::cfg::switch! {
    crate::cfg::std => {
        use std::thread as thread_impl;
    }
    _ => {
        mod fallback;
        use fallback as thread_impl;
    }
}
