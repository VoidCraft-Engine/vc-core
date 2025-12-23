use core::{error, fmt};

use crate::info::{
    ArrayInfo, CustomAttributes, EnumInfo, Generics, ListInfo, MapInfo, OpaqueInfo, SetInfo,
    StructInfo, TupleInfo, TupleStructInfo, Type,
};

/// An enumeration of the "kinds" of a reflected type.
///
/// Each kind corresponds to a specific reflection trait,
/// such as [`Struct`](crate::ops::Struct) or [`List`](crate::ops::List),
/// which itself corresponds to the kind or structure of a type.
///
/// A [`ReflectKind`] is obtained via [`Reflect::reflect_kind`](crate::Reflect::reflect_kind),
/// or via [`ReflectRef::kind`](crate::ops::ReflectRef),
/// [`ReflectMut::kind`](crate::ops::ReflectMut) or [`ReflectOwned::kind`](crate::ops::ReflectOwned).
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum ReflectKind {
    Struct,
    TupleStruct,
    Tuple,
    List,
    Array,
    Map,
    Set,
    Enum,
    Opaque,
}

impl fmt::Display for ReflectKind {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Struct => f.pad("Struct"),
            Self::TupleStruct => f.pad("TupleStruct"),
            Self::Tuple => f.pad("Tuple"),
            Self::List => f.pad("List"),
            Self::Array => f.pad("Array"),
            Self::Map => f.pad("Map"),
            Self::Set => f.pad("Set"),
            Self::Enum => f.pad("Enum"),
            Self::Opaque => f.pad("Opaque"),
        }
    }
}

/// Error returned when a `TypeInfo` value is not the expected `ReflectKind`.
#[derive(Debug)]
pub struct ReflectKindError {
    pub expected: ReflectKind,
    pub received: ReflectKind,
}

impl fmt::Display for ReflectKindError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "reflect kind mismatch: expected {}, received {}",
            self.expected, self.received
        )
    }
}

impl error::Error for ReflectKindError {}

/// Compile-time type information for various reflected types.
///
/// Generally, for any given type, this value can be retrieved in one of four ways:
///
/// 1. [`Typed::type_info`]
/// 2. [`DynamicTyped::reflect_type_info`]
/// 3. [`Reflect::represented_type_info`]
/// 4. [`TypeRegistry::get_type_info`]
///
/// Each returns a static reference to [`TypeInfo`], but they all have their own use cases.
/// For example, if you know the type at compile time, [`Typed::type_info`] is probably
/// the simplest. If you have a `dyn Reflect` you can use [`DynamicTyped::reflect_type_info`].
/// If you only care about data content (such as serialization), then [`Reflect::represented_type_info`] should be used.
/// Lastly, if all you have is a [`TypeId`] or [type path], you will need to go through
/// [`TypeRegistry::get_type_info`].
///
/// You may also opt to use [`TypeRegistry::get_type_info`] in place of the other methods simply because
/// it can be more performant. This is because those other methods may require attaining a lock on
/// the static [`TypeInfo`], while the registry simply checks a map.
///
/// [type path]: crate::info::TypePath
/// [`Typed::type_info`]: crate::info::Typed::type_info
/// [`DynamicTyped::reflect_type_info`]: crate::info::DynamicTyped::reflect_type_info
/// [`Reflect::represented_type_info`]: crate::Reflect::represented_type_info
/// [`TypeRegistry::get_type_info`]: crate::registry::TypeRegistry::get_type_info
#[derive(Debug, Clone)]
pub enum TypeInfo {
    Struct(StructInfo),
    TupleStruct(TupleStructInfo),
    Tuple(TupleInfo),
    List(ListInfo),
    Array(ArrayInfo),
    Map(MapInfo),
    Set(SetInfo),
    Enum(EnumInfo),
    Opaque(OpaqueInfo),
}

