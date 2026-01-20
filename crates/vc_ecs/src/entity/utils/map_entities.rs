use alloc::collections::{BTreeMap, BTreeSet, VecDeque};
use alloc::vec::Vec;
use core::hash::{BuildHasher, Hash};

use vc_utils::hash::{HashMap, HashSet};
use vc_utils::hash::{NoOpHashMap, NoOpHashSet};
use vc_utils::hash::{SparseHashMap, SparseHashSet};
use vc_utils::index::{IndexMap, IndexSet};
use vc_utils::index::{SparseIndexMap, SparseIndexSet};
use vc_utils::vec::{AutoVec, FastVec};

// `EntityHashMap<T>` is `SparseHashMap<Entity, T>`.
// `EntityIndexMap<T>` is `SparseIndexMap<Entity, T>`.
use super::{EntityHashMap, EntityIndexMap};

use crate::entity::Entity;

// -----------------------------------------------------------------------------
// EntityMapper

pub trait EntityMapper {
    fn get_mapped(&mut self, source: Entity) -> Entity;
    fn set_mapped(&mut self, source: Entity, target: Entity);
}

// -----------------------------------------------------------------------------
// MapEntities

pub trait MapEntities {
    fn map_entities<E: EntityMapper>(&mut self, entity_mapper: &mut E);
}

// -----------------------------------------------------------------------------
// MapEntities Implementation

impl MapEntities for Entity {
    fn map_entities<E: EntityMapper>(&mut self, entity_mapper: &mut E) {
        *self = entity_mapper.get_mapped(*self);
    }
}

impl<T: MapEntities> MapEntities for Option<T> {
    fn map_entities<E: EntityMapper>(&mut self, entity_mapper: &mut E) {
        if let Some(entities) = self {
            entities.map_entities(entity_mapper);
        }
    }
}

macro_rules! map_entities_for_hashmap {
    () => {
        fn map_entities<E: EntityMapper>(&mut self, entity_mapper: &mut E) {
            *self = self
                .drain()
                .map(|(mut key, mut val)| {
                    key.map_entities(entity_mapper);
                    val.map_entities(entity_mapper);
                    (key, val)
                })
                .collect();
        }
    };
}

macro_rules! map_entities_for_indexmap {
    () => {
        fn map_entities<E: EntityMapper>(&mut self, entity_mapper: &mut E) {
            *self = self
                .drain(..)
                .map(|(mut key, mut val)| {
                    key.map_entities(entity_mapper);
                    val.map_entities(entity_mapper);
                    (key, val)
                })
                .collect();
        }
    };
}

macro_rules! map_entities_for_hashset {
    () => {
        fn map_entities<E: EntityMapper>(&mut self, entity_mapper: &mut E) {
            *self = self
                .drain()
                .map(|mut item| {
                    item.map_entities(entity_mapper);
                    item
                })
                .collect();
        }
    };
}

macro_rules! map_entities_for_indexset {
    () => {
        fn map_entities<E: EntityMapper>(&mut self, entity_mapper: &mut E) {
            *self = self
                .drain(..)
                .map(|mut item| {
                    item.map_entities(entity_mapper);
                    item
                })
                .collect();
        }
    };
}

impl<K, V, S> MapEntities for HashMap<K, V, S>
where
    K: MapEntities + Eq + Hash,
    V: MapEntities,
    S: BuildHasher + Default,
{
    map_entities_for_hashmap! {}
}

impl<K, V> MapEntities for SparseHashMap<K, V>
where
    K: MapEntities + Eq + Hash,
    V: MapEntities,
{
    map_entities_for_hashmap! {}
}

impl<K, V> MapEntities for NoOpHashMap<K, V>
where
    K: MapEntities + Eq + Hash,
    V: MapEntities,
{
    map_entities_for_hashmap! {}
}

impl<T, S> MapEntities for HashSet<T, S>
where
    T: MapEntities + Eq + Hash,
    S: BuildHasher + Default,
{
    map_entities_for_hashset! {}
}

