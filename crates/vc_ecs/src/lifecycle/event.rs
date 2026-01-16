use vc_reflect::derive::Reflect;

use crate::component::ComponentId;
use crate::entity::Entity;
use crate::event::EventKey;

// -----------------------------------------------------------------------------
// EventKeys

pub const ADD: EventKey = EventKey(ComponentId::new(0));
pub const INSERT: EventKey = EventKey(ComponentId::new(1));
pub const REPLACE: EventKey = EventKey(ComponentId::new(2));
pub const REMOVE: EventKey = EventKey(ComponentId::new(3));
pub const DESPAWN: EventKey = EventKey(ComponentId::new(4));

// -----------------------------------------------------------------------------
// Event - Add

#[derive(Reflect, Debug, Clone)]
#[reflect(mini, debug)]
pub struct Add {
    pub entity: Entity,
}

// -----------------------------------------------------------------------------
// Event - Insert

#[derive(Reflect, Debug, Clone)]
#[reflect(mini, debug)]
pub struct Insert {
    pub entity: Entity,
}

// -----------------------------------------------------------------------------
// Event - Insert

#[derive(Reflect, Debug, Clone)]
#[reflect(mini, debug)]
pub struct Replace {
    pub entity: Entity,
}

// -----------------------------------------------------------------------------
// Event - Remove

#[derive(Reflect, Debug, Clone)]
#[reflect(mini, debug)]
pub struct Remove {
    pub entity: Entity,
}

// -----------------------------------------------------------------------------
// Event - Despawn

#[derive(Reflect, Debug, Clone)]
#[reflect(mini, debug)]
pub struct Despawn {
    pub entity: Entity,
}
