use alloc::{boxed::Box, vec::Vec};
use core::fmt;

use crate::{
    Reflect,
    impls::NonGenericTypeInfoCell,
    info::{OpaqueInfo, TupleStructInfo, TypeInfo, TypePath, Typed},
    ops::{ApplyError, DynamicTuple, ReflectCloneError, Tuple},
    reflection::impl_reflect_cast_fn,
};

impl From<DynamicTuple> for DynamicTupleStruct {
    fn from(value: DynamicTuple) -> Self {
        Self {
            tuple_struct_info: None,
            fields: Tuple::drain(Box::new(value)),
        }
    }
}

/// Represents a [`TupleStruct`], used to dynamically modify data and its reflected type information.
///
/// Dynamic types are special in that their `TypeInfo` is [`OpaqueInfo`],
/// but other APIs behave like the represented type, such as [`reflect_kind`] and [`reflect_ref`].
///
/// [`reflect_kind`]: crate::Reflect::reflect_kind
/// [`reflect_ref`]: crate::Reflect::reflect_ref
pub struct DynamicTupleStruct {
    tuple_struct_info: Option<&'static TypeInfo>,
    fields: Vec<Box<dyn Reflect>>,
}

impl TypePath for DynamicTupleStruct {
    #[inline]
    fn type_path() -> &'static str {
        "vc_reflect::ops::DynamicTupleStruct"
    }

    #[inline]
    fn type_name() -> &'static str {
        "DynamicTupleStruct"
    }

    #[inline]
    fn type_ident() -> &'static str {
        "DynamicTupleStruct"
    }

    #[inline]
    fn module_path() -> Option<&'static str> {
        Some("vc_reflect::ops")
    }
}

impl Typed for DynamicTupleStruct {
    fn type_info() -> &'static TypeInfo {
        static CELL: NonGenericTypeInfoCell = NonGenericTypeInfoCell::new();
        CELL.get_or_init(|| TypeInfo::Opaque(OpaqueInfo::new::<Self>()))
    }
}

impl DynamicTupleStruct {
    /// Create a empty [`DynamicTupleStruct`].
    #[inline]
    pub const fn new() -> Self {
        Self {
            tuple_struct_info: None,
            fields: Vec::new(),
        }
    }

    /// See [`Vec::with_capacity`]
    #[inline]
    pub fn with_capacity(capacity: usize) -> Self {
        Self {
            tuple_struct_info: None,
            fields: Vec::with_capacity(capacity),
        }
    }

    /// Sets the [`TypeInfo`] to be represented by this `DynamicTupleStruct`.
    #[inline]
    pub fn set_type_info(&mut self, tuple_struct_info: Option<&'static TypeInfo>) {
        match tuple_struct_info {
            Some(TypeInfo::TupleStruct(_)) | None => {}
            _ => {
                panic!(
                    "Call `DynamicTupleStruct::set_type_info`, but the input is not tuple-struct information or None."
                )
            }
        }

        self.tuple_struct_info = tuple_struct_info;
    }

    /// Appends an element with value `value` to the tuple struct.
    #[inline]
    pub fn insert_boxed(&mut self, value: Box<dyn Reflect>) {
        self.fields.push(value);
    }

    /// Appends a typed element with value `value` to the tuple struct.
    #[inline]
    pub fn insert<T: Reflect>(&mut self, value: T) {
        self.fields.push(Box::new(value));
    }
}

impl Reflect for DynamicTupleStruct {
    impl_reflect_cast_fn!(TupleStruct);

    #[inline]
    fn is_dynamic(&self) -> bool {
        true
    }

    #[inline]
    fn represented_type_info(&self) -> Option<&'static TypeInfo> {
        self.tuple_struct_info
    }

    #[inline]
    fn to_dynamic(&self) -> Box<dyn Reflect> {
        Box::new(<Self as TupleStruct>::to_dynamic_tuple_struct(self))
    }

    #[inline]
    fn reflect_clone(&self) -> Result<Box<dyn Reflect>, ReflectCloneError> {
        Ok(Box::new(<Self as TupleStruct>::to_dynamic_tuple_struct(
            self,
        )))
    }

    #[inline]
    fn try_apply(&mut self, value: &dyn Reflect) -> Result<(), ApplyError> {
        crate::impls::tuple_struct_try_apply(self, value)
    }

    #[inline]
    fn reflect_partial_eq(&self, other: &dyn Reflect) -> Option<bool> {
        crate::impls::tuple_struct_partial_eq(self, other)
    }

    #[inline]
    fn reflect_hash(&self) -> Option<u64> {
        crate::impls::tuple_struct_hash(self)
    }

    #[inline]
    fn reflect_debug(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "DynamicTupleStruct(")?;
        crate::impls::tuple_struct_debug(self, f)?;
        write!(f, ")")
    }
}

impl fmt::Debug for DynamicTupleStruct {
    #[inline]
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        self.reflect_debug(f)
    }
}

impl FromIterator<Box<dyn Reflect>> for DynamicTupleStruct {
    fn from_iter<T: IntoIterator<Item = Box<dyn Reflect>>>(iter: T) -> Self {
        Self {
            tuple_struct_info: None,
            fields: iter.into_iter().collect(),
        }
    }
}

