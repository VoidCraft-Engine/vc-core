use core::fmt;

use vc_os::sync::atomic::AtomicU32;

use super::WorldId;
use crate::archetype::Archetypes;
use crate::component::{ComponentIdGenerator, Components};
use crate::entity::{Entities, EntityAllocator};
use crate::storage::Storages;
use crate::tick::Tick;

#[allow(unused)]
pub struct World {
    id: WorldId,
    archetypes: Archetypes,
    storages: Storages,
    entities: Entities,
    allocator: EntityAllocator,
    components: Components,
    generator: ComponentIdGenerator,
    change_tick: AtomicU32,
    last_check_tick: Tick,
    last_change_tick: Tick,
    // TODO
}

impl fmt::Debug for World {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        // SAFETY: `UnsafeWorldCell` requires that this must only access metadata.
        // Accessing any data stored in the world would be unsound.
        f.debug_struct("World")
            .field("id", &self.id)
            .field("entity_count", &self.entities.count_spawned())
            .field("archetype_count", &self.archetypes.len())
            .field("resource_count", &self.storages.resources.len())
            .finish()
    }
}
