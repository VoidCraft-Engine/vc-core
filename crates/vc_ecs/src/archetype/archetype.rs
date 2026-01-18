#![expect(unsafe_code)]

use alloc::boxed::Box;
use alloc::vec::Vec;
use nonmax::NonMaxU32;

use super::{ArchetypeEntity, ArchetypeFlags, ArchetypeId, Edges};
use crate::archetype::ArchetypeRow;
use crate::component::ComponentId;
use crate::entity::{Entity, EntityLocation};
use crate::storage::{FixedSparseMap, StorageType, TableId, TableRow};

pub struct ArchetypeSwapRemoveResult {
    pub swapped_entity: Option<Entity>,
    pub table_row: TableRow,
}

pub struct Archetype {
    id: ArchetypeId,
    edges: Edges,
    flags: ArchetypeFlags,
    table_id: TableId,
    entities: Vec<ArchetypeEntity>,
    component_ids: Box<[ComponentId]>,
    storage_types: Box<[StorageType]>,
    sparse: FixedSparseMap<ComponentId, StorageType>,
}

impl Archetype {
    #[inline(always)]
    pub fn id(&self) -> ArchetypeId {
        self.id
    }

    #[inline(always)]
    pub fn flags(&self) -> ArchetypeFlags {
        self.flags
    }

    #[inline(always)]
    pub fn table_id(&self) -> TableId {
        self.table_id
    }

    #[inline(always)]
    pub fn edges(&self) -> &Edges {
        &self.edges
    }

    #[inline(always)]
    pub fn edges_mut(&mut self) -> &mut Edges {
        &mut self.edges
    }

    #[inline]
    pub fn entity_table_row(&self, row: ArchetypeRow) -> TableRow {
        self.entities[row.index()].table_row
    }

    #[inline]
    pub fn set_entity_table_row(&mut self, row: ArchetypeRow, table_row: TableRow) {
        self.entities[row.index()].table_row = table_row;
    }

    #[inline(always)]
    pub fn entities(&self) -> &[ArchetypeEntity] {
        &self.entities
    }

    #[inline]
    pub fn entities_with_location(&self) -> impl Iterator<Item = (Entity, EntityLocation)> {
        self.entities
            .iter()
            .enumerate()
            .map(|(arche_row, arche_entity)| {
                (
                    arche_entity.entity,
                    EntityLocation {
                        archetype_id: self.id,
                        archetype_row: unsafe {
                            ArchetypeRow::new(NonMaxU32::new_unchecked(arche_row as u32))
                        },
                        table_id: self.table_id,
                        table_row: arche_entity.table_row,
                    },
                )
            })
    }

