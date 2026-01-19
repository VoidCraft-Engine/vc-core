#![doc = include_str!("../README.md")]
#![cfg_attr(docsrs, feature(doc_cfg))]
#![no_std]

// -----------------------------------------------------------------------------
// No STD Support

extern crate alloc;

// -----------------------------------------------------------------------------
// Modules

mod default;
mod range_invoke;
mod unsafe_deref;

pub mod extra;
pub mod hash;

// -----------------------------------------------------------------------------
// Top-level exports

pub use fastvec as vec;
pub use indexmap as index;

pub use default::default;
pub use unsafe_deref::UnsafeCellDeref;
