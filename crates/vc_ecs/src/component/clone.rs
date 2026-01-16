use vc_ptr::Ptr;

use super::ComponentInfo;

// -----------------------------------------------------------------------------
// SourceComponent

/// Provides read access to the source component (the component being cloned) in a [`ComponentCloneFn`].
pub struct SourceComponent<'a> {
    _ptr: Ptr<'a>,
    _info: &'a ComponentInfo,
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
