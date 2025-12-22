#![doc = include_str!("../README.md")]
#![cfg_attr(docsrs, feature(doc_cfg))]
#![no_std]

pub mod cfg {
    pub(crate) use vc_cfg::switch;
    pub use vc_cfg::{panic_abort, panic_unwind, std};

    vc_cfg::define_alias! {
        #[cfg(all(target_arch = "wasm32", feature = "web"))] => web,
        #[cfg(feature = "critical-section")] => critical_section,
    }
}

cfg::std! {
    extern crate std;
}

extern crate alloc;

pub mod sync;
pub mod thread;
pub mod time;

#[doc(hidden)]
pub mod exports {
    crate::cfg::web! {
        pub use js_sys;
        pub use wasm_bindgen;
        pub use wasm_bindgen_futures;
    }
}
