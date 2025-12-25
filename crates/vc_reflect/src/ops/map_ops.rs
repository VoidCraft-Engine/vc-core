use crate::{
    Reflect,
    impls::NonGenericTypeInfoCell,
    info::{MapInfo, OpaqueInfo, TypeInfo, TypePath, Typed},
    ops::{ApplyError, ReflectCloneError},
    reflection::impl_reflect_cast_fn,
};
use alloc::{boxed::Box, vec::Vec};
use core::fmt;
use vc_utils::hash::{HashTable, hash_table};

/// Representing [`Map`], used to dynamically modify the type of data and information.
///
/// Dynamic types are special in that their TypeInfo is [`OpaqueInfo`],
/// but other APIs are consistent with the type they represent, such as [`reflect_kind`], [`reflect_ref`]
///
/// [`reflect_kind`]: crate::Reflect::reflect_kind
/// [`reflect_ref`]: crate::Reflect::reflect_ref
pub struct DynamicMap {
    map_info: Option<&'static TypeInfo>,
    hash_table: HashTable<(Box<dyn Reflect>, Box<dyn Reflect>)>,
}

impl TypePath for DynamicMap {
    #[inline]
    fn type_path() -> &'static str {
        "vc_reflect::ops::DynamicMap"
    }
    #[inline]
    fn type_name() -> &'static str {
        "DynamicMap"
    }
    #[inline]
    fn type_ident() -> &'static str {
        "DynamicMap"
    }
    #[inline]
    fn module_path() -> Option<&'static str> {
        Some("vc_reflect::ops")
    }
}

impl Typed for DynamicMap {
    fn type_info() -> &'static TypeInfo {
        static CELL: NonGenericTypeInfoCell = NonGenericTypeInfoCell::new();
        CELL.get_or_init(|| TypeInfo::Opaque(OpaqueInfo::new::<Self>()))
    }
}

impl DynamicMap {
    /// Create a empty [`DynamicMap`].
    #[inline]
    pub const fn new() -> Self {
        Self {
            map_info: None,
            hash_table: HashTable::new(),
        }
    }

    /// See [`Vec::with_capacity`]
    #[inline]
    pub fn with_capacity(capacity: usize) -> Self {
        Self {
            map_info: None,
            hash_table: HashTable::with_capacity(capacity),
        }
    }

    /// Sets the [`TypeInfo`] to be represented by this `DynamicMap`.
    ///
    /// # Panic
    ///
    /// If the input is not list info or None.
    #[inline]
    pub fn set_type_info(&mut self, map_info: Option<&'static TypeInfo>) {
        match map_info {
            Some(TypeInfo::Map(_)) | None => {}
            _ => {
                panic!(
                    "Call `DynamicMap::set_type_info`, but the input is not map information or None."
                )
            }
        }

        self.map_info = map_info;
    }

    fn internal_hash(value: &dyn Reflect) -> u64 {
        value.reflect_hash().unwrap_or_else(|| {
            panic!(
                "the given value of type `{}` does not support reflect hashing",
                value.reflect_type_path(),
            );
        })
    }

    fn internal_eq(
        key: &dyn Reflect,
    ) -> impl FnMut(&(Box<dyn Reflect>, Box<dyn Reflect>)) -> bool + '_ {
        |(other, _)| {
            key
            .reflect_partial_eq(&**other)
            .expect("underlying type does not reflect `PartialEq` and hence doesn't support equality checks")
        }
    }
}

impl Reflect for DynamicMap {
    impl_reflect_cast_fn!(Map);

    #[inline]
    fn is_dynamic(&self) -> bool {
        true
    }

    #[inline]
    fn represented_type_info(&self) -> Option<&'static TypeInfo> {
        self.map_info
    }

    #[inline]
    fn to_dynamic(&self) -> Box<dyn Reflect> {
        Box::new(<Self as Map>::to_dynamic_map(self))
    }

    #[inline]
    fn reflect_clone(&self) -> Result<Box<dyn Reflect>, ReflectCloneError> {
        Ok(Box::new(<Self as Map>::to_dynamic_map(self)))
    }

    #[inline]
    fn try_apply(&mut self, value: &dyn Reflect) -> Result<(), ApplyError> {
        crate::impls::map_try_apply(self, value)
    }

    #[inline]
    fn reflect_partial_eq(&self, other: &dyn Reflect) -> Option<bool> {
        crate::impls::map_partial_eq(self, other)
    }

    #[inline]
    fn reflect_hash(&self) -> Option<u64> {
        crate::impls::map_hash(self)
    }

    #[inline]
    fn reflect_debug(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "DynamicMap(")?;
        crate::impls::map_debug(self, f)?;
        write!(f, ")")
    }
}

impl fmt::Debug for DynamicMap {
    #[inline]
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        self.reflect_debug(f)
    }
}

