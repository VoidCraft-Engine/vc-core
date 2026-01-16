// -----------------------------------------------------------------------------
// Modules

mod id;

mod borrow;
mod tick;

mod mutable;

mod clone;
mod info;

// -----------------------------------------------------------------------------
// Internal API

use crate::storage::StorageType;

pub(crate) use tick::{ComponentTicksMut, ComponentTicksRef};

// -----------------------------------------------------------------------------
// Exports

pub use id::ComponentId;

pub use borrow::{Mut, MutUntyped, Ref};
pub use borrow::{NonSend, NonSendMut, Res, ResMut};
pub use clone::{ComponentCloneBehavior, ComponentCloneFn, SourceComponent};
pub use info::{ComponentDescriptor, ComponentInfo};
pub use mutable::{ComponentMutability, Immutable, Mutable};
pub use tick::{ComponentTickCells, ComponentTicks};

// -----------------------------------------------------------------------------
// TODO

pub trait Component: Send + Sync + 'static {
    const STORAGE_TYPE: StorageType;
    type Mutability: ComponentMutability;

    // TODO
}
