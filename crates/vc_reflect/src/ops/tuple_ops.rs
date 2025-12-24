use crate::{
    Reflect,
    impls::NonGenericTypeInfoCell,
    info::{OpaqueInfo, TupleInfo, TypeInfo, TypePath, Typed},
    ops::{ApplyError, ReflectCloneError},
    reflection::impl_reflect_cast_fn,
};
use alloc::{boxed::Box, vec::Vec};
use core::fmt;

/// Represents a [`Tuple`], used to dynamically modify data and its reflected type information.
///
/// Dynamic types are special in that their `TypeInfo` is [`OpaqueInfo`],
/// but other APIs behave like the represented type, such as [`reflect_kind`] and [`reflect_ref`].
///
/// [`reflect_kind`]: crate::Reflect::reflect_kind
/// [`reflect_ref`]: crate::Reflect::reflect_ref
pub struct DynamicTuple {
    tuple_info: Option<&'static TypeInfo>,
    fields: Vec<Box<dyn Reflect>>,
}

impl TypePath for DynamicTuple {
    #[inline]
    fn type_path() -> &'static str {
        "vc_reflect::ops::DynamicTuple"
    }

    #[inline]
    fn type_name() -> &'static str {
        "DynamicTuple"
    }

    #[inline]
    fn type_ident() -> &'static str {
        "DynamicTuple"
    }

    #[inline]
    fn module_path() -> Option<&'static str> {
        Some("vc_reflect::ops")
    }
}

impl Typed for DynamicTuple {
    fn type_info() -> &'static TypeInfo {
        static CELL: NonGenericTypeInfoCell = NonGenericTypeInfoCell::new();
        CELL.get_or_init(|| TypeInfo::Opaque(OpaqueInfo::new::<Self>()))
    }
}

impl DynamicTuple {
    /// Create a empty [`DynamicTuple`].
    #[inline]
    pub const fn new() -> Self {
        Self {
            tuple_info: None,
            fields: Vec::new(),
        }
    }

    /// See [`Vec::with_capacity`]
    #[inline]
    pub fn with_capacity(capacity: usize) -> Self {
        Self {
            tuple_info: None,
            fields: Vec::with_capacity(capacity),
        }
    }

    /// Sets the [`TypeInfo`] to be represented by this `DynamicTuple`.
    ///
    /// # Panic
    ///
    /// If the input is not list info or None.
    #[inline]
    pub fn set_type_info(&mut self, tuple_info: Option<&'static TypeInfo>) {
        match tuple_info {
            Some(TypeInfo::Tuple(_)) | None => {}
            _ => {
                panic!(
                    "Call `DynamicMap::set_type_info`, but the input is not tuple information or None."
                )
            }
        }

        self.tuple_info = tuple_info;
    }

    /// Appends an element with value `value` to the tuple.
    #[inline]
    pub fn insert(&mut self, value: Box<dyn Reflect>) {
        self.fields.push(value);
    }
}

impl Reflect for DynamicTuple {
    impl_reflect_cast_fn!(Tuple);

    #[inline]
    fn is_dynamic(&self) -> bool {
        true
    }

    #[inline]
    fn represented_type_info(&self) -> Option<&'static TypeInfo> {
        self.tuple_info
    }

    #[inline]
    fn to_dynamic(&self) -> Box<dyn Reflect> {
        Box::new(<Self as Tuple>::to_dynamic_tuple(self))
    }

    #[inline]
    fn reflect_clone(&self) -> Result<Box<dyn Reflect>, ReflectCloneError> {
        Ok(Box::new(<Self as Tuple>::to_dynamic_tuple(self)))
    }

    #[inline]
    fn try_apply(&mut self, value: &dyn Reflect) -> Result<(), ApplyError> {
        crate::impls::tuple_try_apply(self, value)
    }

    #[inline]
    fn reflect_hash(&self) -> Option<u64> {
        crate::impls::tuple_hash(self)
    }

    #[inline]
    fn reflect_partial_eq(&self, other: &dyn Reflect) -> Option<bool> {
        crate::impls::tuple_partial_eq(self, other)
    }

    #[inline]
    fn reflect_debug(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "DynamicTuple(")?;
        crate::impls::tuple_debug(self, f)?;
        write!(f, ")")
    }
}

impl fmt::Debug for DynamicTuple {
    #[inline]
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        self.reflect_debug(f)
    }
}

impl FromIterator<Box<dyn Reflect>> for DynamicTuple {
    fn from_iter<I: IntoIterator<Item = Box<dyn Reflect>>>(fields: I) -> Self {
        Self {
            tuple_info: None,
            fields: fields.into_iter().collect(),
        }
    }
}