impl FromIterator<(Box<dyn Reflect>, Box<dyn Reflect>)> for DynamicMap {
    fn from_iter<I: IntoIterator<Item = (Box<dyn Reflect>, Box<dyn Reflect>)>>(items: I) -> Self {
        let mut this = DynamicMap::new();
        for (key, value) in items.into_iter() {
            this.insert(key, value);
        }
        this
    }
}

impl<K: Reflect, V: Reflect> FromIterator<(K, V)> for DynamicMap {
    fn from_iter<I: IntoIterator<Item = (K, V)>>(items: I) -> Self {
        let mut this = DynamicMap::new();
        for (key, value) in items.into_iter() {
            this.insert(Box::new(key), Box::new(value));
        }
        this
    }
}

impl IntoIterator for DynamicMap {
    type Item = (Box<dyn Reflect>, Box<dyn Reflect>);
    type IntoIter = hash_table::IntoIter<Self::Item>;

    #[inline]
    fn into_iter(self) -> Self::IntoIter {
        self.hash_table.into_iter()
    }
}

impl<'a> IntoIterator for &'a DynamicMap {
    type Item = (&'a dyn Reflect, &'a dyn Reflect);
    type IntoIter = core::iter::Map<
        hash_table::Iter<'a, (Box<dyn Reflect>, Box<dyn Reflect>)>,
        fn(&'a (Box<dyn Reflect>, Box<dyn Reflect>)) -> Self::Item,
    >;

    #[inline]
    fn into_iter(self) -> Self::IntoIter {
        self.hash_table.iter().map(|(k, v)| (&**k, &**v))
    }
}

/// A trait used to power [map-like] operations via [reflection].
///
/// Maps contain zero or more entries of a key and its associated value,
/// and correspond to types like `HashMap` and [`BTreeMap`].
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
/// as it is still relied on by [`DynamicMap`].
///
/// # Example
///
/// ```
/// use vc_reflect::{Reflect, ops::Map};
/// use std::collections::BTreeMap;
///
/// let foo: &mut dyn Map = &mut BTreeMap::<u32, bool>::new();
/// foo.insert(Box::new(123_u32), Box::new(true));
/// assert_eq!(foo.len(), 1);
///
/// let field: &dyn Reflect = foo.get(&123_u32).unwrap();
/// assert_eq!(field.downcast_ref::<bool>(), Some(&true));
/// ```
///
/// [`BTreeMap`]: alloc::collections::BTreeMap
/// [map-like]: https://doc.rust-lang.org/book/ch08-03-hash-maps.html
/// [reflection]: crate
pub trait Map: Reflect {
    /// Returns a reference to the value associated with the given key.
    ///
    /// If no value is associated with `key`, returns `None`.
    fn get(&self, key: &dyn Reflect) -> Option<&dyn Reflect>;

    /// Returns a mutable reference to the value associated with the given key.
    ///
    /// If no value is associated with `key`, returns `None`.
    fn get_mut(&mut self, key: &dyn Reflect) -> Option<&mut dyn Reflect>;

    /// Returns the number of elements in the map.
    fn len(&self) -> usize;

    /// Returns `true` if the list contains no elements.
    #[inline]
    fn is_empty(&self) -> bool {
        self.len() == 0
    }

    /// Returns an iterator over the key-value pairs of the map.
    fn iter(&self) -> Box<dyn Iterator<Item = (&dyn Reflect, &dyn Reflect)> + '_>;

    /// Drain the key-value pairs of this map to get a vector of owned values.
    ///
    /// After calling this function, `self` will be empty.
    fn drain(&mut self) -> Vec<(Box<dyn Reflect>, Box<dyn Reflect>)>;

    /// Retain only the elements specified by the predicate.
    ///
    /// In other words, remove all pairs `(k, v)` such that `f(&k, &mut v)` returns `false`.
    fn retain(&mut self, f: &mut dyn FnMut(&dyn Reflect, &mut dyn Reflect) -> bool);

    /// Creates a new [`DynamicMap`] from this map.
    ///
    /// Usually, `to_dynamic_map` recursively converts all data to a dynamic type, except for 'Opaque'.
    /// But for Map keys, converting them to dynamic types is not a good idea.
    ///
    /// Therefore, for keys, we choose to directly clone them if feasible.
    fn to_dynamic_map(&self) -> DynamicMap {
        let mut map = DynamicMap::with_capacity(self.len());
        map.set_type_info(self.represented_type_info());
        for (key, value) in self.iter() {
            if let Ok(k) = key.reflect_clone() {
                debug_assert_eq!(
                    k.ty_id(),
                    key.ty_id(),
                    "`Reflect::reflect_clone` should return the same type: {}",
                    value.reflect_type_path(),
                );
                map.insert(k, value.to_dynamic());
            } else {
                map.insert(key.to_dynamic(), value.to_dynamic());
            }
        }
        map
    }

