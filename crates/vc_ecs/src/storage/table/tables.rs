use alloc::boxed::Box;
use alloc::vec::Vec;
use core::ops::{Index, IndexMut};

use vc_utils::hash::HashMap;

use super::{Table, TableId};

use crate::component::ComponentId;
use crate::storage::TableBuilder;
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
    pub fn new() -> Self {
        let mut tables: Vec<Table> = Vec::new();
        let mut table_ids: HashMap<Box<[ComponentId]>, TableId> = HashMap::new();
        tables.push(TableBuilder::new(0).build());
        table_ids.insert(Box::new([]), TableId::EMPTY);

        Tables { tables, table_ids }
    }

    #[inline]
    pub fn len(&self) -> usize {
        self.tables.len()
    }

    #[inline]
    pub fn get(&self, id: TableId) -> Option<&Table> {
        self.tables.get(id.index())
    }

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

    pub fn iter(&self) -> core::slice::Iter<'_, Table> {
        self.tables.iter()
    }

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