impl IntoIterator for DynamicTuple {
    type Item = Box<dyn Reflect>;
    type IntoIter = alloc::vec::IntoIter<Self::Item>;

    fn into_iter(self) -> Self::IntoIter {
        self.fields.into_iter()
    }
}

impl<'a> IntoIterator for &'a DynamicTuple {
    type Item = &'a dyn Reflect;
    type IntoIter = TupleFieldIter<'a>;

    #[inline]
    fn into_iter(self) -> Self::IntoIter {
        self.iter_fields()
    }
}

/// A trait used to power [tuple-like] operations via [reflection].
///
/// This trait uses the [`Reflect`] trait to allow implementors to have their fields
/// be dynamically addressed by index.
///
/// This trait is automatically implemented for arbitrary tuples of up to **12**
/// elements, provided that each element implements [`Reflect`].
///
/// # Example
///
/// ```
/// use vc_reflect::{Reflect, ops::Tuple};
///
/// let foo = (123_u32, true);
/// assert_eq!(foo.field_len(), 2);
///
/// let field: &dyn Reflect = foo.field(0).unwrap();
/// assert_eq!(field.downcast_ref::<u32>(), Some(&123));
/// ```
///
/// [tuple-like]: https://doc.rust-lang.org/book/ch03-02-data-types.html#the-tuple-type
/// [reflection]: crate
pub trait Tuple: Reflect {
    /// Returns a reference to the value of the field with index `index` as a
    /// `&dyn Reflect`.
    fn field(&self, index: usize) -> Option<&dyn Reflect>;

    /// Returns a mutable reference to the value of the field with index `index`
    /// as a `&mut dyn Reflect`.
    fn field_mut(&mut self, index: usize) -> Option<&mut dyn Reflect>;

    /// Returns the number of fields in the tuple.
    fn field_len(&self) -> usize;

    /// Returns an iterator over the values of the tuple's fields.
    fn iter_fields(&self) -> TupleFieldIter<'_>;

    /// Drain the fields of this tuple to get a vector of owned values.
    fn drain(self: Box<Self>) -> Vec<Box<dyn Reflect>>;

    /// Creates a new [`DynamicTuple`] from this tuple.
    fn to_dynamic_tuple(&self) -> DynamicTuple {
        DynamicTuple {
            tuple_info: self.represented_type_info(),
            fields: self.iter_fields().map(Reflect::to_dynamic).collect(),
        }
    }

    /// Get actual [`TupleInfo`] of underlying types.
    ///
    /// If it is a dynamic type, it will return `None`.
    ///
    /// If it is not a dynamic type and the returned value is not `None` or `TupleInfo`, it will panic.
    /// (If you want to implement dynamic types yourself, please return None.)
    #[inline]
    fn reflect_tuple_info(&self) -> Option<&'static TupleInfo> {
        self.reflect_type_info().as_tuple().ok()
    }

    /// Get the [`TupleInfo`] of representation.
    ///
    /// Normal types return their own information,
    /// while dynamic types return `None`` if they do not represent an object
    #[inline]
    fn represented_tuple_info(&self) -> Option<&'static TupleInfo> {
        self.represented_type_info()?.as_tuple().ok()
    }
}

impl Tuple for DynamicTuple {
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
    fn iter_fields(&self) -> TupleFieldIter<'_> {
        TupleFieldIter::new(self)
    }

    #[inline]
    fn drain(self: Box<Self>) -> Vec<Box<dyn Reflect>> {
        self.fields
    }

    #[inline]
    fn reflect_tuple_info(&self) -> Option<&'static TupleInfo> {
        None
    }

    #[inline]
    fn represented_tuple_info(&self) -> Option<&'static TupleInfo> {
        self.tuple_info?.as_tuple().ok()
    }
}

/// An iterator over the field values of a tuple.
pub struct TupleFieldIter<'a> {
    tuple: &'a dyn Tuple,
    index: usize,
}

impl<'a> TupleFieldIter<'a> {
    #[inline(always)]
    pub fn new(value: &'a dyn Tuple) -> Self {
        TupleFieldIter {
            tuple: value,
            index: 0,
        }
    }
}

impl<'a> Iterator for TupleFieldIter<'a> {
    type Item = &'a dyn Reflect;

    #[inline]
    fn next(&mut self) -> Option<Self::Item> {
        let value = self.tuple.field(self.index);
        self.index += value.is_some() as usize;
        value
    }

    #[inline]
    fn size_hint(&self) -> (usize, Option<usize>) {
        let size = self.tuple.field_len();
        (size - self.index, Some(size))
    }
}

impl<'a> ExactSizeIterator for TupleFieldIter<'a> {}
