use crate::{
    Reflect,
    impls::NonGenericTypeInfoCell,
    info::{OpaqueInfo, SetInfo, TypeInfo, TypePath, Typed},
    ops::{ApplyError, ReflectCloneError},
    reflection::impl_reflect_cast_fn,
};
use alloc::{boxed::Box, vec::Vec};
use core::fmt;
use vc_utils::hash::{HashTable, hash_table};

/// Represents a [`Set`], used to dynamically modify data and its reflected type information.
///
/// Dynamic types are special in that their `TypeInfo` is [`OpaqueInfo`],
/// but other APIs behave like the represented type, such as [`reflect_kind`] and [`reflect_ref`].
///
/// [`reflect_kind`]: crate::Reflect::reflect_kind
/// [`reflect_ref`]: crate::Reflect::reflect_ref
pub struct DynamicSet {
    set_info: Option<&'static TypeInfo>,
    hash_table: HashTable<Box<dyn Reflect>>,
}

impl TypePath for DynamicSet {
    #[inline]
    fn type_path() -> &'static str {
        "vc_reflect::ops::DynamicSet"
    }

    #[inline]
    fn type_name() -> &'static str {
        "DynamicSet"
    }

    #[inline]
    fn type_ident() -> &'static str {
        "DynamicSet"
    }

    #[inline]
    fn module_path() -> Option<&'static str> {
        Some("vc_reflect::ops")
    }
}

impl Typed for DynamicSet {
    fn type_info() -> &'static TypeInfo {
        static CELL: NonGenericTypeInfoCell = NonGenericTypeInfoCell::new();
        CELL.get_or_init(|| TypeInfo::Opaque(OpaqueInfo::new::<Self>()))
    }
}

impl DynamicSet {
    /// Create a empty [`DynamicSet`].
    #[inline]
    pub const fn new() -> Self {
        Self {
            set_info: None,
            hash_table: HashTable::new(),
        }
    }

    /// See [`Vec::with_capacity`]
    #[inline]
    pub fn with_capacity(capacity: usize) -> Self {
        Self {
            set_info: None,
            hash_table: HashTable::with_capacity(capacity),
        }
    }

    /// Sets the [`TypeInfo`] to be represented by this `DynamicSet`.
    ///
    /// # Panic
    ///
    /// If the input is not list info or None.
    #[inline]
    pub fn set_type_info(&mut self, set_info: Option<&'static TypeInfo>) {
        match set_info {
            Some(TypeInfo::Set(_)) | None => {}
            _ => {
                panic!(
                    "Call `DynamicSet::set_type_info`, but the input is not set information or None."
                )
            }
        }

        self.set_info = set_info;
    }

    fn internal_hash(value: &dyn Reflect) -> u64 {
        value.reflect_hash().unwrap_or_else(|| {
            panic!(
                "the given value of type `{}` does not support reflect hashing",
                value.reflect_type_path(),
            );
        })
    }

    fn internal_eq(value: &dyn Reflect) -> impl FnMut(&Box<dyn Reflect>) -> bool + '_ {
        |other| {
            value
                .reflect_partial_eq(&**other)
                .expect("Underlying type does not reflect `PartialEq` and hence doesn't support equality checks")
        }
    }
}

impl Reflect for DynamicSet {
    impl_reflect_cast_fn!(Set);

    #[inline]
    fn is_dynamic(&self) -> bool {
        true
    }

    #[inline]
    fn represented_type_info(&self) -> Option<&'static TypeInfo> {
        self.set_info
    }

    #[inline]
    fn to_dynamic(&self) -> Box<dyn Reflect> {
        Box::new(<Self as Set>::to_dynamic_set(self))
    }

    #[inline]
    fn reflect_clone(&self) -> Result<Box<dyn Reflect>, ReflectCloneError> {
        Ok(Box::new(<Self as Set>::to_dynamic_set(self)))
    }

    #[inline]
    fn try_apply(&mut self, value: &dyn Reflect) -> Result<(), ApplyError> {
        crate::impls::set_try_apply(self, value)
    }

    #[inline]
    fn reflect_hash(&self) -> Option<u64> {
        crate::impls::set_hash(self)
    }

    #[inline]
    fn reflect_partial_eq(&self, other: &dyn Reflect) -> Option<bool> {
        crate::impls::set_partial_eq(self, other)
    }

    #[inline]
    fn reflect_debug(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "DynamicSet(")?;
        crate::impls::set_debug(self, f)?;
        write!(f, ")")
    }
}

