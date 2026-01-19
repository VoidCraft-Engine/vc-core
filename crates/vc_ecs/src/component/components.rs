use alloc::vec::Vec;

use vc_os::sync::RwLock;
use vc_utils::extra::TypeIdMap;

use super::{ComponentId, ComponentInfo, QueuedComponents};

#[derive(Debug, Default)]
pub struct Components {
    pub infos: Vec<Option<ComponentInfo>>,
    pub indices: TypeIdMap<ComponentId>,
    pub resource_indices: TypeIdMap<ComponentId>,
    // This is kept internal and local to verify that no deadlocks can occur.
    pub queued: RwLock<QueuedComponents>,
}
