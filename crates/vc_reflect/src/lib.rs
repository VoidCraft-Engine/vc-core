//! Runtime reflection system for Rust.
//!
//! This library implements a dynamic reflection system in Rust, designed to provide
//! comprehensive runtime type information and data manipulation capabilities.
//!
//! While it's a general-purpose reflection system suitable for various scenarios,
//! it's specifically designed for the VoidCraft Engine and may include
//! platform-specific dependencies from VoidCraft that could be redundant in
//! non-game-engine contexts.
//!
//! # Goals
//!
//! As a dynamic reflection system, this library aims to support:
//!
//! - **Runtime Type Information**:
//!     - Basic information: type names, TypeId, field lists, generic parameters
//!     - Custom attributes: similar to C# attributes, allowing user-defined metadata on types
//!     - Type documentation (optional): useful for game engine editors and tools
//!     - See more information in [`vc_reflect::info`].
//!
//! - **Data Manipulation**:
//!     - Type erasure: achieve effects similar to `Object` in other languages through trait objects
//!     - Specialized interfaces through reflection subtraits: `Struct`, `Enum`, etc.
//!     - Dynamic object composition with ability to apply to concrete types when needed
//!     - See more information in [`vc_reflect::ops`] and [`vc_reflect::Reflect`].
//!
//! - **Type Registration**:
//!     - Metadata: type metadata containing both type information and available function pointers
//!     - Registry: storage system for metadata enabling type information retrieval without instances
//!     - Auto-registration (optional): type registration through static initialization
//!     - See more information in [`vc_reflect::registry`].
//!
//! - **Trait Reflection**:
//!     - Trait reflection based on registration system, enabling dynamic trait object retrieval
//!     - See more infomation in [`registry::TypeTrait`] and [`derive::reflect_trait`]
//!
//! - **Reflection Macros**:
//!     - Automatic generation of reflection implementations for types
//!     - See more infomation in [`vc_reflect::derive`].
//!
//! - **Serialization**:
//!     - Serialization system based on registry, allowing types without explicit `Serialize`/`Deserialize` implementations
//!
//! - **Path-Based Access**:
//!     - Multi-level data access via string paths (struct fields, array elements, etc.)
//!     - See more infomation in [`vc_reflect::access`].
//!
//! # Available Features
//!
//! ## `default`
//!
//! Includes `std` and `auto_register`.
//!
//! ## `std`
//!
//! Enabled by default.
//!
//! Provides reflection implementations for standard library containers like `HashMap`.
//!
//! ## `auto_register`
//!
//! Enabled by default.
//!
//! Enables automatic type registration through static initialization.
//!
//! When disabled, auto-registration functions remain available but perform no operation.
//!
//! See [`TypeRegistry::auto_register`](crate::registry::TypeRegistry::auto_register) for details.
//!
//! ## `reflect_docs`
//!
//! Enables type documentation collection. Automatically gathers standard documentation
//! from `#[doc = "..."]` attributes. Disabled by default.
//!
//! When disabled, documentation functions remain available but always return empty values.
//!
//! See [`TypeInfo::docs`](crate::info::TypeInfo::docs) for details.
//!
//! [`Struct`]: ops::Struct
//! [`Enum`]: ops::Enum
//! [`Tuple`]: ops::Tuple
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

/// See [vc_cfg] .
pub mod cfg {
    pub use vc_cfg::std;
    vc_cfg::define_alias! {
        #[cfg(feature = "auto_register")] => auto_register,
        #[cfg(feature = "reflect_docs")] => reflect_docs,
    }
}

mod reflection;
pub use reflection::{FromReflect, Reflect, reflect_hasher};

pub mod access;
pub mod impls;
pub mod info;
pub mod ops;
pub mod registry;

pub mod __macro_exports;

pub use vc_reflect_derive as derive;

pub mod prelude {
    pub use crate::{FromReflect, Reflect, registry::TypeRegistry};
}
