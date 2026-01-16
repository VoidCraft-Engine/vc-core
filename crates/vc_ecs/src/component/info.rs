use core::alloc::Layout;
use core::any::TypeId;

use vc_ptr::OwningPtr;

use super::clone::ComponentCloneBehavior;

use crate::relationship::RelationshipAccessor;
use crate::storage::StorageType;
use crate::utils::DebugName;

// -----------------------------------------------------------------------------
// ComponentDescriptor

#[derive(Clone)]
pub struct ComponentDescriptor {
    name: DebugName,
    storage_type: StorageType,
    is_send_and_sync: bool,
    type_id: Option<TypeId>,
    layout: Layout,
    drop: Option<for<'a> unsafe fn(OwningPtr<'a>)>,
    mutable: bool,
    clone_behavior: ComponentCloneBehavior,
    relationship_accessor: Option<RelationshipAccessor>,
}

impl core::fmt::Debug for ComponentDescriptor {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_struct("ComponentDescriptor")
            .field("name", &self.name)
            .field("storage_type", &self.storage_type)
            .field("is_send_and_sync", &self.is_send_and_sync)
            .field("type_id", &self.type_id)
            .field("layout", &self.layout)
            .field("drop", &self.drop)
            .field("mutable", &self.mutable)
            .field("clone_behavior", &self.clone_behavior)
            .field("relationship_accessor", &self.relationship_accessor)
            .finish()
    }
}

// -----------------------------------------------------------------------------
// ComponentInfo

#[derive(Clone)]
pub struct ComponentInfo {
    // TODO
}
