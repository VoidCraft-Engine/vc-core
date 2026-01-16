use alloc::boxed::Box;
use alloc::vec::Vec;
use core::ops::{Index, IndexMut};

use vc_utils::hash::HashMap;

use super::{Table, TableId};

use crate::component::ComponentId;
use crate::tick::CheckTicks;

// -----------------------------------------------------------------------------
// Tables

#[allow(unused)]
pub struct Tables {
    tables: Vec<Table>,
    table_ids: HashMap<Box<[ComponentId]>, TableId>,
}

impl Tables {
    #[inline]
    pub const fn empty() -> Self {
        Tables {
            tables: Vec::new(),
            table_ids: HashMap::new(),
        }
    }

    /// Returns the number of [`Table`]s this collection contains
    #[inline]
    pub fn len(&self) -> usize {
        self.tables.len()
    }

    /// Returns true if this collection contains no [`Table`]s
    #[inline]
    pub fn is_empty(&self) -> bool {
        self.tables.is_empty()
    }

    /// Fetches a [`Table`] by its [`TableId`].
    ///
    /// Returns `None` if `id` is invalid.
    #[inline]
    pub fn get(&self, id: TableId) -> Option<&Table> {
        self.tables.get(id.index())
    }

    /// Fetches mutable references to two different [`Table`]s.
    ///
    /// # Panics
    /// Panics if `a` and `b` are equal.
    #[inline]
    pub fn get_2_mut(&mut self, a: TableId, b: TableId) -> (&mut Table, &mut Table) {
        if a.index() > b.index() {
            let (b_slice, a_slice) = self.tables.split_at_mut(a.index());
            (&mut a_slice[0], &mut b_slice[b.index()])
        } else {
            let (a_slice, b_slice) = self.tables.split_at_mut(b.index());
            (&mut a_slice[a.index()], &mut b_slice[0])
        }
    }

    /// Iterates through all of the tables stored within in [`TableId`] order.
    pub fn iter(&self) -> core::slice::Iter<'_, Table> {
        self.tables.iter()
    }

    /// Clears all data from all [`Table`]s stored within.
    pub fn clear(&mut self) {
        for table in &mut self.tables {
            table.clear();
        }
    }

    pub fn check_ticks(&mut self, check: CheckTicks) {
        for table in &mut self.tables {
            table.check_ticks(check);
        }
    }
}

impl Index<TableId> for Tables {
    type Output = Table;

    #[inline]
    fn index(&self, index: TableId) -> &Self::Output {
        &self.tables[index.index()]
    }
}

impl IndexMut<TableId> for Tables {
    #[inline]
    fn index_mut(&mut self, index: TableId) -> &mut Self::Output {
        &mut self.tables[index.index()]
    }
}
