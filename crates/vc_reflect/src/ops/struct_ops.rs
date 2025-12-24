use crate::{
    Reflect,
    impls::NonGenericTypeInfoCell,
    info::{OpaqueInfo, StructInfo, TypeInfo, TypePath, Typed},
    ops::{ApplyError, ReflectCloneError},
    reflection::impl_reflect_cast_fn,
};
use alloc::{
    borrow::{Cow, ToOwned},
    boxed::Box,
    vec::Vec,
};
use core::fmt;
use vc_utils::hash::HashMap;

/// Represents a [`Struct`], used to dynamically modify data and its reflected type information.
///
/// Dynamic types are special in that their `TypeInfo` is [`OpaqueInfo`],
/// but other APIs behave like the represented type, such as [`reflect_kind`] and [`reflect_ref`].
///
/// [`reflect_kind`]: crate::Reflect::reflect_kind
/// [`reflect_ref`]: crate::Reflect::reflect_ref
pub struct DynamicStruct {
    struct_info: Option<&'static TypeInfo>,
    fields: Vec<Box<dyn Reflect>>,
    field_names: Vec<Cow<'static, str>>,
    field_indices: HashMap<Cow<'static, str>, usize>,
}

impl TypePath for DynamicStruct {
    #[inline]
    fn type_path() -> &'static str {
        "vc_reflect::ops::DynamicStruct"
    }

    #[inline]
    fn type_name() -> &'static str {
        "DynamicStruct"
    }

    #[inline]
    fn type_ident() -> &'static str {
        "DynamicStruct"
    }

    #[inline]
    fn module_path() -> Option<&'static str> {
        Some("vc_reflect::ops")
    }
}

impl Typed for DynamicStruct {
    fn type_info() -> &'static TypeInfo {
        static CELL: NonGenericTypeInfoCell = NonGenericTypeInfoCell::new();
        CELL.get_or_init(|| TypeInfo::Opaque(OpaqueInfo::new::<Self>()))
    }
}

impl DynamicStruct {
    /// Create a empty [`DynamicStruct`].
    #[inline]
    pub const fn new() -> Self {
        Self {
            struct_info: None,
            fields: Vec::new(),
            field_names: Vec::new(),
            field_indices: HashMap::<_, _>::new(),
        }
    }

    /// See [`Vec::with_capacity`]
    #[inline]
    pub fn with_capacity(capacity: usize) -> Self {
        Self {
            struct_info: None,
            fields: Vec::with_capacity(capacity),
            field_names: Vec::with_capacity(capacity),
            field_indices: HashMap::<_, _>::with_capacity(capacity),
        }
    }

    /// Sets the [`StructInfo`] to be represented by this `DynamicStruct`.
    #[inline]
    pub fn set_type_info(&mut self, struct_info: Option<&'static TypeInfo>) {
        match struct_info {
            Some(TypeInfo::Struct(_)) | None => {}
            _ => {
                panic!(
                    "Call `DynamicStruct::set_type_info`, but the input is not struct information or None."
                )
            }
        }

        self.struct_info = struct_info;
    }

    /// Inserts a field named `name` with value `value` into the struct.
    ///
    /// If the field already exists, it is overwritten.
    pub fn insert(&mut self, name: impl Into<Cow<'static, str>>, value: Box<dyn Reflect>) {
        let name: Cow<'static, str> = name.into();
        if let Some(index) = self.field_indices.get(&name) {
            self.fields[*index] = value;
        } else {
            self.fields.push(value);
            self.field_indices
                .insert(name.clone(), self.fields.len() - 1);
            self.field_names.push(name);
        }
    }

    /// Gets the index of the field with the given name.
    #[inline]
    pub fn index_of(&self, name: &str) -> Option<usize> {
        self.field_indices.get(name).copied()
    }
}

impl Reflect for DynamicStruct {
    impl_reflect_cast_fn!(Struct);

    #[inline]
    fn is_dynamic(&self) -> bool {
        true
    }

