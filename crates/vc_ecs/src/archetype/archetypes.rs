use core::ops::{Index, IndexMut};

use alloc::boxed::Box;
use alloc::vec::Vec;

use nonmax::NonMaxU32;
use vc_utils::hash::{HashMap, NoOpHashSet};

use super::{Archetype, ArchetypeId};
use crate::{component::ComponentId, storage::SparseMap};

#[derive(Hash, PartialEq, Eq)]
pub struct ArchetypeComponents {
    table_components: Box<[ComponentId]>,
    sparse_set_components: Box<[ComponentId]>,
}

pub struct Archetypes {
    pub archetypes: Vec<Archetype>,
    pub precise_map: HashMap<ArchetypeComponents, ArchetypeId>,
    pub rough_table: Vec<NoOpHashSet<ArchetypeId>>,
    pub rough_map: SparseMap<ComponentId, NonMaxU32>,
}

impl Archetypes {
    pub fn len(&self) -> usize {
        self.archetypes.len()
    }

    #[inline]
    pub fn get(&self, id: ArchetypeId) -> Option<&Archetype> {
        self.archetypes.get(id.index())
    }

    #[inline]
    pub fn get_2_mut(
        &mut self,
        a: ArchetypeId,
        b: ArchetypeId,
    ) -> (&mut Archetype, &mut Archetype) {
        if a.index() > b.index() {
            let (b_slice, a_slice) = self.archetypes.split_at_mut(a.index());
            (&mut a_slice[0], &mut b_slice[b.index()])
        } else {
            let (a_slice, b_slice) = self.archetypes.split_at_mut(b.index());
            (&mut a_slice[a.index()], &mut b_slice[0])
        }
    }

    #[inline]
    pub fn iter(&self) -> impl Iterator<Item = &Archetype> {
        self.archetypes.iter()
    }
}

impl Index<ArchetypeId> for Archetypes {
    type Output = Archetype;

    #[inline]
    fn index(&self, index: ArchetypeId) -> &Self::Output {
        &self.archetypes[index.index()]
    }
}

impl IndexMut<ArchetypeId> for Archetypes {
    #[inline]
    fn index_mut(&mut self, index: ArchetypeId) -> &mut Self::Output {
        &mut self.archetypes[index.index()]
    }
}
