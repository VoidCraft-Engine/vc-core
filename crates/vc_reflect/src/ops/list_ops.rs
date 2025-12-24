use crate::{
    Reflect,
    impls::NonGenericTypeInfoCell,
    info::{ListInfo, OpaqueInfo, TypeInfo, TypePath, Typed},
    ops::{ApplyError, ReflectCloneError},
    reflection::impl_reflect_cast_fn,
};
use alloc::{boxed::Box, vec::Vec};
use core::fmt;

/// Represents a [`List`], used to dynamically modify data and its reflected type information.
///
/// Dynamic types are special in that their `TypeInfo` is [`OpaqueInfo`],
/// but other APIs behave like the represented type, such as [`reflect_kind`] and [`reflect_ref`].
///
/// [`reflect_kind`]: crate::Reflect::reflect_kind
/// [`reflect_ref`]: crate::Reflect::reflect_ref
pub struct DynamicList {
    list_info: Option<&'static TypeInfo>,
    values: Vec<Box<dyn Reflect>>,
}

impl TypePath for DynamicList {
    #[inline]
    fn type_path() -> &'static str {
        "vc_reflect::ops::DynamicList"
    }

    #[inline]
    fn type_name() -> &'static str {
        "DynamicList"
    }

    #[inline]
    fn type_ident() -> &'static str {
        "DynamicList"
    }

    #[inline]
    fn module_path() -> Option<&'static str> {
        Some("vc_reflect::ops")
    }
}

impl Typed for DynamicList {
    fn type_info() -> &'static TypeInfo {
        static CELL: NonGenericTypeInfoCell = NonGenericTypeInfoCell::new();
        CELL.get_or_init(|| TypeInfo::Opaque(OpaqueInfo::new::<Self>()))
    }
}

impl DynamicList {
    /// Create a empty [`DynamicList`].
    #[inline]
    pub const fn new() -> Self {
        Self {
            list_info: None,
            values: Vec::new(),
        }
    }

    /// See [`Vec::with_capacity`]
    #[inline]
    pub fn with_capacity(capcity: usize) -> Self {
        Self {
            list_info: None,
            values: Vec::with_capacity(capcity),
        }
    }

    /// Sets the [`TypeInfo`] to be represented by this `DynamicList`.
    ///
    /// # Panic
    ///
    /// If the input is not list info or None.
    #[inline]
    pub fn set_type_info(&mut self, list_info: Option<&'static TypeInfo>) {
        match list_info {
            Some(TypeInfo::List(_)) | None => {}
            _ => {
                panic!(
                    "Call `DynamicList::set_type_info`, but the input is not list information or None."
                )
            }
        }

        self.list_info = list_info;
    }
}

impl Reflect for DynamicList {
    impl_reflect_cast_fn!(List);

    #[inline]
    fn is_dynamic(&self) -> bool {
        true
    }

    #[inline]
    fn represented_type_info(&self) -> Option<&'static TypeInfo> {
        self.list_info
    }

    #[inline]
    fn to_dynamic(&self) -> Box<dyn Reflect> {
        Box::new(<Self as List>::to_dynamic_list(self))
    }

    #[inline]
    fn reflect_clone(&self) -> Result<Box<dyn Reflect>, ReflectCloneError> {
        Ok(Box::new(<Self as List>::to_dynamic_list(self)))
    }

    #[inline]
    fn try_apply(&mut self, value: &dyn Reflect) -> Result<(), ApplyError> {
        crate::impls::list_try_apply(self, value)
    }

    #[inline]
    fn reflect_hash(&self) -> Option<u64> {
        crate::impls::list_hash(self)
    }

    #[inline]
    fn reflect_partial_eq(&self, other: &dyn Reflect) -> Option<bool> {
        crate::impls::list_partial_eq(self, other)
    }

    #[inline]
    fn reflect_debug(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "DynamicList(")?;
        crate::impls::list_debug(self, f)?;
        write!(f, ")")
    }
}

impl fmt::Debug for DynamicList {
    #[inline]
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        self.reflect_debug(f)
    }
}

impl<T: Reflect> FromIterator<T> for DynamicList {
    fn from_iter<I: IntoIterator<Item = T>>(values: I) -> Self {
        Self {
            list_info: None,
            values: values
                .into_iter()
                .map(|field| Box::new(field) as Box<dyn Reflect>)
                .collect(),
        }
    }
}

