use core::any::TypeId;

use crate::hash::{HashMap, NoOpHashState, hash_map::Entry};

/// A specialized hashmap type with Key of [`TypeId`]
/// Iteration order only depends on the order of insertions and deletions.
///
/// We chose `HashMap` over `BTreeMap`, assuming it would have better performance — though this hasn’t been tested.
pub type TypeIdMap<V> = HashMap<TypeId, V, NoOpHashState>;

impl<V> TypeIdMap<V> {
    /// Create a empty [`TypeIdMap`]
    ///
    /// Use `empty` instead of `new` to avoid duplicate.
    #[inline]
    pub const fn empty() -> Self {
        Self::with_hasher(NoOpHashState)
    }

    /// Inserts a value for the type `T`.
    #[inline(always)]
    pub fn insert_type<T: ?Sized + 'static>(&mut self, v: V) -> Option<V> {
        self.insert(TypeId::of::<T>(), v)
    }

    /// Returns a reference to the value for type `T`, if one exists.
    #[inline(always)]
    pub fn get_type<T: ?Sized + 'static>(&self) -> Option<&V> {
        self.get(&TypeId::of::<T>())
    }

    /// Returns a mutable reference to the value for type `T`, if one exists.
    #[inline(always)]
    pub fn get_type_mut<T: ?Sized + 'static>(&mut self) -> Option<&mut V> {
        self.get_mut(&TypeId::of::<T>())
    }

    /// Removes type `T` from the map, returning the value for this
    /// key if it was previously present.
    #[inline(always)]
    pub fn remove_type<T: ?Sized + 'static>(&mut self) -> Option<V> {
        self.remove(&TypeId::of::<T>())
    }

    /// Gets the type `T`'s entry in the map for in-place manipulation.
    #[inline(always)]
    pub fn entry_type<T: ?Sized + 'static>(&mut self) -> Entry<'_, TypeId, V, NoOpHashState> {
        self.entry(TypeId::of::<T>())
    }
}