impl IntoIterator for DynamicTupleStruct {
    type Item = Box<dyn Reflect>;
    type IntoIter = alloc::vec::IntoIter<Self::Item>;

    fn into_iter(self) -> Self::IntoIter {
        self.fields.into_iter()
    }
}

impl<'a> IntoIterator for &'a DynamicTupleStruct {
    type Item = &'a dyn Reflect;
    type IntoIter = TupleStructFieldIter<'a>;

    #[inline]
    fn into_iter(self) -> Self::IntoIter {
        self.iter_fields()
    }
}

/// A trait used to power [tuple struct-like] operations via [reflection].
///
/// This trait uses the [`Reflect`] trait to allow implementors to have their fields
/// be dynamically addressed by index.
///
/// When using [`#[derive(Reflect)]`](crate::derive::Reflect) on a tuple struct,
/// this trait will be automatically implemented.
///
/// # Note
///
/// This includes `struct T()`, but not `struct T;`.
/// The latter will be considered as [`Opaque`](crate::info::OpaqueInfo) type
/// and can be optimized extensively.
///
/// # Example
///
/// ```
/// use vc_reflect::{derive::Reflect, Reflect, ops::TupleStruct};
///
/// #[derive(Reflect)]
/// struct Foo(u32);
///
/// let foo = Foo(123);
///
/// assert_eq!(foo.field_len(), 1);
///
/// let field: &dyn Reflect = foo.field(0).unwrap();
/// assert_eq!(field.downcast_ref::<u32>(), Some(&123));
/// ```
///
/// [tuple struct-like]: https://doc.rust-lang.org/book/ch05-01-defining-structs.html#using-tuple-structs-without-named-fields-to-create-different-types
/// [reflection]: crate
pub trait TupleStruct: Reflect {
    /// Returns a reference to the value of the field with index `index` as a
    /// `&dyn Reflect`.
    fn field(&self, index: usize) -> Option<&dyn Reflect>;

    /// Returns a mutable reference to the value of the field with index `index`
    /// as a `&mut dyn Reflect`.
    fn field_mut(&mut self, index: usize) -> Option<&mut dyn Reflect>;

    /// Returns the number of fields in the tuple struct.
    fn field_len(&self) -> usize;

    /// Returns an iterator over the values of the tuple struct's fields.
    fn iter_fields(&self) -> TupleStructFieldIter<'_>;

    /// Creates a new [`DynamicTupleStruct`] from this tuple struct.
    fn to_dynamic_tuple_struct(&self) -> DynamicTupleStruct {
        DynamicTupleStruct {
            tuple_struct_info: self.represented_type_info(),
            fields: self.iter_fields().map(Reflect::to_dynamic).collect(),
        }
    }

    /// Get actual [`TupleStructInfo`] of underlying types.
    ///
    /// If it is a dynamic type, it will return `None`.
    ///
    /// If it is not a dynamic type and the returned value is not `None` or `TupleStructInfo`, it will panic.
    /// (If you want to implement dynamic types yourself, please return None.)
    #[inline]
    fn reflect_tuple_struct_info(&self) -> Option<&'static TupleStructInfo> {
        self.reflect_type_info().as_tuple_struct().ok()
    }

    /// Get the [`TupleStructInfo`] of representation.
    ///
    /// Normal types return their own information,
    /// while dynamic types return `None`` if they do not represent an object
    #[inline]
    fn represented_tuple_struct_info(&self) -> Option<&'static TupleStructInfo> {
        self.represented_type_info()?.as_tuple_struct().ok()
    }
}

/// An iterator over the field values of a tuple struct.
pub struct TupleStructFieldIter<'a> {
    tuple_struct: &'a dyn TupleStruct,
    index: usize,
}

impl<'a> TupleStructFieldIter<'a> {
    #[inline(always)]
    pub fn new(value: &'a dyn TupleStruct) -> Self {
        TupleStructFieldIter {
            tuple_struct: value,
            index: 0,
        }
    }
}

impl<'a> Iterator for TupleStructFieldIter<'a> {
    type Item = &'a dyn Reflect;

    #[inline]
    fn next(&mut self) -> Option<Self::Item> {
        let value = self.tuple_struct.field(self.index);
        self.index += value.is_some() as usize;
        value
    }

    #[inline]
    fn size_hint(&self) -> (usize, Option<usize>) {
        let size = self.tuple_struct.field_len();
        (size - self.index, Some(size))
    }
}

impl<'a> ExactSizeIterator for TupleStructFieldIter<'a> {}

impl TupleStruct for DynamicTupleStruct {
    #[inline]
    fn field(&self, index: usize) -> Option<&dyn Reflect> {
        self.fields.get(index).map(|field| &**field)
    }

    #[inline]
    fn field_mut(&mut self, index: usize) -> Option<&mut dyn Reflect> {
        self.fields.get_mut(index).map(|field| &mut **field)
    }

    #[inline]
    fn field_len(&self) -> usize {
        self.fields.len()
    }

    #[inline]
    fn iter_fields(&self) -> TupleStructFieldIter<'_> {
        TupleStructFieldIter::new(self)
    }
}
