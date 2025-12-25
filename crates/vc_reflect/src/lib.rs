#![cfg_attr(docsrs, expect(internal_features, reason = "needed for fake_variadic"))]
#![cfg_attr(docsrs, feature(doc_cfg, rustdoc_internals))]
#![no_std]

// Required to make proc macros work in crate itself.
//
// Usually, we need to use `crate` in the crate itself and use `vc_reflect` in doc testing.
// But `macro_utils::Manifest` can only choose one, so we must have an
// `extern self` to ensure `vc_reflect` can be used as an alias for `crate`.
extern crate self as vc_reflect;

extern crate alloc;

pub mod cfg {
    pub use vc_cfg::std;
    vc_cfg::define_alias! {
        #[cfg(feature = "auto_register")] => auto_register,
        #[cfg(feature = "reflect_docs")] => reflect_docs,
    }
}

mod reflection;
pub use reflection::{FromReflect, Reflect, reflect_hasher};

pub mod impls;
pub mod info;
pub mod ops;
pub mod registry;

pub mod __macro_exports;

pub use vc_reflect_derive as derive;

pub mod prelude {
    pub use crate::{FromReflect, Reflect};
}
