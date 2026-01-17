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
