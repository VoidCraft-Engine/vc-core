use alloc::borrow::Cow;
use core::fmt;
use vc_os::sync::Arc;

use crate::info::{
    CustomAttributes, Type, TypeInfo, Typed, impl_custom_attributes_fn, impl_docs_fn, impl_type_fn,
    impl_with_custom_attributes,
};

/// Information for a named (struct) field.
#[derive(Clone, Debug)]
pub struct NamedField {
    ty: Type,
    name: &'static str,
    // `TypeInfo` is created on first access; using a function pointer delays it.
    type_info: fn() -> &'static TypeInfo,
    // Use `Option` to reduce unnecessary heap requests (when empty content).
    custom_attributes: Option<Arc<CustomAttributes>>,
    #[cfg(feature = "reflect_docs")]
    docs: Option<&'static str>,
}

impl NamedField {
    impl_docs_fn!(docs);
    impl_type_fn!(ty);
    impl_custom_attributes_fn!(custom_attributes);
    impl_with_custom_attributes!(custom_attributes);

    /// Creates a new [`NamedField`] for the given field `name` and type `T`.
    #[inline]
    pub const fn new<T: Typed>(name: &'static str) -> Self {
        Self {
            name,
            type_info: T::type_info,
            ty: Type::of::<T>(),
            custom_attributes: None,
            #[cfg(feature = "reflect_docs")]
            docs: None,
        }
    }

    /// Returns the field name.
    #[inline]
    pub const fn name(&self) -> &'static str {
        self.name
    }

    /// Returns the field's [`TypeInfo`].
    #[inline]
    pub fn type_info(&self) -> &'static TypeInfo {
        (self.type_info)()
    }
}

/// Information for an unnamed (tuple) field.
#[derive(Clone, Debug)]
pub struct UnnamedField {
    ty: Type,
    index: usize,
    // `TypeInfo` is created on first access; using a function pointer delays it.
    type_info: fn() -> &'static TypeInfo,
    // Use `Option` to reduce unnecessary heap requests (when empty content).
    custom_attributes: Option<Arc<CustomAttributes>>,
    #[cfg(feature = "reflect_docs")]
    docs: Option<&'static str>,
}

impl UnnamedField {
    impl_docs_fn!(docs);
    impl_type_fn!(ty);
    impl_custom_attributes_fn!(custom_attributes);
    impl_with_custom_attributes!(custom_attributes);

    /// Creates a new [`UnnamedField`] for the field at `index` with type `T`.
    #[inline]
    pub const fn new<T: Typed>(index: usize) -> Self {
        Self {
            index,
            type_info: T::type_info,
            ty: Type::of::<T>(),
            custom_attributes: None,
            #[cfg(feature = "reflect_docs")]
            docs: None,
        }
    }

    /// Returns the field index (position in the tuple struct).
    #[inline]
    pub const fn index(&self) -> usize {
        self.index
    }

    /// Returns the field's [`TypeInfo`].
    #[inline]
    pub fn type_info(&self) -> &'static TypeInfo {
        (self.type_info)()
    }
}

/// Represents a field identifier, either named or unnamed.
///
/// Primarily used for formatting field identifiers in error messages and
/// diagnostics.
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum FieldId {
    Named(Cow<'static, str>),
    Unnamed(usize),
}

impl fmt::Display for FieldId {
    #[inline]
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Named(name) => fmt::Display::fmt(name, f),
            Self::Unnamed(name) => fmt::Display::fmt(name, f),
        }
    }
}