impl<T> MapEntities for SparseHashSet<T>
where
    T: MapEntities + Eq + Hash,
{
    map_entities_for_hashset! {}
}

impl<T> MapEntities for NoOpHashSet<T>
where
    T: MapEntities + Eq + Hash,
{
    map_entities_for_hashset! {}
}

impl<K, V, S> MapEntities for IndexMap<K, V, S>
where
    K: MapEntities + Eq + Hash,
    V: MapEntities,
    S: BuildHasher + Default,
{
    map_entities_for_indexmap! {}
}

impl<K, V> MapEntities for SparseIndexMap<K, V>
where
    K: MapEntities + Eq + Hash,
    V: MapEntities,
{
    map_entities_for_indexmap! {}
}

impl<T, S> MapEntities for IndexSet<T, S>
where
    T: MapEntities + Eq + Hash,
    S: BuildHasher + Default,
{
    map_entities_for_indexset! {}
}

impl<T> MapEntities for SparseIndexSet<T>
where
    T: MapEntities + Eq + Hash,
{
    map_entities_for_indexset! {}
}

impl<K, V> MapEntities for BTreeMap<K, V>
where
    K: MapEntities + Ord,
    V: MapEntities,
{
    fn map_entities<E: EntityMapper>(&mut self, entity_mapper: &mut E) {
        *self = core::mem::take(self)
            .into_iter()
            .map(|(mut key, mut value)| {
                key.map_entities(entity_mapper);
                value.map_entities(entity_mapper);
                (key, value)
            })
            .collect();
    }
}

impl<T> MapEntities for BTreeSet<T>
where
    T: MapEntities + Ord,
{
    fn map_entities<E: EntityMapper>(&mut self, entity_mapper: &mut E) {
        *self = core::mem::take(self)
            .into_iter()
            .map(|mut item| {
                item.map_entities(entity_mapper);
                item
            })
            .collect();
    }
}

impl<T: MapEntities, const N: usize> MapEntities for [T; N] {
    fn map_entities<E: EntityMapper>(&mut self, entity_mapper: &mut E) {
        for entities in self.iter_mut() {
            entities.map_entities(entity_mapper);
        }
    }
}

impl<T: MapEntities> MapEntities for Vec<T> {
    fn map_entities<E: EntityMapper>(&mut self, entity_mapper: &mut E) {
        for entities in self.iter_mut() {
            entities.map_entities(entity_mapper);
        }
    }
}

impl<T: MapEntities> MapEntities for VecDeque<T> {
    fn map_entities<E: EntityMapper>(&mut self, entity_mapper: &mut E) {
        for entities in self.iter_mut() {
            entities.map_entities(entity_mapper);
        }
    }
}

impl<T: MapEntities, const N: usize> MapEntities for FastVec<T, N> {
    fn map_entities<E: EntityMapper>(&mut self, entity_mapper: &mut E) {
        // `iter_mut` is `as_slice_mut + iter_mut`, it's safe and fast.
        for entities in self.iter_mut() {
            entities.map_entities(entity_mapper);
        }
    }
}

impl<T: MapEntities, const N: usize> MapEntities for AutoVec<T, N> {
    fn map_entities<E: EntityMapper>(&mut self, entity_mapper: &mut E) {
        for entities in self.iter_mut() {
            entities.map_entities(entity_mapper);
        }
    }
}

// -----------------------------------------------------------------------------
// EntityMapper Implementation

/// A mapper that do nothing.
impl EntityMapper for () {
    // return `source` directly
    #[inline(always)]
    fn get_mapped(&mut self, source: Entity) -> Entity {
        source
    }

    /// do nothing
    #[inline(always)]
    fn set_mapped(&mut self, _source: Entity, _target: Entity) {}
}

impl EntityMapper for (Entity, Entity) {
    // - If source == `self.0`, return `self.1` .
    // - Otherwise return `self.0`.
    #[inline(always)]
    fn get_mapped(&mut self, source: Entity) -> Entity {
        if source == self.0 { self.1 } else { source }
    }

    /// do nothing
    #[inline(always)]
    fn set_mapped(&mut self, _source: Entity, _target: Entity) {}
}

