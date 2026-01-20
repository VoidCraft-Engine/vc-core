use vc_utils::hash::SparseHashState;
use vc_utils::hash::{SparseHashMap, hash_map};
use vc_utils::hash::{SparseHashSet, hash_set};
use vc_utils::index::{SparseIndexMap, map};
use vc_utils::index::{SparseIndexSet, set};

use super::EntitySetIterator;
use crate::entity::Entity;

// -----------------------------------------------------------------------------
// Alias

pub type EntityHashMap<V> = SparseHashMap<Entity, V>;
pub type EntityHashSet = SparseHashSet<Entity>;
pub type EntityIndexMap<V> = SparseIndexMap<Entity, V>;
pub type EntityIndexSet = SparseIndexSet<Entity>;

// -----------------------------------------------------------------------------
// EntityHashMap

unsafe impl<V> EntitySetIterator for hash_map::IntoKeys<Entity, V> {}
unsafe impl<V> EntitySetIterator for hash_map::Keys<'_, Entity, V> {}

// -----------------------------------------------------------------------------
// EntityIndexMap

unsafe impl<V> EntitySetIterator for map::Keys<'_, Entity, V> {}
unsafe impl<V> EntitySetIterator for map::IntoKeys<Entity, V> {}

// -----------------------------------------------------------------------------
// EntityHashSet

unsafe impl EntitySetIterator for hash_set::Iter<'_, Entity> {}
unsafe impl EntitySetIterator for hash_set::IntoIter<Entity> {}
unsafe impl EntitySetIterator for hash_set::Drain<'_, Entity> {}
unsafe impl EntitySetIterator for hash_set::Difference<'_, Entity, SparseHashState> {}
unsafe impl EntitySetIterator for hash_set::Intersection<'_, Entity, SparseHashState> {}
unsafe impl EntitySetIterator for hash_set::SymmetricDifference<'_, Entity, SparseHashState> {}
unsafe impl EntitySetIterator for hash_set::Union<'_, Entity, SparseHashState> {}
unsafe impl<F: FnMut(&Entity) -> bool> EntitySetIterator for hash_set::ExtractIf<'_, Entity, F> {}

// -----------------------------------------------------------------------------
// EntityIndexMap

unsafe impl EntitySetIterator for set::Iter<'_, Entity> {}
unsafe impl EntitySetIterator for set::IntoIter<Entity> {}
unsafe impl EntitySetIterator for set::Drain<'_, Entity> {}
unsafe impl EntitySetIterator for set::Difference<'_, Entity, SparseHashState> {}
unsafe impl EntitySetIterator for set::Intersection<'_, Entity, SparseHashState> {}
unsafe impl EntitySetIterator for set::Union<'_, Entity, SparseHashState> {}
unsafe impl EntitySetIterator
    for set::SymmetricDifference<'_, Entity, SparseHashState, SparseHashState>
{
}
unsafe impl<I: Iterator<Item = Entity>> EntitySetIterator
    for set::Splice<'_, I, Entity, SparseHashState>
{
}
