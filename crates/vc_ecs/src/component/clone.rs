#![expect(unsafe_code, reason = "read ptr is unsafe")]

use core::any::TypeId;

use vc_ptr::Ptr;
use vc_reflect::Reflect;
use vc_reflect::registry::{TypeRegistry, TypeTraitFromPtr};

use super::ComponentInfo;

// -----------------------------------------------------------------------------
// SourceComponent

/// Provides read access to the source component (the component being cloned) in a [`ComponentCloneFn`].
pub struct SourceComponent<'a> {
    ptr: Ptr<'a>,
    info: &'a ComponentInfo,
}

impl<'a> SourceComponent<'a> {
    pub fn ptr(&self) -> Ptr<'a> {
        self.ptr
    }

    pub fn read<C>(&self) -> Option<&C>
    where
        C: crate::component::Component,
    {
        let type_id = self.info.type_id()?;

        if type_id == TypeId::of::<C>() {
            self.ptr.debug_assert_aligned::<C>();
            unsafe { Some(self.ptr.as_ref::<C>()) }
        } else {
            None
        }
    }

    pub fn read_reflect(&self, registry: &TypeRegistry) -> Option<&dyn Reflect> {
        let type_id = self.info.type_id()?;
        let from_ptr = registry.get_type_trait::<TypeTraitFromPtr>(type_id)?;

        if type_id == from_ptr.ty_id() {
            unsafe { Some(from_ptr.as_reflect(self.ptr)) }
        } else {
            None
        }
    }
}

pub struct ComponentCloneCtx {
    // TODO
}

// -----------------------------------------------------------------------------
// ComponentCloneFn

/// Function type that can be used to clone a component of an entity.
pub type ComponentCloneFn = fn(&SourceComponent, &mut ComponentCloneCtx);

// -----------------------------------------------------------------------------
// ComponentCloneBehavior

#[derive(Clone, Debug, Default)]
pub enum ComponentCloneBehavior {
    #[default]
    Default,
    Ignore,
    Custom(ComponentCloneFn),
}
