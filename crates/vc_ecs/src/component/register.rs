use alloc::boxed::Box;
use alloc::vec::Vec;

use vc_utils::extra::TypeIdMap;

use super::{ComponentDescriptor, ComponentId, ComponentIdGenerator, Components};

pub struct ComponentsRegistrator<'w> {
    pub components: &'w mut Components,
    pub generator: &'w mut ComponentIdGenerator,
    pub recursion_check_stack: Vec<ComponentId>,
}

pub struct QueuedRegistration {
    pub registrator:
        Box<dyn FnOnce(&mut ComponentsRegistrator, ComponentIdGenerator, ComponentDescriptor)>,
    pub id: ComponentId,
    pub descriptor: ComponentDescriptor,
}

impl core::fmt::Debug for QueuedRegistration {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_struct("QueuedRegistration")
            .field("id", &self.id)
            .field("descriptor", &self.descriptor)
            .finish()
    }
}

#[derive(Default, Debug)]
pub struct QueuedComponents {
    pub components: TypeIdMap<QueuedRegistration>,
    pub resources: TypeIdMap<QueuedRegistration>,
    pub dynamic_registrations: Vec<QueuedRegistration>,
}
