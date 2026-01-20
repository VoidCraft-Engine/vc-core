use alloc::boxed::Box;
use alloc::rc::Rc;
use vc_os::sync::Arc;

use crate::entity::Entity;

// -----------------------------------------------------------------------------
// Traits

pub trait ContainsEntity {
    /// Returns the contained entity.
    fn entity(&self) -> Entity;
}

pub unsafe trait EntityEquivalent: ContainsEntity + Eq {}

// -----------------------------------------------------------------------------
// Implementation

unsafe impl EntityEquivalent for Entity {}
impl ContainsEntity for Entity {
    fn entity(&self) -> Entity {
        *self
    }
}

unsafe impl<T: EntityEquivalent> EntityEquivalent for &T {}
impl<T: ContainsEntity> ContainsEntity for &T {
    fn entity(&self) -> Entity {
        (**self).entity()
    }
}

unsafe impl<T: EntityEquivalent> EntityEquivalent for &mut T {}
impl<T: ContainsEntity> ContainsEntity for &mut T {
    fn entity(&self) -> Entity {
        (**self).entity()
    }
}

unsafe impl<T: EntityEquivalent> EntityEquivalent for Box<T> {}
impl<T: ContainsEntity> ContainsEntity for Box<T> {
    fn entity(&self) -> Entity {
        (**self).entity()
    }
}

unsafe impl<T: EntityEquivalent> EntityEquivalent for Rc<T> {}
impl<T: ContainsEntity> ContainsEntity for Rc<T> {
    fn entity(&self) -> Entity {
        (**self).entity()
    }
}

unsafe impl<T: EntityEquivalent> EntityEquivalent for Arc<T> {}
impl<T: ContainsEntity> ContainsEntity for Arc<T> {
    fn entity(&self) -> Entity {
        (**self).entity()
    }
}
