use core::alloc::Layout;
use core::any::TypeId;

use vc_ptr::OwningPtr;
use vc_utils::hash::FixedHashState;
use vc_utils::index::IndexSet;

use super::{ComponentCloneBehavior, RequiredComponents};

use crate::component::ComponentId;
use crate::lifecycle::ComponentHooks;
use crate::relationship::RelationshipAccessor;
use crate::storage::StorageType;
use crate::utils::DebugName;

// -----------------------------------------------------------------------------
// ComponentDescriptor

#[derive(Clone)]
pub struct ComponentDescriptor {
    debug_name: DebugName,
    storage_type: StorageType,
    is_send_and_sync: bool,
    type_id: Option<TypeId>,
    layout: Layout,
    drop_fn: Option<for<'a> unsafe fn(OwningPtr<'a>)>,
    mutable: bool,
    clone_behavior: ComponentCloneBehavior,
    relationship_accessor: Option<RelationshipAccessor>,
}

impl core::fmt::Debug for ComponentDescriptor {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_struct("ComponentDescriptor")
            .field("debug_name", &self.debug_name)
            .field("storage_type", &self.storage_type)
            .field("is_send_and_sync", &self.is_send_and_sync)
            .field("type_id", &self.type_id)
            .field("layout", &self.layout)
            .field("drop_fn", &self.drop_fn)
            .field("mutable", &self.mutable)
            .field("clone_behavior", &self.clone_behavior)
            .field("relationship_accessor", &self.relationship_accessor)
            .finish()
    }
}

// -----------------------------------------------------------------------------
// ComponentInfo

#[derive(Debug, Clone)]
pub struct ComponentInfo {
    pub id: ComponentId,
    pub descriptor: ComponentDescriptor,
    pub hooks: ComponentHooks,
    pub required_components: RequiredComponents,
    pub required_by: IndexSet<ComponentId, FixedHashState>,
    // TODO
}

impl ComponentInfo {
    pub const fn id(&self) -> ComponentId {
        self.id
    }

    pub const fn index(&self) -> usize {
        self.id.index()
    }

    pub const fn storage_type(&self) -> StorageType {
        self.descriptor.storage_type
    }

    pub const fn layout(&self) -> Layout {
        self.descriptor.layout
    }

    pub const fn drop_fn(&self) -> Option<for<'a> unsafe fn(OwningPtr<'a>)> {
        self.descriptor.drop_fn
    }

    pub const fn is_send_and_sync(&self) -> bool {
        self.descriptor.is_send_and_sync
    }

    pub const fn type_id(&self) -> Option<TypeId> {
        self.descriptor.type_id
    }

    pub fn debug_name(&self) -> DebugName {
        self.descriptor.debug_name.clone()
    }
}
