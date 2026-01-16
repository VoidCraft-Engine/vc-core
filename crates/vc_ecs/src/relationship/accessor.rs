use alloc::boxed::Box;

use vc_ptr::Ptr;

use crate::entity::Entity;

#[derive(Debug, Clone, Copy)]
pub enum RelationshipAccessor {
    Relationship {
        entity_field_offset: usize,
        linked_spawn: bool,
    },
    RelationshipTarget {
        iter: for<'a> unsafe fn(Ptr<'a>) -> Box<dyn Iterator<Item = Entity> + 'a>,
        linked_spawn: bool,
    },
}