    #[inline]
    fn represented_type_info(&self) -> Option<&'static TypeInfo> {
        self.struct_info
    }

    #[inline]
    fn to_dynamic(&self) -> Box<dyn Reflect> {
        Box::new(<Self as Struct>::to_dynamic_struct(self))
    }

    #[inline]
    fn reflect_clone(&self) -> Result<Box<dyn Reflect>, ReflectCloneError> {
        Ok(Box::new(<Self as Struct>::to_dynamic_struct(self)))
    }

    #[inline]
    fn try_apply(&mut self, value: &dyn Reflect) -> Result<(), ApplyError> {
        crate::impls::struct_try_apply(self, value)
    }

    #[inline]
    fn reflect_hash(&self) -> Option<u64> {
        crate::impls::struct_hash(self)
    }

    #[inline]
    fn reflect_partial_eq(&self, other: &dyn Reflect) -> Option<bool> {
        crate::impls::struct_partial_eq(self, other)
    }

    #[inline]
    fn reflect_debug(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "DynamicStruct(")?;
        crate::impls::struct_debug(self, f)?;
        write!(f, ")")
    }
}

impl fmt::Debug for DynamicStruct {
    #[inline]
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        self.reflect_debug(f)
    }
}

impl<'a, N: Into<Cow<'static, str>>> FromIterator<(N, Box<dyn Reflect>)> for DynamicStruct {
    fn from_iter<T: IntoIterator<Item = (N, Box<dyn Reflect>)>>(fields: T) -> Self {
        let mut dynamic_struct = DynamicStruct::new();
        for (name, value) in fields.into_iter() {
            dynamic_struct.insert(name, value);
        }
        dynamic_struct
    }
}

impl IntoIterator for DynamicStruct {
    type Item = Box<dyn Reflect>;
    type IntoIter = alloc::vec::IntoIter<Self::Item>;

    fn into_iter(self) -> Self::IntoIter {
        self.fields.into_iter()
    }
}

impl<'a> IntoIterator for &'a DynamicStruct {
    type Item = &'a dyn Reflect;
    type IntoIter = StructFieldIter<'a>;

    #[inline]
    fn into_iter(self) -> Self::IntoIter {
        self.iter_fields()
    }
}

/// A trait used to power [struct-like] operations via [reflection].
///
/// This trait uses the [`Reflect`] trait to allow implementors to have their fields
/// be dynamically addressed by both name and index.
///
/// When using [`#[derive(Reflect)]`](crate::derive::Reflect) on a standard struct,
/// this trait will be automatically implemented.
///
/// # Note
///
/// This includes `struct T{}`, but not `struct T;`.
/// The latter will be considered as [`Opaque`](crate::info::OpaqueInfo) type
/// and can be optimized extensively.
///
/// # Example
///
/// ```
/// use vc_reflect::{derive::Reflect, Reflect, ops::Struct};
///
/// #[derive(Reflect)]
/// struct Foo {
///     bar: u32,
/// }
///
/// let foo = Foo { bar: 123 };
///
/// let p: &dyn Struct = &foo;
///
/// assert_eq!(foo.field_len(), 1);
/// assert_eq!(foo.name_at(0), Some("bar"));
///
/// let field: &dyn Reflect = foo.field("bar").unwrap();
/// assert_eq!(field.downcast_ref::<u32>(), Some(&123));
/// ```
///
/// [struct-like]: https://doc.rust-lang.org/book/ch05-01-defining-structs.html
/// [reflection]: crate
/// [unit structs]: https://doc.rust-lang.org/book/ch05-01-defining-structs.html#unit-like-structs-without-any-fields
pub trait Struct: Reflect {
    /// Returns a reference to the value of the field named `name` as a `&dyn
    /// PartialReflect`.
    fn field(&self, name: &str) -> Option<&dyn Reflect>;

    /// Returns a mutable reference to the value of the field named `name` as a
    /// `&mut dyn PartialReflect`.
    fn field_mut(&mut self, name: &str) -> Option<&mut dyn Reflect>;

    /// Returns a reference to the value of the field with index `index` as a
    /// `&dyn PartialReflect`.
    fn field_at(&self, index: usize) -> Option<&dyn Reflect>;

