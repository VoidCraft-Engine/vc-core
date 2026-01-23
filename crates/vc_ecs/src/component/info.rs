use core::alloc::Layout;
use core::any::TypeId;

use vc_ptr::OwningPtr;
use vc_utils::index::SparseIndexSet;

use super::{ComponentCloneBehavior, RequiredComponents};

use crate::archetype::ArchetypeFlags;
use crate::component::ComponentId;
use crate::lifecycle::ComponentHooks;
use crate::relationship::RelationshipAccessor;
use crate::storage::StorageType;
use crate::utils::DebugName;

// -----------------------------------------------------------------------------
// ComponentDescriptor

#[derive(Debug, Clone)]
pub struct ComponentDescriptor {
    debug_name: DebugName,
    storage_type: StorageType,
    is_send_and_sync: bool,
    type_id: Option<TypeId>,
    layout: Layout,
    mutable: bool,
    drop_fn: Option<for<'a> unsafe fn(OwningPtr<'a>)>,
    clone_behavior: ComponentCloneBehavior,
    relationship_accessor: Option<RelationshipAccessor>,
}

impl ComponentDescriptor {
    /// Returns a value indicating the storage strategy for the current component.
    #[inline(always)]
    pub fn storage_type(&self) -> StorageType {
        self.storage_type
    }

    /// Returns the [`TypeId`] of the underlying component type.
    /// Returns `None` if the component does not correspond to a Rust type.
    #[inline(always)]
    pub fn type_id(&self) -> Option<TypeId> {
        self.type_id
    }

    /// Returns the name of the current component.
    #[inline(always)]
    pub fn debug_name(&self) -> &DebugName {
        &self.debug_name
    }

    /// Returns whether this component is mutable.
    #[inline(always)]
    pub fn mutable(&self) -> bool {
        self.mutable
    }
}

// -----------------------------------------------------------------------------
// ComponentInfo

#[derive(Debug, Clone)]
pub struct ComponentInfo {
    pub(super) id: ComponentId,
    pub(super) descriptor: ComponentDescriptor,
    pub(super) hooks: ComponentHooks,
    pub(super) required_components: RequiredComponents,
    /// Invariant: components in this set always appear
    /// after the components that they require.
    pub(super) required_by: SparseIndexSet<ComponentId>,
}

impl ComponentInfo {
    #[inline(always)]
    pub const fn id(&self) -> ComponentId {
        self.id
    }

    #[inline(always)]
    pub const fn index(&self) -> usize {
        self.id.index()
    }

    #[inline(always)]
    pub const fn storage_type(&self) -> StorageType {
        self.descriptor.storage_type
    }

    #[inline(always)]
    pub const fn layout(&self) -> Layout {
        self.descriptor.layout
    }

    #[inline(always)]
    pub const fn drop_fn(&self) -> Option<for<'a> unsafe fn(OwningPtr<'a>)> {
        self.descriptor.drop_fn
    }

    #[inline(always)]
    pub const fn is_send_and_sync(&self) -> bool {
        self.descriptor.is_send_and_sync
    }

    #[inline(always)]
    pub const fn type_id(&self) -> Option<TypeId> {
        self.descriptor.type_id
    }

    #[inline(always)]
    pub const fn mutable(&self) -> bool {
        self.descriptor.mutable
    }

    #[inline(always)]
    pub const fn clone_behavior(&self) -> &ComponentCloneBehavior {
        &self.descriptor.clone_behavior
    }

    #[inline(always)]
    pub const fn debug_name(&self) -> &DebugName {
        &self.descriptor.debug_name
    }

    #[inline(always)]
    pub const fn hooks(&self) -> &ComponentHooks {
        &self.hooks
    }

    #[inline(always)]
    pub const fn required_components(&self) -> &RequiredComponents {
        &self.required_components
    }

    #[inline(always)]
    pub const fn required_by(&self) -> &SparseIndexSet<ComponentId> {
        &self.required_by
    }

    #[inline(always)]
    pub const fn relationship_accessor(&self) -> Option<&RelationshipAccessor> {
        self.descriptor.relationship_accessor.as_ref()
    }
}

