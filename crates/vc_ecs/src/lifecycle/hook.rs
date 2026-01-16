use crate::component::ComponentId;
use crate::entity::Entity;
use crate::relationship::RelationshipHookMode;
use crate::utils::DebugLocation;
use crate::world::DeferredWorld;

// -----------------------------------------------------------------------------
// HookContext

#[derive(Clone, Copy, Debug)]
pub struct HookContext {
    pub entity: Entity,
    pub component_id: ComponentId,
    pub caller: DebugLocation,
    pub relationship_hook_mode: RelationshipHookMode,
}

// -----------------------------------------------------------------------------
// ComponentHook

pub type ComponentHook = for<'w> fn(DeferredWorld<'w>, HookContext);

// -----------------------------------------------------------------------------
// ComponentHooks

pub struct ComponentHooks {
    pub(crate) _on_add: Option<ComponentHook>,
    pub(crate) _on_insert: Option<ComponentHook>,
    pub(crate) _on_replace: Option<ComponentHook>,
    pub(crate) _on_remove: Option<ComponentHook>,
    pub(crate) _on_despawn: Option<ComponentHook>,
}
