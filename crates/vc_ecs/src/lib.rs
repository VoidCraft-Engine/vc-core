//! TODO
#![cfg_attr(docsrs, expect(internal_features, reason = "needed for fake_variadic"))]
#![cfg_attr(docsrs, feature(doc_cfg, rustdoc_internals))]
#![allow(clippy::missing_safety_doc)]
#![no_std]

// -----------------------------------------------------------------------------
// Compilation config

/// Some macros used for compilation control.
pub mod cfg {
    pub use vc_os::cfg::{std, web};

    vc_cfg::define_alias! {
        // In some places, `#[cfg]` is directly used instead of `debug!`.
        // When adjusting this, be sure to modify it in other places accordingly.
        #[cfg(any(feature = "debug", debug_assertions))] => debug,
    }
}

// -----------------------------------------------------------------------------
// Extern Self

// Usually, we need to use `crate` in the crate itself and use `vc_ecs` in doc testing.
// But `macro_utils::Manifest` can only choose one, so we must have an
// `extern self` to ensure `vc_ecs` can be used as an alias for `crate`.
extern crate self as vc_ecs;

// -----------------------------------------------------------------------------
// no_std support

crate::cfg::std! { extern crate std; }

extern crate alloc;

// -----------------------------------------------------------------------------
// Modules

pub mod tick;

pub mod utils;

pub mod change_detection;

pub mod batching;
pub mod intern;
pub mod label;
pub mod name;
pub mod resource;
pub mod system;

pub mod archetype;

pub mod component;
pub mod entity;
pub mod event;
pub mod lifecycle;
pub mod relationship;
pub mod storage;
pub mod world;

// -----------------------------------------------------------------------------
// Exports

// -----------------------------------------------------------------------------
// Macro-Utils

pub mod __macro_utils {
    pub use alloc::boxed::Box;
    pub use alloc::vec::Vec;
}