// -----------------------------------------------------------------------------
// Extra implementation for ComponentInfo

impl ComponentInfo {
    pub const fn new(id: ComponentId, descriptor: ComponentDescriptor) -> Self {
        ComponentInfo {
            id,
            descriptor,
            hooks: ComponentHooks::empty(),
            required_components: RequiredComponents::empty(),
            required_by: SparseIndexSet::new(),
        }
    }

    pub fn update_archetype_flags(&self, flags: &mut ArchetypeFlags) {
        if self.hooks.on_add.is_some() {
            flags.insert(ArchetypeFlags::ON_ADD_HOOK);
        }
        if self.hooks.on_insert.is_some() {
            flags.insert(ArchetypeFlags::ON_INSERT_HOOK);
        }
        if self.hooks.on_replace.is_some() {
            flags.insert(ArchetypeFlags::ON_REPLACE_HOOK);
        }
        if self.hooks.on_remove.is_some() {
            flags.insert(ArchetypeFlags::ON_REMOVE_HOOK);
        }
        if self.hooks.on_despawn.is_some() {
            flags.insert(ArchetypeFlags::ON_DESPAWN_HOOK);
        }
    }
}

// -----------------------------------------------------------------------------
// Extra implementation for ComponentDescriptor

use super::{Component, ComponentMutability};
use crate::resource::Resource;

/// # Safety
/// As same as `OwingPtr::drop_as`
unsafe fn drop_owning<T>(x: OwningPtr<'_>) {
    // `OwningPtr::drop_as` cannot convert to `unsafe fn(OwingPtr<'_>)` directly.
    unsafe {
        x.drop_as::<T>();
    }
}

const fn get_drop_fn<T>() -> Option<unsafe fn(OwningPtr<'_>)> {
    if core::mem::needs_drop::<T>() {
        Some(drop_owning::<T>)
    } else {
        None
    }
}

impl ComponentDescriptor {
    #[inline]
    pub fn new_component<T: Component>() -> Self {
        Self {
            type_id: Some(TypeId::of::<T>()),
            layout: Layout::new::<T>(),
            drop_fn: get_drop_fn::<T>(),
            mutable: T::Mutability::MUTABLE,
            is_send_and_sync: true,
            storage_type: T::STORAGE_TYPE,
            debug_name: DebugName::type_name::<T>(),
            clone_behavior: T::clone_behavior(),
            relationship_accessor: T::relationship_accessor(),
        }
    }

    #[inline]
    pub fn new_resource<T: Resource>() -> Self {
        Self {
            type_id: Some(TypeId::of::<T>()),
            layout: Layout::new::<T>(),
            drop_fn: get_drop_fn::<T>(),
            mutable: true,
            is_send_and_sync: true,
            // This field has no effect for `Resource` types,
            // as they are always stored in `Resources` rather
            // than in `Tables` or `SparseSets`.
            storage_type: StorageType::SparseSet,
            debug_name: DebugName::type_name::<T>(),
            clone_behavior: ComponentCloneBehavior::Default,
            relationship_accessor: None,
        }
    }

    #[inline]
    pub fn new_non_send<T: core::any::Any>() -> Self {
        Self {
            type_id: Some(TypeId::of::<T>()),
            layout: Layout::new::<T>(),
            drop_fn: get_drop_fn::<T>(),
            mutable: true,
            is_send_and_sync: false,
            storage_type: StorageType::Table,
            debug_name: DebugName::type_name::<T>(),
            clone_behavior: ComponentCloneBehavior::Default,
            relationship_accessor: None,
        }
    }

    #[inline]
    pub unsafe fn new_dynamic(
        debug_name: DebugName,
        storage_type: StorageType,
        layout: Layout,
        drop_fn: Option<for<'a> unsafe fn(OwningPtr<'a>)>,
        mutable: bool,
        clone_behavior: ComponentCloneBehavior,
        relationship_accessor: Option<RelationshipAccessor>,
    ) -> Self {
        Self {
            debug_name,
            storage_type,
            is_send_and_sync: true,
            type_id: None,
            layout,
            drop_fn,
            mutable,
            clone_behavior,
            relationship_accessor,
        }
    }
}
