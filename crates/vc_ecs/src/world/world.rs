use core::fmt;

use vc_os::sync::atomic::AtomicU32;

use super::WorldId;
use crate::archetype::Archetypes;
use crate::component::{ComponentIdGenerator, Components};
use crate::entity::{Entities, EntityAllocator};
use crate::storage::Storages;
use crate::tick::Tick;

#[allow(unused, reason = "todo")]
pub struct World {
    id: WorldId,
    pub(crate) archetypes: Archetypes,
    pub(crate) storages: Storages,
    pub(crate) entities: Entities,
    pub(crate) allocator: EntityAllocator,
    pub(crate) components: Components,
    pub(crate) generator: ComponentIdGenerator,
    pub(crate) change_tick: AtomicU32,
    pub(crate) last_check_tick: Tick,
    pub(crate) last_change_tick: Tick,
    // TODO
}

impl fmt::Debug for World {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.debug_struct("World")
            .field("id", &self.id)
            .field("entity_count", &self.entities.count_spawned())
            .field("archetype_count", &self.archetypes.len())
            .field("component_count", &self.generator.component_count())
            .field("resource_count", &self.storages.resources.len())
            .field(
                "no_send_resource_count",
                &self.storages.non_send_resources.len(),
            )
            .finish()
    }
}