impl EntityMapper for &mut dyn EntityMapper {
    #[inline(always)]
    fn get_mapped(&mut self, source: Entity) -> Entity {
        (*self).get_mapped(source)
    }

    #[inline(always)]
    fn set_mapped(&mut self, source: Entity, target: Entity) {
        (*self).set_mapped(source, target);
    }
}

impl EntityMapper for EntityHashMap<Entity> {
    /// Returns the corresponding mapped entity
    /// or returns `entity` if there is no mapped entity
    fn get_mapped(&mut self, source: Entity) -> Entity {
        self.get(&source).copied().unwrap_or(source)
    }

    fn set_mapped(&mut self, source: Entity, target: Entity) {
        self.insert(source, target);
    }
}

impl EntityMapper for EntityIndexMap<Entity> {
    /// Returns the corresponding mapped entity
    /// or returns `entity` if there is no mapped entity
    fn get_mapped(&mut self, source: Entity) -> Entity {
        self.get(&source).copied().unwrap_or(source)
    }

    fn set_mapped(&mut self, source: Entity, target: Entity) {
        self.insert(source, target);
    }
}

// -----------------------------------------------------------------------------
// SceneEntityMapper Implementation

/// A wrapper for [`EntityHashMap<Entity>`], augmenting it with the ability
/// to allocate new [`Entity`] references in a destination world.
///
/// These newly allocated references are guaranteed to never point to any living
/// entity in that world.
///
/// References are allocated by returning increasing generations starting from an
/// internally initialized base [`Entity`]. After it is finished being used, this
/// entity is despawned and the requisite number of generations reserved.
pub struct SceneEntityMapper<'m> {
    /// A mapping from one set of entities to another.
    map: &'m mut EntityHashMap<Entity>,
    /// A base [`Entity`] used to allocate new references.
    dead_start: Entity,
    /// The number of generations this mapper has allocated thus far.
    versions: u32,
}

impl EntityMapper for SceneEntityMapper<'_> {
    /// Returns the corresponding mapped entity or reserves a new
    /// dead entity ID in the current world if it is absent.
    fn get_mapped(&mut self, source: Entity) -> Entity {
        if let Some(&mapped) = self.map.get(&source) {
            return mapped;
        }

        let id = self.dead_start.id();
        let generation = self.dead_start.generation().after(self.versions);
        // this new entity reference is specifically designed to never represent any living entity
        let dst = Entity::new(id, generation);

        self.versions = self.versions.wrapping_add(1);

        self.map.insert(source, dst);
        dst
    }

    fn set_mapped(&mut self, source: Entity, target: Entity) {
        self.map.insert(source, target);
    }
}

use crate::world::World;

impl<'m> SceneEntityMapper<'m> {
    /// Creates a new [`SceneEntityMapper`], spawning a temporary base [`Entity`] in the provided [`World`]
    pub fn new(world: &World, map: &'m mut EntityHashMap<Entity>) -> Self {
        Self {
            map,
            dead_start: world.allocator.alloc(),
            versions: 0,
        }
    }

    /// Gets a reference to the underlying [`EntityHashMap<Entity>`].
    pub fn get_map(&'m self) -> &'m EntityHashMap<Entity> {
        self.map
    }

    /// Gets a mutable reference to the underlying [`EntityHashMap<Entity>`].
    pub fn get_map_mut(&'m mut self) -> &'m mut EntityHashMap<Entity> {
        self.map
    }

    pub fn finish(self, world: &mut World) {
        // SAFETY: We never constructed the entity and never released it for something else to construct.
        let reuse_row = unsafe {
            world
                .entities
                .make_free(self.dead_start.id(), self.versions)
        };
        world.allocator.free(reuse_row);
    }

    pub fn world_scope<R>(
        world: &mut World,
        entity_map: &'m mut EntityHashMap<Entity>,
        f: impl FnOnce(&mut World, &mut Self) -> R,
    ) -> R {
        let mut mapper = Self::new(world, entity_map);
        let result = f(world, &mut mapper);
        mapper.finish(world);
        result
    }
}
