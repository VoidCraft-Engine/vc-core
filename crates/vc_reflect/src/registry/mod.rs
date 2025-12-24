//! # VoidCraft Reflection Registry
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
