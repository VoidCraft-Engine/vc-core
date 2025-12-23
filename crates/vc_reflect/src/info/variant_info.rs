use alloc::boxed::Box;
use core::{error, fmt};
use vc_os::sync::Arc;
use vc_utils::hash::HashMap;

use crate::info::{
    CustomAttributes, NamedField, UnnamedField,
    attributes::{impl_custom_attributes_fn, impl_with_custom_attributes},
    docs_macro::impl_docs_fn,
};

/// Represents the kind/form of an enum variant.
#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub enum VariantKind {
    Struct,
    Tuple,
    Unit,
}

impl fmt::Display for VariantKind {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Struct => f.pad("Struct"),
            Self::Tuple => f.pad("Tuple"),
            Self::Unit => f.pad("Unit"),
        }
    }
}

/// Metadata for struct style enum variants.
///
/// # Examples
///
/// ```
/// # use vc_reflect::{derive::Reflect, info::{Typed}};
///
/// #[derive(Reflect)]
/// enum MyEnum {
///   A {  // <-- struct variant
///     foo: usize
///   },
///   Other{ /* ... */ },
/// }
///
/// let info = MyEnum::type_info()
///     .as_enum().unwrap()
///     .variant("A").unwrap()
///     .as_struct_variant().unwrap();
///
/// assert_eq!(info.name(), "A");
/// assert_eq!(info.field_len(), 1);
/// ```
#[derive(Clone, Debug)]
pub struct StructVariantInfo {
    name: &'static str,
    fields: Box<[NamedField]>,
    field_names: Box<[&'static str]>,
    field_indices: HashMap<&'static str, usize>,
    // Use `Option` to avoid allocating when there are no custom attributes.
    custom_attributes: Option<Arc<CustomAttributes>>,
    #[cfg(feature = "reflect_docs")]
    docs: Option<&'static str>,
}

impl StructVariantInfo {
    impl_docs_fn!(docs);
    impl_custom_attributes_fn!(custom_attributes);
    impl_with_custom_attributes!(custom_attributes);

    /// Create a new [`StructVariantInfo`].
    pub fn new(name: &'static str, fields: &[NamedField]) -> Self {
        let field_indices = fields
            .iter()
            .enumerate()
            .map(|(index, field)| (field.name(), index))
            .collect();

        let field_names = fields.iter().map(NamedField::name).collect();

        Self {
            name,
            fields: fields.to_vec().into_boxed_slice(),
            field_names,
            field_indices,
            custom_attributes: None,
            #[cfg(feature = "reflect_docs")]
            docs: None,
        }
    }

    /// Returns the name of this variant.
    #[inline]
    pub const fn name(&self) -> &'static str {
        self.name
    }

    /// Returns a slice containing the field names in declaration order.
    #[inline]
    pub fn field_names(&self) -> &[&'static str] {
        &self.field_names
    }

    /// Returns the field with the given `name`, if present.
    #[inline]
    pub fn field(&self, name: &str) -> Option<&NamedField> {
        self.field_indices
            .get(name)
            .map(|index| &self.fields[*index])
    }

    /// Returns the field at the given index, if present.
    #[inline]
    pub fn field_at(&self, index: usize) -> Option<&NamedField> {
        self.fields.get(index)
    }

    /// Returns the index of the field with the given `name`, if present.
    #[inline]
    pub fn index_of(&self, name: &str) -> Option<usize> {
        self.field_indices.get(name).copied()
    }

    /// Returns an iterator over the fields of this variant in declaration order.
    #[inline]
    pub fn iter(&self) -> core::slice::Iter<'_, NamedField> {
        self.fields.iter()
    }

    /// Returns the total number of fields in this variant.
    #[inline]
    pub fn field_len(&self) -> usize {
        self.fields.len()
    }
}

/// Metadata for tuple style enum variants.
///
/// # Examples
///
/// ```
/// # use vc_reflect::{derive::Reflect, info::{Typed}};
///
/// #[derive(Reflect)]
/// enum MyEnum {
///   A(usize),  // <-- tuple variant
///   Other{ /* ... */ },
/// }
///
/// let info = MyEnum::type_info()
///     .as_enum().unwrap()
///     .variant("A").unwrap()
///     .as_tuple_variant().unwrap();
///
/// assert_eq!(info.name(), "A");
/// assert_eq!(info.field_len(), 1);
/// ```
#[derive(Clone, Debug)]
pub struct TupleVariantInfo {
    name: &'static str,
    fields: Box<[UnnamedField]>,
    // Use `Option` to reduce unnecessary heap requests (when empty content).
    custom_attributes: Option<Arc<CustomAttributes>>,
    #[cfg(feature = "reflect_docs")]
    docs: Option<&'static str>,
}

impl TupleVariantInfo {
    impl_docs_fn!(docs);
    impl_custom_attributes_fn!(custom_attributes);
    impl_with_custom_attributes!(custom_attributes);

    /// Create a new [`TupleVariantInfo`].
    pub fn new(name: &'static str, fields: &[UnnamedField]) -> Self {
        // Not inline: Consistent with StructVariantInfo
        Self {
            name,
            fields: fields.to_vec().into_boxed_slice(),
            custom_attributes: None,
            #[cfg(feature = "reflect_docs")]
            docs: None,
        }
    }

