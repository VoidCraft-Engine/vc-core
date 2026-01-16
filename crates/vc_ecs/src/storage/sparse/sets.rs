use super::SparseSet;

use crate::component::ComponentId;
use crate::storage::SparseComponent;
use crate::tick::CheckTicks;

pub struct SparseSets {
    sets: SparseSet<ComponentId, SparseComponent>,
}

impl SparseSets {
    #[inline]
    pub const fn empty() -> Self {
        Self {
            sets: SparseSet::empty(),
        }
    }

    #[inline]
    pub fn len(&self) -> usize {
        self.sets.len()
    }

    #[inline]
    pub fn is_empty(&self) -> bool {
        self.sets.is_empty()
    }

    #[inline]
    pub fn iter(&self) -> impl Iterator<Item = (ComponentId, &SparseComponent)> {
        self.sets.iter().map(|(&id, data)| (id, data))
    }

    #[inline]
    pub fn get(&self, id: ComponentId) -> Option<&SparseComponent> {
        self.sets.get(id)
    }

    #[inline]
    pub fn get_mut(&mut self, id: ComponentId) -> Option<&mut SparseComponent> {
        self.sets.get_mut(id)
    }

    #[inline]
    pub fn clear_entities(&mut self) {
        for set in self.sets.values_mut() {
            set.clear();
        }
    }

    #[inline]
    pub fn check_ticks(&mut self, check: CheckTicks) {
        for set in self.sets.values_mut() {
            set.check_ticks(check);
        }
    }
}
