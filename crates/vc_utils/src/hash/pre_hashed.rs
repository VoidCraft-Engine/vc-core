use core::{
    fmt::Debug,
    hash::{BuildHasher, Hash, Hasher},
    marker::PhantomData,
    ops::Deref,
};

use hashbrown::hash_map::RawEntryMut;

use crate::hash::{FixedHashState, HashMap, NoOpHashState};

/// A pre-hashed value of a specific type.
/// Pre-hashing enables memoization of hashes that are expensive to compute.
///
/// It also enables faster [`PartialEq`] comparisons by short circuiting on hash equality.
/// See `PreHashMap` for a hashmap pre-configured to use [`Hashed`] keys.
pub struct Hashed<V: Hash + Eq + PartialEq + Clone, S: BuildHasher + Default = FixedHashState> {
    hash: u64,
    value: V,
    marker: PhantomData<S>,
}

impl<V: Hash + Eq + PartialEq + Clone, S: BuildHasher + Default> Hashed<V, S> {
    /// Pre-hashes the given value using the [`BuildHasher`] configured in the [`Hashed`] type.
    #[inline]
    pub fn new(value: V) -> Self {
        Self {
            hash: S::default().hash_one(&value),
            value,
            marker: PhantomData,
        }
    }

    /// The pre-computed hash.
    #[inline(always)]
    pub fn hash(&self) -> u64 {
        self.hash
    }
}

impl<V: Hash + Eq + PartialEq + Clone, S: BuildHasher + Default> Hash for Hashed<V, S> {
    #[inline]
    fn hash<R: Hasher>(&self, state: &mut R) {
        state.write_u64(self.hash);
    }
}

impl<V: Hash + Eq + PartialEq + Clone, S: BuildHasher + Default> Deref for Hashed<V, S> {
    type Target = V;

    #[inline]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl<V: Hash + Eq + PartialEq + Clone, S: BuildHasher + Default> PartialEq for Hashed<V, S> {
    #[inline]
    fn eq(&self, other: &Self) -> bool {
        self.hash == other.hash && self.value.eq(&other.value)
    }
}

impl<V: Hash + Eq + PartialEq + Clone, S: BuildHasher + Default> Eq for Hashed<V, S> {}

impl<V: Hash + Eq + PartialEq + Clone + Debug, S: BuildHasher + Default> Debug for Hashed<V, S> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        // Not inline: Debug allows for slight performance loss
        f.debug_struct("Hashed")
            .field("hash", &self.hash)
            .field("value", &self.value)
            .finish()
    }
}

impl<V: Hash + Eq + PartialEq + Clone, S: BuildHasher + Default> Clone for Hashed<V, S> {
    #[inline]
    fn clone(&self) -> Self {
        Self {
            hash: self.hash,
            value: self.value.clone(),
            marker: PhantomData,
        }
    }
}

impl<V: Hash + Eq + PartialEq + Clone + Copy, S: BuildHasher + Default> Copy for Hashed<V, S> {}

/// A [`HashMap`] pre-configured to use [`Hashed`] keys and [`PassHash`] passthrough hashing.
/// Iteration order only depends on the order of insertions and deletions.
pub type PreHashMap<K, V> = HashMap<Hashed<K>, V, NoOpHashState>;

impl<K: Hash + Eq + PartialEq + Clone, V> PreHashMap<K, V> {
    /// Create a empty [`PreHashMap`]
    ///
    /// Use `empty` instead of `new` to avoid duplicate name.
    #[inline]
    pub const fn empty() -> Self {
        Self::with_hasher(NoOpHashState)
    }
}

impl<K: Hash + Eq + PartialEq + Clone, V> PreHashMap<K, V> {
    /// Tries to get or insert the value for the given `key` using the pre-computed hash first.
    /// If the [`PreHashMap`] does not already contain the `key`, it will clone it and insert
    /// the value returned by `func`.
    pub fn get_or_insert_with<F: FnOnce() -> V>(&mut self, key: &Hashed<K>, func: F) -> &mut V {
        let entry: RawEntryMut<'_, Hashed<K>, V, NoOpHashState> = self
            .raw_entry_mut()
            .from_key_hashed_nocheck(key.hash(), key);

        match entry {
            RawEntryMut::Occupied(entry) => entry.into_mut(),
            RawEntryMut::Vacant(entry) => {
                let (_, value) = entry.insert_hashed_nocheck(key.hash(), key.clone(), func());
                value
            }
        }
    }
}