    /// The name of this variant.
    #[inline]
    pub const fn name(&self) -> &'static str {
        self.name
    }

    /// Get the field at the given index.
    #[inline]
    pub fn field_at(&self, index: usize) -> Option<&UnnamedField> {
        self.fields.get(index)
    }

    /// Iterate over the fields of this variant.
    #[inline]
    pub fn iter(&self) -> core::slice::Iter<'_, UnnamedField> {
        self.fields.iter()
    }

    /// The total number of fields in this variant.
    #[inline]
    pub fn field_len(&self) -> usize {
        self.fields.len()
    }
}

/// Metadata for unit enum variants.
///
/// # Examples
///
/// ```
/// # use vc_reflect::{derive::Reflect, info::{Typed}};
///
/// #[derive(Reflect)]
/// enum MyEnum {
///   A,  // <-- unit variant
///   Other{ /* ... */ },
/// }
///
/// let info = MyEnum::type_info()
///     .as_enum().unwrap()
///     .variant("A").unwrap()
///     .as_unit_variant().unwrap();
///
/// assert_eq!(info.name(), "A");
/// ```
#[derive(Clone, Debug)]
pub struct UnitVariantInfo {
    name: &'static str,
    // Use `Option` to reduce unnecessary heap requests (when empty content).
    custom_attributes: Option<Arc<CustomAttributes>>,
    #[cfg(feature = "reflect_docs")]
    docs: Option<&'static str>,
}

impl UnitVariantInfo {
    impl_docs_fn!(docs);
    impl_custom_attributes_fn!(custom_attributes);
    impl_with_custom_attributes!(custom_attributes);

    /// Create a new [`UnitVariantInfo`].
    #[inline]
    pub const fn new(name: &'static str) -> Self {
        Self {
            name,
            custom_attributes: None,
            #[cfg(feature = "reflect_docs")]
            docs: None,
        }
    }

    /// The name of this variant.
    #[inline]
    pub const fn name(&self) -> &'static str {
        self.name
    }
}

/// A [`VariantInfo`]-specific error.
#[derive(Debug)]
pub struct VariantKindError {
    /// Expected variant type.
    expected: VariantKind,
    /// Received variant type.
    received: VariantKind,
}

impl fmt::Display for VariantKindError {
    #[inline]
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "variant kind mismatch: expected {:?}, received {:?}",
            self.expected, self.received
        )
    }
}

impl error::Error for VariantKindError {}

/// Container for compile-time enum variant info.
#[derive(Clone, Debug)]
pub enum VariantInfo {
    /// See [`StructVariantInfo`].
    Struct(StructVariantInfo),
    /// See [`TupleVariantInfo`].
    Tuple(TupleVariantInfo),
    /// See [`UnitVariantInfo`].
    Unit(UnitVariantInfo),
}

macro_rules! impl_cast_fn {
    ($name:ident : $kind:ident => $info:ident) => {
        pub fn $name(&self) -> Result<&$info, VariantKindError> {
            match self {
                Self::$kind(info) => Ok(info),
                _ => Err(VariantKindError {
                    expected: VariantKind::$kind,
                    received: self.variant_kind(),
                }),
            }
        }
    };
}

impl VariantInfo {
    impl_cast_fn!(as_struct_variant: Struct => StructVariantInfo);
    impl_cast_fn!(as_tuple_variant: Tuple => TupleVariantInfo);
    impl_cast_fn!(as_unit_variant: Unit => UnitVariantInfo);

    pub fn custom_attributes(&self) -> &CustomAttributes {
        match self {
            Self::Struct(info) => info.custom_attributes(),
            Self::Tuple(info) => info.custom_attributes(),
            Self::Unit(info) => info.custom_attributes(),
        }
    }

    impl_custom_attributes_fn!();

    /// The name of the enum variant.
    pub const fn name(&self) -> &'static str {
        match self {
            Self::Struct(info) => info.name(),
            Self::Tuple(info) => info.name(),
            Self::Unit(info) => info.name(),
        }
    }

    /// Returns the [kind] of this variant.
    ///
    /// [kind]: VariantKind
    pub const fn variant_kind(&self) -> VariantKind {
        match self {
            Self::Struct(_) => VariantKind::Struct,
            Self::Tuple(_) => VariantKind::Tuple,
            Self::Unit(_) => VariantKind::Unit,
        }
    }

    /// The docstring of the underlying variant, if any.
    ///
    /// If `reflect_docs` feature is not enabled, this function always return `None`.
    /// So you can use this without worrying about compilation options.
    #[cfg_attr(not(feature = "reflect_docs"), inline(always))]
    pub const fn docs(&self) -> Option<&str> {
        #[cfg(not(feature = "reflect_docs"))]
        return None;

        #[cfg(feature = "reflect_docs")]
        match self {
            Self::Struct(info) => info.docs(),
            Self::Tuple(info) => info.docs(),
            Self::Unit(info) => info.docs(),
        }
    }
}