    /// Inserts a key-value pair into the map.
    ///
    /// If the map did not have this key present, `None` is returned.
    /// If the map did have this key present, the value is updated, and the old value is returned.
    ///
    /// # Panics
    ///
    /// May Panics if type incompatible.
    fn insert(
        &mut self,
        key: Box<dyn Reflect>,
        value: Box<dyn Reflect>,
    ) -> Option<Box<dyn Reflect>>;

    /// Try insert key values.
    ///
    /// If type incompatible, return  `Err((K, V))`.
    ///
    /// Use for `try_apply` implementation, should not panics.
    fn try_insert(
        &mut self,
        key: Box<dyn Reflect>,
        value: Box<dyn Reflect>,
    ) -> Result<Option<Box<dyn Reflect>>, (Box<dyn Reflect>, Box<dyn Reflect>)>;

    /// Removes an entry from the map.
    ///
    /// If the map did not have this key present, `None` is returned.
    /// If the map did have this key present, the removed value is returned.
    fn remove(&mut self, key: &dyn Reflect) -> Option<Box<dyn Reflect>>;

    /// Get actual [`MapInfo`] of underlying types.
    ///
    /// If it is a dynamic type, it will return `None`.
    ///
    /// If it is not a dynamic type and the returned value is not `None` or `MapInfo`, it will panic.
    /// (If you want to implement dynamic types yourself, please return None.)
    #[inline]
    fn reflect_map_info(&self) -> Option<&'static MapInfo> {
        self.reflect_type_info().as_map().ok()
    }

    /// Get the [`MapInfo`] of representation.
    ///
    /// Normal types return their own information,
    /// while dynamic types return `None`` if they do not represent an object
    #[inline]
    fn represented_map_info(&self) -> Option<&'static MapInfo> {
        self.represented_type_info()?.as_map().ok()
    }
}

impl Map for DynamicMap {
    #[inline]
    fn get(&self, key: &dyn Reflect) -> Option<&dyn Reflect> {
        self.hash_table
            .find(Self::internal_hash(key), Self::internal_eq(key))
            .map(|(_, value)| &**value)
    }

    #[inline]
    fn get_mut(&mut self, key: &dyn Reflect) -> Option<&mut dyn Reflect> {
        self.hash_table
            .find_mut(Self::internal_hash(key), Self::internal_eq(key))
            .map(|(_, value)| &mut **value)
    }

    #[inline]
    fn len(&self) -> usize {
        self.hash_table.len()
    }

    #[inline]
    fn iter(&self) -> Box<dyn Iterator<Item = (&dyn Reflect, &dyn Reflect)> + '_> {
        let iter = self.hash_table.iter().map(|(k, v)| (&**k, &**v));
        Box::new(iter)
    }

    #[inline]
    fn drain(&mut self) -> Vec<(Box<dyn Reflect>, Box<dyn Reflect>)> {
        self.hash_table.drain().collect()
    }

    #[inline]
    fn retain(&mut self, f: &mut dyn FnMut(&dyn Reflect, &mut dyn Reflect) -> bool) {
        self.hash_table
            .retain(move |(key, value)| f(&**key, &mut **value));
    }

    fn insert(
        &mut self,
        key: Box<dyn Reflect>,
        value: Box<dyn Reflect>,
    ) -> Option<Box<dyn Reflect>> {
        debug_assert_eq!(
            key.reflect_partial_eq(&*key),
            Some(true),
            "keys inserted in `Map`-like types are expected to reflect `PartialEq`"
        );

        let hash = Self::internal_hash(&*key);
        let eq = Self::internal_eq(&*key);
        match self.hash_table.find_mut(hash, eq) {
            Some((_, old)) => Some(core::mem::replace(old, value)),
            None => {
                self.hash_table.insert_unique(
                    Self::internal_hash(key.as_ref()),
                    (key, value),
                    |(key, _)| Self::internal_hash(&**key),
                );
                None
            }
        }
    }

    #[inline]
    fn try_insert(
        &mut self,
        key: Box<dyn Reflect>,
        value: Box<dyn Reflect>,
    ) -> Result<Option<Box<dyn Reflect>>, (Box<dyn Reflect>, Box<dyn Reflect>)> {
        Ok(Map::insert(self, key, value))
    }

    fn remove(&mut self, key: &dyn Reflect) -> Option<Box<dyn Reflect>> {
        let hash = Self::internal_hash(key);
        let eq = Self::internal_eq(key);
        match self.hash_table.find_entry(hash, eq) {
            Ok(entry) => {
                let ((_, old_value), _) = entry.remove();
                Some(old_value)
            }
            Err(_) => None,
        }
    }

    #[inline]
    fn reflect_map_info(&self) -> Option<&'static MapInfo> {
        None
    }

    #[inline]
    fn represented_map_info(&self) -> Option<&'static MapInfo> {
        self.map_info?.as_map().ok()
    }
}
