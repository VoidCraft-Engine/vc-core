#![expect(unsafe_code, reason = "get_unchecked is unsafe")]

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

pub struct Tables {
    tables: Vec<Table>,
    table_ids: HashMap<Box<[ComponentId]>, TableId>,
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
    pub fn get_mut(&mut self, id: TableId) -> Option<&mut Table> {
        self.tables.get_mut(id.index())
    }

    #[inline(always)]
    pub fn get_unchecked(&self, id: TableId) -> &Table {
        unsafe { self.tables.get_unchecked(id.index()) }
    }

    #[inline(always)]
    pub fn get_mut_unchecked(&mut self, id: TableId) -> &mut Table {
        unsafe { self.tables.get_unchecked_mut(id.index()) }
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

    #[inline]
    pub fn iter(&self) -> core::slice::Iter<'_, Table> {
        self.tables.iter()
    }

    #[inline]
    pub fn iter_mut(&mut self) -> core::slice::IterMut<'_, Table> {
        self.tables.iter_mut()
    }

    #[inline]
    pub fn clear_entities(&mut self) {
        for table in &mut self.tables {
            table.dealloc();
        }
    }

    #[inline]
    pub fn check_ticks(&mut self, check: CheckTicks) {
        for table in &mut self.tables {
            table.check_ticks(check);
        }
    }
}

// -----------------------------------------------------------------------------
// Create Table From Components

use crate::component::Components;
use crate::utils::DebugCheckedUnwrap;

impl Tables {
    pub unsafe fn get_id_and_raw_indecies_or_insert(
        &mut self,
        ids: &[ComponentId],
        components: &Components,
    ) -> (TableId, Box<[u32]>) {
        use vc_utils::hash::hash_map::RawEntryMut;

        let tables = &mut self.tables;

        let raw_entry = self.table_ids.raw_entry_mut().from_key(ids);

        match raw_entry {
            RawEntryMut::Occupied(entry) => {
                let table_id = *entry.into_key_value().1;
                let table = &mut tables[table_id.index()];

                let mut vec = Vec::<u32>::with_capacity(ids.len());
                for &id in ids {
                    vec.push(unsafe { table.get_raw_index(id).debug_checked_unwrap() });
                }

                let boxed = vec.into_boxed_slice();
                (table_id, boxed)
            }
            RawEntryMut::Vacant(entry) => {
                assert!(tables.len() <= u32::MAX as usize, "too many tables");
                let table_id = TableId::new(tables.len() as u32);

                let mut table = TableBuilder::new(ids.len());

                let mut vec = Vec::<u32>::with_capacity(ids.len());

                for &id in ids {
                    let info = unsafe {
                        components
                            .infos
                            .get_unchecked(id.index())
                            .as_ref()
                            .debug_checked_unwrap()
                    };
                    vec.push(table.insert(id, info.layout(), info.drop_fn()));
                }
                tables.push(table.build());
                entry.insert(ids.into(), table_id);

                let boxed = vec.into_boxed_slice();

                (table_id, boxed)
            }
        }
    }
}