// Helper macro that implements type-safe accessor methods like `as_struct`.
macro_rules! impl_cast_method {
    ($name:ident : $kind:ident => $info:ident) => {
        pub const fn $name(&self) -> Result<&$info, ReflectKindError> {
            match self {
                Self::$kind(info) => Ok(info),
                _ => Err(ReflectKindError {
                    expected: ReflectKind::$kind,
                    received: self.kind(),
                }),
            }
        }
    };
}

impl TypeInfo {
    impl_cast_method!(as_struct: Struct => StructInfo);
    impl_cast_method!(as_tuple_struct: TupleStruct => TupleStructInfo);
    impl_cast_method!(as_tuple: Tuple => TupleInfo);
    impl_cast_method!(as_list: List => ListInfo);
    impl_cast_method!(as_array: Array => ArrayInfo);
    impl_cast_method!(as_map: Map => MapInfo);
    impl_cast_method!(as_set: Set => SetInfo);
    impl_cast_method!(as_enum: Enum => EnumInfo);
    impl_cast_method!(as_opaque: Opaque => OpaqueInfo);

    /// Returns the underlying [`Type`] metadata for this `TypeInfo`.
    pub const fn ty(&self) -> &Type {
        match self {
            Self::Struct(info) => info.ty(),
            Self::TupleStruct(info) => info.ty(),
            Self::Tuple(info) => info.ty(),
            Self::List(info) => info.ty(),
            Self::Array(info) => info.ty(),
            Self::Map(info) => info.ty(),
            Self::Set(info) => info.ty(),
            Self::Enum(info) => info.ty(),
            Self::Opaque(info) => info.ty(),
        }
    }

    crate::info::impl_type_fn!();

    /// Returns the `ReflectKind` for this `TypeInfo` (a fast discriminator).
    pub const fn kind(&self) -> ReflectKind {
        match self {
            Self::Struct(_) => ReflectKind::Struct,
            Self::TupleStruct(_) => ReflectKind::TupleStruct,
            Self::Tuple(_) => ReflectKind::Tuple,
            Self::List(_) => ReflectKind::List,
            Self::Array(_) => ReflectKind::Array,
            Self::Map(_) => ReflectKind::Map,
            Self::Set(_) => ReflectKind::Set,
            Self::Enum(_) => ReflectKind::Enum,
            Self::Opaque(_) => ReflectKind::Opaque,
        }
    }

    /// Returns the generics metadata (type/const parameters) for this type.
    ///
    /// Note: this is not inlined to avoid recursive inline expansion across
    /// `TypeInfo` variants.
    pub const fn generics(&self) -> &Generics {
        match self {
            Self::Struct(info) => info.generics(),
            Self::TupleStruct(info) => info.generics(),
            Self::Tuple(info) => info.generics(),
            Self::List(info) => info.generics(),
            Self::Array(info) => info.generics(),
            Self::Map(info) => info.generics(),
            Self::Set(info) => info.generics(),
            Self::Enum(info) => info.generics(),
            Self::Opaque(info) => info.generics(),
        }
    }

    /// Returns the custom attributes attached to this type, if any.
    ///
    /// For kinds that do not support custom attributes this returns a shared
    /// empty reference (`CustomAttributes::EMPTY`).
    pub fn custom_attributes(&self) -> &CustomAttributes {
        match self {
            Self::Struct(info) => info.custom_attributes(),
            Self::TupleStruct(info) => info.custom_attributes(),
            Self::Enum(info) => info.custom_attributes(),
            Self::Opaque(info) => info.custom_attributes(),
            _ => CustomAttributes::EMPTY,
        }
    }
    crate::info::attributes::impl_custom_attributes_fn!();

    /// Returns the documentation string for the type, if `reflect_docs` is
    /// enabled and docs are present.
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
            Self::TupleStruct(info) => info.docs(),
            Self::Tuple(info) => info.docs(),
            Self::List(info) => info.docs(),
            Self::Array(info) => info.docs(),
            Self::Map(info) => info.docs(),
            Self::Set(info) => info.docs(),
            Self::Enum(info) => info.docs(),
            Self::Opaque(info) => info.docs(),
        }
    }
}