    /// Returns a mutable reference to the value of the field with index `index`
    /// as a `&mut dyn PartialReflect`.
    fn field_at_mut(&mut self, index: usize) -> Option<&mut dyn Reflect>;

    /// Returns the name of the field with index `index`.
    fn name_at(&self, index: usize) -> Option<&str>;

    /// Returns the number of fields in the struct.
    fn field_len(&self) -> usize;

    /// Returns an iterator over the values of the reflectable fields for this struct.
    fn iter_fields(&self) -> StructFieldIter<'_>;

    /// Creates a new [`DynamicStruct`] from this struct.
    fn to_dynamic_struct(&self) -> DynamicStruct {
        let mut dynamic_struct = DynamicStruct::with_capacity(self.field_len());
        dynamic_struct.set_type_info(self.represented_type_info());
        for (i, val) in self.iter_fields().enumerate() {
            dynamic_struct.insert(self.name_at(i).unwrap().to_owned(), val.to_dynamic());
        }
        dynamic_struct
    }

    /// Get actual [`StructInfo`] of underlying types.
    ///
    /// If it is a dynamic type, it will return `None`.
    ///
    /// If it is not a dynamic type and the returned value is not `None` or `StructInfo`, it will panic.
    /// (If you want to implement dynamic types yourself, please return None.)
    #[inline]
    fn reflect_struct_info(&self) -> Option<&'static StructInfo> {
        self.reflect_type_info().as_struct().ok()
    }

    /// Get the [`StructInfo`] of representation.
    ///
    /// Normal types return their own information,
    /// while dynamic types return `None`` if they do not represent an object
    #[inline]
    fn represented_struct_info(&self) -> Option<&'static StructInfo> {
        self.represented_type_info()?.as_struct().ok()
    }
}

/// An iterator over the field values of a struct.
pub struct StructFieldIter<'a> {
    struct_val: &'a dyn Struct,
    index: usize,
}

impl<'a> StructFieldIter<'a> {
    #[inline(always)]
    pub fn new(value: &'a dyn Struct) -> Self {
        StructFieldIter {
            struct_val: value,
            index: 0,
        }
    }
}

impl<'a> Iterator for StructFieldIter<'a> {
    type Item = &'a dyn Reflect;

    #[inline]
    fn next(&mut self) -> Option<Self::Item> {
        let value = self.struct_val.field_at(self.index);
        self.index += value.is_some() as usize;
        value
    }

    #[inline]
    fn size_hint(&self) -> (usize, Option<usize>) {
        let size = self.struct_val.field_len();
        (size - self.index, Some(size))
    }
}

impl<'a> ExactSizeIterator for StructFieldIter<'a> {}

impl Struct for DynamicStruct {
    #[inline]
    fn field(&self, name: &str) -> Option<&dyn Reflect> {
        self.field_indices
            .get(name)
            .map(|index| &*self.fields[*index])
    }

    #[inline]
    fn field_mut(&mut self, name: &str) -> Option<&mut dyn Reflect> {
        self.field_indices
            .get(name)
            .map(|index| &mut *self.fields[*index])
    }

    #[inline]
    fn field_at(&self, index: usize) -> Option<&dyn Reflect> {
        self.fields.get(index).map(|value| &**value)
    }

    #[inline]
    fn field_at_mut(&mut self, index: usize) -> Option<&mut dyn Reflect> {
        self.fields.get_mut(index).map(|value| &mut **value)
    }

    #[inline]
    fn name_at(&self, index: usize) -> Option<&str> {
        self.field_names.get(index).map(AsRef::as_ref)
    }

    #[inline]
    fn field_len(&self) -> usize {
        self.fields.len()
    }

    #[inline]
    fn iter_fields(&self) -> StructFieldIter<'_> {
        StructFieldIter::new(self)
    }

    fn to_dynamic_struct(&self) -> DynamicStruct {
        DynamicStruct {
            struct_info: self.represented_type_info(),
            fields: self.fields.iter().map(|val| val.to_dynamic()).collect(),
            field_names: self.field_names.clone(),
            field_indices: self.field_indices.clone(),
        }
    }
}
