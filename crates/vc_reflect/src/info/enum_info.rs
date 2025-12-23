use alloc::{boxed::Box, format, string::String};
use vc_os::sync::Arc;
use vc_utils::hash::HashMap;

use crate::{
    info::{
        CustomAttributes, Generics, Type, TypePath, VariantInfo, impl_custom_attributes_fn,
        impl_docs_fn, impl_generic_fn, impl_type_fn, impl_with_custom_attributes,
    },
    ops::Enum,
};

/// A container for compile-time enum info, size = 136 (exclude `docs`).
///
/// # Examples
///
/// ```rust
/// use vc_reflect::info::{Typed, EnumInfo};
///
/// let info = <Option<i32> as Typed>::type_info().as_enum().unwrap();
/// assert!(info.contains_variant("Some"));
/// assert!(info.variant("None").is_some());
/// ```
#[derive(Clone, Debug)]
pub struct EnumInfo {
    ty: Type,
    generics: Generics,
    variants: Box<[VariantInfo]>,
    variant_names: Box<[&'static str]>,
    variant_indices: HashMap<&'static str, usize>,
    // Use `Option` to reduce unnecessary heap requests (when empty content).
    custom_attributes: Option<Arc<CustomAttributes>>,
    #[cfg(feature = "reflect_docs")]
    docs: Option<&'static str>,
}

impl EnumInfo {
    impl_type_fn!(ty);
    impl_docs_fn!(docs);
    impl_generic_fn!(generics);
    impl_custom_attributes_fn!(custom_attributes);
    impl_with_custom_attributes!(custom_attributes);

    /// Creates a new [`EnumInfo`].
    ///
    /// # Arguments
    ///
    /// - `variants`: The variants of this enum in the order they are defined
    pub fn new<TEnum: Enum + TypePath>(variants: &[VariantInfo]) -> Self {
        let variant_indices = variants
            .iter()
            .enumerate()
            .map(|(index, variant)| (variant.name(), index))
            .collect();

        let variant_names = variants.iter().map(VariantInfo::name).collect();

        Self {
            ty: Type::of::<TEnum>(),
            generics: Generics::new(),
            variants: variants.to_vec().into_boxed_slice(),
            variant_names,
            variant_indices,
            custom_attributes: None,
            #[cfg(feature = "reflect_docs")]
            docs: None,
        }
    }

    /// Returns the list of variant names in declaration order.
    #[inline]
    pub fn variant_names(&self) -> &[&'static str] {
        &self.variant_names
    }

    /// Returns the [`VariantInfo`] for the given variant name, if present.
    #[inline]
    pub fn variant(&self, name: &str) -> Option<&VariantInfo> {
        self.variant_indices
            .get(name)
            .map(|index| &self.variants[*index])
    }

    /// Returns the [`VariantInfo`] at the given index, if present.
    #[inline]
    pub fn variant_at(&self, index: usize) -> Option<&VariantInfo> {
        self.variants.get(index)
    }

    /// Returns the index for the given variant name, if present.
    #[inline]
    pub fn index_of(&self, name: &str) -> Option<usize> {
        self.variant_indices.get(name).copied()
    }

    /// Returns the full path for a variant name, e.g. `Type::Variant`.
    #[inline]
    pub fn variant_path(&self, name: &str) -> String {
        format!("{}::{name}", self.type_path())
    }

    /// Returns `true` if a variant with the given name exists.
    #[inline]
    pub fn contains_variant(&self, name: &str) -> bool {
        self.variant_indices.contains_key(name)
    }

    /// Returns an iterator over the variants in declaration order.
    #[inline]
    pub fn iter(&self) -> core::slice::Iter<'_, VariantInfo> {
        self.variants.iter()
    }

    /// Returns the number of variants.
    #[inline]
    pub fn variant_len(&self) -> usize {
        self.variants.len()
    }
}
