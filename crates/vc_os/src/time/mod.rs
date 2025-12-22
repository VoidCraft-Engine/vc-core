//! Provides `Instant` for all platforms.

pub use core::time::{Duration, TryFromFloatSecsError};
pub use time_impl::{Instant, SystemTime, SystemTimeError};

crate::cfg::switch! {
    crate::cfg::web => {
        use ::web_time as time_impl;
    }
    crate::cfg::std => {
        use ::std::time as time_impl;
    }
    _ => {
        mod fallback;
        use fallback as time;
    }
}