impl FromIterator<Box<dyn Reflect>> for DynamicList {
    fn from_iter<I: IntoIterator<Item = Box<dyn Reflect>>>(values: I) -> Self {
        Self {
            list_info: None,
            values: values.into_iter().collect(),
        }
    }
}

impl IntoIterator for DynamicList {
    type Item = Box<dyn Reflect>;
    type IntoIter = alloc::vec::IntoIter<Self::Item>;

    fn into_iter(self) -> Self::IntoIter {
        self.values.into_iter()
    }
}

impl<'a> IntoIterator for &'a DynamicList {
    type Item = &'a dyn Reflect;
    type IntoIter = ListItemIter<'a>;

    #[inline]
    fn into_iter(self) -> Self::IntoIter {
        self.iter()
    }
}

/// A trait used to power [list-like] operations via [reflection].
///
/// This corresponds to types, like [`Vec`], which contain an ordered sequence
/// of elements that implement [`Reflect`].
///
/// Unlike the [`Array`](crate::ops::Array) trait, implementors of this trait are not expected to
/// maintain a constant length.
/// Methods like [insertion](List::insert) and [removal](List::remove) explicitly allow for their
/// internal size to change.
///
/// [`push`](List::push) and [`pop`](List::pop) have default implementations,
/// however it will generally be more performant to implement them manually
/// as the default implementation uses a very naive approach to find the correct position.
///
/// This trait expects its elements to be ordered linearly from front to back.
/// The _front_ element starts at index 0 with the _back_ element ending at the largest index.
/// This contract above should be upheld by any manual implementors.
///
/// Due to the [type-erasing] nature of the reflection API as a whole,
/// this trait does not make any guarantees that the implementor's elements
/// are homogeneous (i.e. all the same type).
///
/// # Example
///
/// ```
/// use vc_reflect::{Reflect, ops::List};
///
/// let foo: &mut dyn List = &mut vec![123_u32, 456_u32, 789_u32];
/// assert_eq!(foo.len(), 3);
///
/// let last_field: Box<dyn Reflect> = foo.pop().unwrap();
/// assert_eq!(last_field.downcast_ref::<u32>(), Some(&789));
/// ```
///
/// [list-like]: https://doc.rust-lang.org/book/ch08-01-vectors.html
/// [reflection]: crate
/// [type-erasing]: https://doc.rust-lang.org/book/ch17-02-trait-objects.html
pub trait List: Reflect {
    /// Returns a reference to the element at `index`, or `None` if out of bounds.
    fn get(&self, index: usize) -> Option<&dyn Reflect>;

    /// Returns a mutable reference to the element at `index`, or `None` if out of bounds.
    fn get_mut(&mut self, index: usize) -> Option<&mut dyn Reflect>;

    /// Inserts an element at position `index` within the list,
    /// shifting all elements after it towards the back of the list.
    ///
    /// # Panics
    /// - Panics if `index > len`.
    /// - Panics if input type incompatible, for non-dynamic types.
    fn insert(&mut self, index: usize, element: Box<dyn Reflect>);

    /// Try appends an element to the _back_ of the list.
    ///
    /// Return Err if `index > len` or value type incompatible.
    fn try_insert(&mut self, index: usize, value: Box<dyn Reflect>)
    -> Result<(), Box<dyn Reflect>>;

    /// Removes and returns the element at position `index` within the list,
    /// shifting all elements before it towards the front of the list.
    ///
    /// # Panics
    /// Panics if `index` is out of bounds.
    fn remove(&mut self, index: usize) -> Box<dyn Reflect>;

    /// Appends an element to the _back_ of the list.
    ///
    /// # Panics
    /// - Panics if input type incompatible, for non-dynamic types.
    fn push(&mut self, value: Box<dyn Reflect>);

    /// Try appends an element to the _back_ of the list.
    ///
    /// Return Err if value type incompatible.
    fn try_push(&mut self, value: Box<dyn Reflect>) -> Result<(), Box<dyn Reflect>>;

    /// Removes the _back_ element from the list and returns it, or [`None`] if it is empty.
    fn pop(&mut self) -> Option<Box<dyn Reflect>>;

    /// Returns the number of elements in the list.
    fn len(&self) -> usize;

