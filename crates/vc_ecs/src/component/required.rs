use core::fmt;

use vc_os::sync::Arc;
use vc_utils::hash::FixedHashState;
use vc_utils::index::IndexMap;

use super::ComponentId;
use crate::component::ComponentsRegistrator;
use crate::entity::Entity;
use crate::storage::{SparseSets, Table, TableRow};
use crate::tick::Tick;
use crate::utils::DebugLocation;

#[derive(Clone)]
pub struct RequiredComponent {
    pub constructor:
        Arc<dyn Fn(&mut Table, &mut SparseSets, Tick, TableRow, Entity, DebugLocation)>,
}

impl fmt::Debug for RequiredComponent {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str("RequiredComponent")
    }
}

#[derive(Default, Clone)]
pub struct RequiredComponents {
    pub direct: IndexMap<ComponentId, RequiredComponent, FixedHashState>,
    pub all: IndexMap<ComponentId, RequiredComponent, FixedHashState>,
}

impl fmt::Debug for RequiredComponents {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("RequiredComponents")
            .field("direct", &self.direct.keys())
            .field("all", &self.all.keys())
            .finish()
    }
}

pub struct RequiredComponentsRegistrator<'a, 'w> {
    pub components: &'a mut ComponentsRegistrator<'w>,
    pub required_components: &'a mut RequiredComponents,
}
