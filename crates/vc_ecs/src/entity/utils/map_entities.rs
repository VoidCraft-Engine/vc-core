use core::hash::{BuildHasher, Hash};

use vc_utils::hash::{HashMap, HashSet};
use vc_utils::index::{IndexMap, IndexSet};

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

impl<K, V, S> MapEntities for HashMap<K, V, S>
where
    K: MapEntities + Eq + Hash,
    V: MapEntities,
    S: BuildHasher + Default,
{
    fn map_entities<E: EntityMapper>(&mut self, entity_mapper: &mut E) {
        *self = self
            .drain()
            .map(|(mut key_entities, mut value_entities)| {
                key_entities.map_entities(entity_mapper);
                value_entities.map_entities(entity_mapper);
                (key_entities, value_entities)
            })
            .collect();
    }
}

impl<T, S> MapEntities for HashSet<T, S>
where
    T: MapEntities + Eq + Hash,
    S: BuildHasher + Default,
{
    fn map_entities<E: EntityMapper>(&mut self, entity_mapper: &mut E) {
        *self = self
            .drain()
            .map(|mut entities| {
                entities.map_entities(entity_mapper);
                entities
            })
            .collect();
    }
}

impl<K, V, S> MapEntities for IndexMap<K, V, S>
where
    K: MapEntities + Eq + Hash,
    V: MapEntities,
    S: BuildHasher + Default,
{
    fn map_entities<E: EntityMapper>(&mut self, entity_mapper: &mut E) {
        *self = self
            .drain(..)
            .map(|(mut key_entities, mut value_entities)| {
                key_entities.map_entities(entity_mapper);
                value_entities.map_entities(entity_mapper);
                (key_entities, value_entities)
            })
            .collect();
    }
}

impl<T, S> MapEntities for IndexSet<T, S>
where
    T: MapEntities + Eq + Hash,
    S: BuildHasher + Default,
{
    fn map_entities<E: EntityMapper>(&mut self, entity_mapper: &mut E) {
        *self = self
            .drain(..)
            .map(|mut entities| {
                entities.map_entities(entity_mapper);
                entities
            })
            .collect();
    }
}