impl fmt::Debug for DynamicSet {
    #[inline]
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        self.reflect_debug(f)
    }
}

impl FromIterator<Box<dyn Reflect>> for DynamicSet {
    fn from_iter<I: IntoIterator<Item = Box<dyn Reflect>>>(values: I) -> Self {
        let mut this = DynamicSet::new();

        for value in values {
            this.insert(value);
        }

        this
    }
}

impl<T: Reflect> FromIterator<T> for DynamicSet {
    fn from_iter<I: IntoIterator<Item = T>>(values: I) -> Self {
        let mut this = DynamicSet::new();

        for value in values {
            this.insert(Box::new(value));
        }

        this
    }
}

impl IntoIterator for DynamicSet {
    type Item = Box<dyn Reflect>;
    type IntoIter = hash_table::IntoIter<Self::Item>;

    #[inline]
    fn into_iter(self) -> Self::IntoIter {
        self.hash_table.into_iter()
    }
}

impl<'a> IntoIterator for &'a DynamicSet {
    type Item = &'a dyn Reflect;
    type IntoIter = core::iter::Map<
        hash_table::Iter<'a, Box<dyn Reflect>>,
        fn(&'a Box<dyn Reflect>) -> Self::Item,
    >;

    #[inline]
    fn into_iter(self) -> Self::IntoIter {
        self.hash_table.iter().map(|v| v.as_ref())
    }
}

/// A trait used to power [set-like] operations via [reflection].
///
/// Sets contain zero or more entries of a fixed type, and correspond to types
/// like `HashSet` and [`BTreeSet`].
/// The order of these entries is not guaranteed by this trait.
///
/// # Hashing and equality
///
/// All keys are expected to return a valid hash value from [`Reflect::reflect_hash`] and be
/// comparable using [`Reflect::reflect_partial_eq`].
///
/// If using the [`#[derive(Reflect)]`](crate::derive::Reflect) macro, these functions will provide
/// default implementations (through internal fields), but this is usually not efficient enough.
/// You can add `#[reflect(hash, partial_eq)]` to implement these functions using [`Clone`]
/// and [`PartialEq`] trait.
///
/// The ordering is expected to be total, that is as if the reflected type implements the [`Eq`] trait.
/// This is true even for manual implementors who do not hash or compare values,
/// as it is still relied on by [`DynamicSet`].
///
/// # Example
///
/// ```
/// use vc_reflect::{Reflect, ops::Set};
/// use std::collections::BTreeSet;
///
/// let foo: &mut dyn Set = &mut BTreeSet::<u32>::new();
/// foo.insert(Box::new(123_u32));
/// assert_eq!(foo.len(), 1);
///
/// let field: &dyn Reflect = foo.get(&123_u32).unwrap();
/// assert_eq!(field.downcast_ref::<u32>(), Some(&123_u32));
/// ```
///
/// [`BTreeSet`]: alloc::collections::BTreeSet
/// [set-like]: https://doc.rust-lang.org/stable/std/collections/struct.HashSet.html
/// [reflection]: crate
pub trait Set: Reflect {
    /// Returns a reference to the value.
    ///
    /// If no value is contained, returns `None`.
    fn get(&self, value: &dyn Reflect) -> Option<&dyn Reflect>;

    /// Returns the number of elements in the set.
    fn len(&self) -> usize;

    /// Returns `true` if the list contains no elements.
    #[inline]
    fn is_empty(&self) -> bool {
        self.len() == 0
    }

    /// Returns an iterator over the values of the set.
    fn iter(&self) -> Box<dyn Iterator<Item = &dyn Reflect> + '_>;

    /// Drain the values of this set to get a vector of owned values.
    ///
    /// After calling this function, `self` will be empty.
    fn drain(&mut self) -> Vec<Box<dyn Reflect>>;

    /// Retain only the elements specified by the predicate.
    ///
    /// In other words, remove all elements `e` for which `f(&e)` returns `false`.
    fn retain(&mut self, f: &mut dyn FnMut(&dyn Reflect) -> bool);

