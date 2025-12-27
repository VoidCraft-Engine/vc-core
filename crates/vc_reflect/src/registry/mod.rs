//! Provide type registry for non-object infomation querying.
//!
//! ## Menu
//!
//! - [`TypeTrait`]: A trait representing a capability supported by a type.
//! - [`FromType`]: A trait provide a function to crate a `TypeTrait` from a type.
//! - [`TypeMeta`]: A container including a [`TypeInfo`] and a [`TypeTrait`] table.
//! - [`GetTypeMeta`]: A trait provide a function to crate a `TypeMeta` from a type.
//! - [`TypeRegistry`]: A container for storaging and operating `TypeMeta`s.
//! - TypeTraits:
//!     - [`TypeTraitDefault`]: Provide [`Default`] capability for reflecion type.
//!     - [`TypeTraitFromPtr`]: Convert ptr to reflection reference.
//!     - [`TypeTraitFromReflect`]: Provide [`FromReflect`] support for deserialization.
//!     - [`TypeTraitSerialize`]: Provide serialization support for reflection type.
//!     - [`TypeTraitDeserialize`]: Provide deserialization support for reflection type.
//! - [`reflect_trait`]: a attribute macro, which generate a `Reflect{trait_name}` struct, can be used as [`TypeTrait`].
//!
//! ## auto_register
//!
//! See [`TypeRegistry::auto_register`] .
//!
//! We use [`inventory`] crate to implement static registration,
//! not all platforms support it (although major platforms do).
//!
//! The good news is that if it is not supported,
//! this function will directly return false without causing any errors.
//!
//! ### auto_register type menu
//!
//! - `()` `bool` `char` `f32` `f64`
//! - `i8` `i16` `i32` `i64` `i128` `isize`
//! - `u8` `u16` `u32` `u64` `u128` `usize`
//! - `String` `&'static str`
//! - `TypeId`
//! - `Atomic`: Bool I8-I64 U8-U64 Isize Usize (without Ptr)
//!
//!
//! [`reflect_trait`]: crate::derive::reflect_trait
//! [`FromReflect`]: crate::FromReflect
//! [`TypeInfo`]: crate::info::TypeInfo

mod type_trait;
pub use type_trait::TypeTrait;

mod type_meta;
pub use type_meta::{GetTypeMeta, TypeMeta};

mod from_type;
pub use from_type::FromType;

mod traits;
pub use traits::{
    TypeTraitDefault, TypeTraitDeserialize, TypeTraitFromPtr, TypeTraitFromReflect,
    TypeTraitSerialize,
};

mod type_registry;
pub use type_registry::{TypeRegistry, TypeRegistryArc};
