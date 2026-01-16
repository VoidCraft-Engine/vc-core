use super::ResourceData;
use crate::component::ComponentId;
use crate::storage::SparseSet;
use crate::tick::CheckTicks;

pub struct Resources<const SEND: bool> {
    resources: SparseSet<ComponentId, ResourceData<SEND>>,
}

impl<const SEND: bool> Resources<SEND> {
    #[inline]
    pub fn len(&self) -> usize {
        self.resources.len()
    }

    #[inline]
    pub fn is_empty(&self) -> bool {
        self.resources.is_empty()
    }

    #[inline]
    pub fn iter(&self) -> impl Iterator<Item = (ComponentId, &ResourceData<SEND>)> {
        self.resources.iter().map(|(id, data)| (*id, data))
    }

    #[inline]
    pub fn get(&self, component_id: ComponentId) -> Option<&ResourceData<SEND>> {
        self.resources.get(component_id)
    }

    #[inline]
    pub fn get_mut(&mut self, component_id: ComponentId) -> Option<&mut ResourceData<SEND>> {
        self.resources.get_mut(component_id)
    }

    #[inline]
    pub fn clear(&mut self) {
        self.resources.clear();
    }

    #[inline]
    pub fn check_ticks(&mut self, check: CheckTicks) {
        for info in self.resources.values_mut() {
            info.check_ticks(check);
        }
    }
}