    /// Returns `true` if the collection contains no elements.
    #[inline]
    fn is_empty(&self) -> bool {
        self.len() == 0
    }

    /// Returns an iterator over the list.
    fn iter(&self) -> ListItemIter<'_>;

    /// Drain the elements of this list to get a vector of owned values.
    ///
    /// After calling this function, `self` will be empty. The order of items in the returned
    /// [`Vec`] will match the order of items in `self`.
    fn drain(&mut self) -> Vec<Box<dyn Reflect>>;

    /// Creates a new [`DynamicList`] from this list.
    ///
    /// This function will replace all content with dynamic types, except for `Opaque`.
    fn to_dynamic_list(&self) -> DynamicList {
        DynamicList {
            list_info: self.represented_type_info(),
            values: self.iter().map(Reflect::to_dynamic).collect(),
        }
    }

    /// Get actual [`ListInfo`] of underlying types.
    ///
    /// If it is a dynamic type, it will return `None`.
    ///
    /// If it is not a dynamic type and the returned value is not `None` or `ListInfo`, it will panic.
    /// (If you want to implement dynamic types yourself, please return None.)
    #[inline]
    fn reflect_list_info(&self) -> Option<&'static ListInfo> {
        self.reflect_type_info().as_list().ok()
    }

    /// Get the [`ListInfo`] of representation.
    ///
    /// Normal types return their own information,
    /// while dynamic types return `None`` if they do not represent an object
    #[inline]
    fn represented_list_info(&self) -> Option<&'static ListInfo> {
        self.represented_type_info()?.as_list().ok()
    }
}

pub struct ListItemIter<'a> {
    list: &'a dyn List,
    index: usize,
}

impl ListItemIter<'_> {
    #[inline(always)]
    pub fn new(list: &dyn List) -> ListItemIter<'_> {
        ListItemIter { list, index: 0 }
    }
}

impl<'a> Iterator for ListItemIter<'a> {
    type Item = &'a dyn Reflect;

    #[inline]
    fn next(&mut self) -> Option<Self::Item> {
        let value = self.list.get(self.index);
        self.index += value.is_some() as usize;
        value
    }

    #[inline]
    fn size_hint(&self) -> (usize, Option<usize>) {
        let size = self.list.len();
        (size - self.index, Some(size))
    }
}

impl ExactSizeIterator for ListItemIter<'_> {}

impl List for DynamicList {
    #[inline]
    fn get(&self, index: usize) -> Option<&dyn Reflect> {
        self.values.get(index).map(|value| &**value)
    }

    #[inline]
    fn get_mut(&mut self, index: usize) -> Option<&mut dyn Reflect> {
        self.values.get_mut(index).map(|value| &mut **value)
    }

    #[inline]
    fn insert(&mut self, index: usize, element: Box<dyn Reflect>) {
        self.values.insert(index, element);
    }

    fn try_insert(
        &mut self,
        index: usize,
        value: Box<dyn Reflect>,
    ) -> Result<(), Box<dyn Reflect>> {
        if index <= self.values.len() {
            self.values.insert(index, value);
            Ok(())
        } else {
            Err(value)
        }
    }

    #[inline]
    fn remove(&mut self, index: usize) -> Box<dyn Reflect> {
        self.values.remove(index)
    }

    #[inline]
    fn push(&mut self, value: Box<dyn Reflect>) {
        self.values.push(value);
    }

    #[inline]
    fn try_push(&mut self, value: Box<dyn Reflect>) -> Result<(), Box<dyn Reflect>> {
        self.values.push(value);
        Ok(())
    }

    #[inline]
    fn pop(&mut self) -> Option<Box<dyn Reflect>> {
        self.values.pop()
    }

    #[inline]
    fn len(&self) -> usize {
        self.values.len()
    }

    #[inline]
    fn iter(&self) -> ListItemIter<'_> {
        ListItemIter::new(self)
    }

    #[inline]
    fn drain(&mut self) -> Vec<Box<dyn Reflect>> {
        self.values.drain(..).collect()
    }

    #[inline]
    fn reflect_list_info(&self) -> Option<&'static ListInfo> {
        None
    }

    #[inline]
    fn represented_list_info(&self) -> Option<&'static ListInfo> {
        self.list_info?.as_list().ok()
    }
}