    #[inline]
    pub fn table_components(&self) -> impl Iterator<Item = ComponentId> + '_ {
        self.component_ids
            .iter()
            .zip(self.storage_types.iter())
            .filter_map(|(&component, &storage)| {
                if storage == StorageType::Table {
                    Some(component)
                } else {
                    None
                }
            })
    }

    #[inline]
    pub fn sparse_set_components(&self) -> impl Iterator<Item = ComponentId> + '_ {
        self.component_ids
            .iter()
            .zip(self.storage_types.iter())
            .filter_map(|(&component, &storage)| {
                if storage == StorageType::SparseSet {
                    Some(component)
                } else {
                    None
                }
            })
    }

    #[inline]
    pub fn components(&self) -> &[ComponentId] {
        &self.component_ids
    }

    #[inline]
    pub fn iter_components(&self) -> impl Iterator<Item = ComponentId> + Clone {
        self.component_ids.iter().copied()
    }

    #[inline]
    pub fn component_count(&self) -> usize {
        self.component_ids.len()
    }

    #[inline]
    pub fn entity_count(&self) -> usize {
        self.entities.len()
    }

    #[inline]
    pub unsafe fn allocate(&mut self, entity: Entity, table_row: TableRow) -> EntityLocation {
        // SAFETY: An entity can not have multiple archetype rows and there can not be more than u32::MAX entities.
        let archetype_row =
            unsafe { ArchetypeRow::new(NonMaxU32::new_unchecked(self.entities.len() as u32)) };
        self.entities.push(ArchetypeEntity { entity, table_row });

        EntityLocation {
            archetype_id: self.id,
            archetype_row,
            table_id: self.table_id,
            table_row,
        }
    }

    #[inline]
    pub fn reserve(&mut self, additional: usize) {
        self.entities.reserve(additional);
    }

    #[inline]
    pub fn swap_remove(&mut self, row: ArchetypeRow) -> ArchetypeSwapRemoveResult {
        let row_index = row.index();
        let is_last: bool = row_index == self.entities.len() - 1;

        let table_row = self.entities.swap_remove(row_index).table_row;

        let swapped_entity = if is_last {
            None
        } else {
            unsafe { Some(self.entities.get_unchecked(row_index).entity) }
        };
        ArchetypeSwapRemoveResult {
            swapped_entity,
            table_row,
        }
    }

    #[inline]
    pub fn len(&self) -> usize {
        self.entities.len()
    }

    #[inline]
    pub fn is_empty(&self) -> bool {
        self.entities.is_empty()
    }

    #[inline]
    pub fn contains(&self, component_id: ComponentId) -> bool {
        self.sparse.contains(component_id)
    }

    #[inline]
    pub fn get_storage_type(&self, component_id: ComponentId) -> Option<StorageType> {
        self.sparse.get_copied(component_id)
    }

    pub fn clear_entities(&mut self) {
        self.entities.clear();
    }

    /// Returns true if any of the components in this archetype have `on_add` hooks
    #[inline]
    pub fn has_add_hook(&self) -> bool {
        self.flags().contains(ArchetypeFlags::ON_ADD_HOOK)
    }

    /// Returns true if any of the components in this archetype have `on_insert` hooks
    #[inline]
    pub fn has_insert_hook(&self) -> bool {
        self.flags().contains(ArchetypeFlags::ON_INSERT_HOOK)
    }

    /// Returns true if any of the components in this archetype have `on_replace` hooks
    #[inline]
    pub fn has_replace_hook(&self) -> bool {
        self.flags().contains(ArchetypeFlags::ON_REPLACE_HOOK)
    }

    /// Returns true if any of the components in this archetype have `on_remove` hooks
    #[inline]
    pub fn has_remove_hook(&self) -> bool {
        self.flags().contains(ArchetypeFlags::ON_REMOVE_HOOK)
    }

    /// Returns true if any of the components in this archetype have `on_despawn` hooks
    #[inline]
    pub fn has_despawn_hook(&self) -> bool {
        self.flags().contains(ArchetypeFlags::ON_DESPAWN_HOOK)
    }

    /// Returns true if any of the components in this archetype have at least one [`Add`] observer
    ///
    /// [`Add`]: crate::lifecycle::Add
    #[inline]
    pub fn has_add_observer(&self) -> bool {
        self.flags().contains(ArchetypeFlags::ON_ADD_OBSERVER)
    }

    /// Returns true if any of the components in this archetype have at least one [`Insert`] observer
    ///
    /// [`Insert`]: crate::lifecycle::Insert
    #[inline]
    pub fn has_insert_observer(&self) -> bool {
        self.flags().contains(ArchetypeFlags::ON_INSERT_OBSERVER)
    }

    /// Returns true if any of the components in this archetype have at least one [`Replace`] observer
    ///
    /// [`Replace`]: crate::lifecycle::Replace
    #[inline]
    pub fn has_replace_observer(&self) -> bool {
        self.flags().contains(ArchetypeFlags::ON_REPLACE_OBSERVER)
    }

    /// Returns true if any of the components in this archetype have at least one [`Remove`] observer
    ///
    /// [`Remove`]: crate::lifecycle::Remove
    #[inline]
    pub fn has_remove_observer(&self) -> bool {
        self.flags().contains(ArchetypeFlags::ON_REMOVE_OBSERVER)
    }

    /// Returns true if any of the components in this archetype have at least one [`Despawn`] observer
    ///
    /// [`Despawn`]: crate::lifecycle::Despawn
    #[inline]
    pub fn has_despawn_observer(&self) -> bool {
        self.flags().contains(ArchetypeFlags::ON_DESPAWN_OBSERVER)
    }
}