    /// Creates a new [`DynamicSet`] from this set.
    ///
    /// Usually, `to_dynamic_map` recursively converts all data to a dynamic type,
    /// except for 'Opaque'. But for Set values, converting them to dynamic types
    /// is not a good idea, may cause changes in the result of hash and eq.
    ///
    /// Therefore,  we choose to directly clone them if feasible.
    fn to_dynamic_set(&self) -> DynamicSet {
        let mut set = DynamicSet::with_capacity(self.len());
        set.set_type_info(self.represented_type_info());
        for value in self.iter() {
            if let Ok(v) = value.reflect_clone() {
                debug_assert_eq!(
                    v.ty_id(),
                    value.ty_id(),
                    "`Reflect::reflect_clone` should return the same type: {}",
                    value.reflect_type_path(),
                );
                set.insert(v);
            } else {
                set.insert(value.to_dynamic());
            }
        }
        set
    }

    /// Inserts a value into the set.
    ///
    /// If the set did not have this value present, `true` is returned.
    /// If the set did have this value present, `false` is returned.
    ///
    /// # Panics
    ///
    /// May Panics if type incompatible.
    fn insert(&mut self, value: Box<dyn Reflect>) -> bool;

    /// Try insert key values.
    ///
    /// If type incompatible, return  `Err(V)`.
    ///
    /// Use for `try_apply` implementation, should not panics.
    fn try_insert(&mut self, value: Box<dyn Reflect>) -> Result<bool, Box<dyn Reflect>>;

    /// Removes a value from the set.
    ///
    /// If the set did not have this value present, `true` is returned.
    /// If the set did have this value present, `false` is returned.
    fn remove(&mut self, value: &dyn Reflect) -> bool;

    /// Checks if the given value is contained in the set
    fn contains(&self, value: &dyn Reflect) -> bool;

    /// Get actual [`SetInfo`] of underlying types.
    ///
    /// If it is a dynamic type, it will return `None`.
    ///
    /// If it is not a dynamic type and the returned value is not `None` or `SetInfo`, it will panic.
    /// (If you want to implement dynamic types yourself, please return None.)
    #[inline]
    fn reflect_set_info(&self) -> Option<&'static SetInfo> {
        self.reflect_type_info().as_set().ok()
    }

    /// Get the [`SetInfo`] of representation.
    ///
    /// Normal types return their own information,
    /// while dynamic types return `None`` if they do not represent an object
    #[inline]
    fn represented_set_info(&self) -> Option<&'static SetInfo> {
        self.represented_type_info()?.as_set().ok()
    }
}

impl Set for DynamicSet {
    #[inline]
    fn get(&self, value: &dyn Reflect) -> Option<&dyn Reflect> {
        self.hash_table
            .find(Self::internal_hash(value), Self::internal_eq(value))
            .map(|value| &**value)
    }

    #[inline]
    fn len(&self) -> usize {
        self.hash_table.len()
    }

    #[inline]
    fn iter(&self) -> Box<dyn Iterator<Item = &dyn Reflect> + '_> {
        Box::new(self.hash_table.iter().map(|v| &**v))
    }

    #[inline]
    fn drain(&mut self) -> Vec<Box<dyn Reflect>> {
        self.hash_table.drain().collect::<Vec<_>>()
    }

    #[inline]
    fn retain(&mut self, f: &mut dyn FnMut(&dyn Reflect) -> bool) {
        self.hash_table.retain(move |value| f(&**value));
    }

    fn insert(&mut self, value: Box<dyn Reflect>) -> bool {
        debug_assert_eq!(
            value.reflect_partial_eq(&*value),
            Some(true),
            "Values inserted in `Set` like types are expected to reflect `PartialEq`"
        );
        match self
            .hash_table
            .find_mut(Self::internal_hash(&*value), Self::internal_eq(&*value))
        {
            Some(old) => {
                *old = value;
                false
            }
            None => {
                self.hash_table.insert_unique(
                    Self::internal_hash(value.as_ref()),
                    value,
                    |boxed| Self::internal_hash(boxed.as_ref()),
                );
                true
            }
        }
    }

    #[inline]
    fn try_insert(&mut self, value: Box<dyn Reflect>) -> Result<bool, Box<dyn Reflect>> {
        Ok(Set::insert(self, value))
    }

    #[inline]
    fn remove(&mut self, value: &dyn Reflect) -> bool {
        self.hash_table
            .find_entry(Self::internal_hash(value), Self::internal_eq(value))
            .map(hash_table::OccupiedEntry::remove)
            .is_ok()
    }

    #[inline]
    fn contains(&self, value: &dyn Reflect) -> bool {
        self.hash_table
            .find(Self::internal_hash(value), Self::internal_eq(value))
            .is_some()
    }
}
