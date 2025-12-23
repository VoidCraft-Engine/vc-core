use alloc::boxed::Box;
use vc_os::sync::Arc;
use vc_utils::hash::HashMap;

use crate::{
    info::{
        CustomAttributes, Generics, NamedField, Type, TypePath, impl_custom_attributes_fn,
        impl_docs_fn, impl_generic_fn, impl_type_fn, impl_with_custom_attributes,
    },
    ops::Struct,
};

/// A container for compile-time named struct info.
///
/// # Examples
///
/// ```rust
/// # use vc_reflect::{derive::Reflect, info::{Typed, Type}};
///
/// #[derive(Reflect)]
/// struct A {
///     val: f32,
/// }
///
/// let info = <A as Typed>::type_info().as_struct().unwrap();
///
/// assert_eq!(info.field_len(), 1);
/// assert_eq!(info.index_of("val"), Some(0));
/// ```
#[derive(Clone, Debug)]
pub struct StructInfo {
    ty: Type,
    generics: Generics,
    fields: Box<[NamedField]>,
    field_names: Box<[&'static str]>,
    field_indices: HashMap<&'static str, usize>,
    // Use `Option` to reduce unnecessary heap requests (when empty content).
    custom_attributes: Option<Arc<CustomAttributes>>,
    #[cfg(feature = "reflect_docs")]
    docs: Option<&'static str>,
}

impl StructInfo {
    impl_docs_fn!(docs);
    impl_type_fn!(ty);
    impl_generic_fn!(generics);
    impl_custom_attributes_fn!(custom_attributes);
    impl_with_custom_attributes!(custom_attributes);

    /// Create a new [`StructInfo`].
    pub fn new<T: Struct + TypePath>(fields: &[NamedField]) -> Self {
        let field_indices = fields
            .iter()
            .enumerate()
            .map(|(index, field)| (field.name(), index))
            .collect();

        let field_names = fields.iter().map(NamedField::name).collect();

        Self {
            ty: Type::of::<T>(),
            generics: Generics::new(),
            fields: fields.to_vec().into_boxed_slice(),
            field_names,
            field_indices,
            custom_attributes: None,
            #[cfg(feature = "reflect_docs")]
            docs: None,
        }
    }

    /// Returns the field names in declaration order.
    #[inline]
    pub fn field_names(&self) -> &[&'static str] {
        &self.field_names
    }

    /// Returns the [`NamedField`] for the given `name`, if present.
    #[inline]
    pub fn field(&self, name: &str) -> Option<&NamedField> {
        self.field_indices
            .get(name)
            .map(|index| &self.fields[*index])
    }

    /// Returns the [`NamedField`] at the given index, if present.
    #[inline]
    pub fn field_at(&self, index: usize) -> Option<&NamedField> {
        self.fields.get(index)
    }

    /// Returns the index for the given field `name`, if present.
    #[inline]
    pub fn index_of(&self, name: &str) -> Option<usize> {
        self.field_indices.get(name).copied()
    }

    /// Returns an iterator over the fields in declaration order.
    #[inline]
    pub fn iter(&self) -> core::slice::Iter<'_, NamedField> {
        self.fields.iter()
    }

    /// Returns the number of fields.
    #[inline]
    pub fn field_len(&self) -> usize {
        self.fields.len()
    }
}
