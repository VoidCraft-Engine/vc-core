#![expect(unsafe_code, reason = "original implementation requires unsafe code.")]

use alloc::boxed::Box;
use alloc::vec::Vec;
use core::alloc::Layout;
use core::cell::UnsafeCell;
use core::num::NonZeroUsize;
use core::panic::Location;

use nonmax::{NonMaxU8, NonMaxU32};
use vc_ptr::{OwningPtr, Ptr};

use super::TableRow;
use crate::cfg;
use crate::component::{ComponentId, ComponentTicks};
use crate::entity::Entity;
use crate::storage::utils::{AbortOnDrop, VecSwapRemove};
use crate::storage::{Column, FixedSparseMap, SparseMap};
use crate::tick::CheckTicks;
use crate::tick::Tick;
use crate::utils::{DebugCheckedUnwrap, DebugLocation};

// -----------------------------------------------------------------------------
// TableMoveResult

pub struct TableMoveResult {
    pub swapped_entity: Option<Entity>,
    pub new_row: TableRow,
}

// -----------------------------------------------------------------------------
// TableBuilder
pub struct TableBuilder {
    columns: Vec<Column>,
    indices: Vec<ComponentId>,
    sparse: SparseMap<ComponentId, NonMaxU8>,
    entities: Vec<Entity>,
}

impl TableBuilder {
    pub fn new(column_count: usize) -> Self {
        Self {
            columns: Vec::with_capacity(column_count),
            indices: Vec::with_capacity(column_count),
            sparse: SparseMap::empty(),
            entities: Vec::new(),
        }
    }

    #[must_use]
    pub fn add_column(
        mut self,
        id: ComponentId,
        item_layout: Layout,
        drop_fn: Option<unsafe fn(OwningPtr<'_>)>,
    ) -> Self {
        let col = Column::empty(item_layout, drop_fn);

        if let Some(column_index) = self.sparse.get_copied(id) {
            // SAFETY: dense indices stored in self.sparse always exist
            unsafe {
                *self.columns.get_unchecked_mut(column_index.get() as usize) = col;
            }
        } else {
            assert!(
                self.columns.len() < u8::MAX as usize,
                "Component number in a Entity storaged in Table cannot exceed `254`"
            );

            self.sparse.insert(id, unsafe {
                NonMaxU8::new_unchecked(self.columns.len() as u8)
            });
            self.indices.push(id);
            self.columns.push(col);
        }

        self
    }

    #[must_use]
    #[inline]
    pub fn build(self) -> Table {
        Table {
            columns: self.columns.into_boxed_slice(),
            indices: self.indices.into_boxed_slice(),
            sparse: self.sparse.into_fixed(),
            entities: self.entities,
        }
    }
}

// -----------------------------------------------------------------------------
// Table

pub struct Table {
    columns: Box<[Column]>,
    indices: Box<[ComponentId]>,
    sparse: FixedSparseMap<ComponentId, NonMaxU8>,
    entities: Vec<Entity>,
}

impl Drop for Table {
    fn drop(&mut self) {
        let len = self.entity_count();
        let current_capacity = self.capacity();
        self.entities.clear();
        for column in &mut self.columns {
            unsafe {
                column.dealloc(current_capacity, len);
            }
        }
    }
}

impl Table {
    #[inline]
    pub fn get_column(&self, id: ComponentId) -> Option<&Column> {
        self.sparse
            .get_copied(id)
            .map(|index| unsafe { self.columns.get_unchecked(index.get() as usize) })
    }

    #[inline]
    pub fn get_column_mut(&mut self, id: ComponentId) -> Option<&mut Column> {
        self.sparse
            .get_copied(id)
            .map(|index| unsafe { self.columns.get_unchecked_mut(index.get() as usize) })
    }

    #[inline(always)]
    pub fn entities(&self) -> &[Entity] {
        &self.entities
    }

    #[inline(always)]
    pub fn capacity(&self) -> usize {
        self.entities.capacity()
    }

    #[inline(always)]
    pub fn entity_count(&self) -> usize {
        self.entities.len()
    }

    #[inline(always)]
    pub fn component_count(&self) -> usize {
        self.columns.len()
    }

    #[inline(always)]
    pub fn is_empty(&self) -> bool {
        self.entities.is_empty()
    }

    #[inline]
    pub fn has_component(&self, component_id: ComponentId) -> bool {
        self.sparse.contains(component_id)
    }

    pub unsafe fn get_component(&self, id: ComponentId, row: TableRow) -> Option<Ptr<'_>> {
        cfg::debug! { assert!(row.index() < self.entity_count()); }

        self.get_column(id)
            .map(|col| unsafe { col.get_data(row.index()) })
    }

    pub unsafe fn take_component_unchecked(
        &mut self,
        component_id: ComponentId,
        row: TableRow,
    ) -> OwningPtr<'_> {
        cfg::debug! { assert!(row.index() < self.entity_count()); }

        unsafe {
            self.get_column_mut(component_id)
                .debug_checked_unwrap()
                .get_data_mut(row.index())
                .promote()
        }
    }

    pub fn clear(&mut self) {
        let len = self.entity_count();

        self.entities.clear();

        for column in &mut self.columns {
            unsafe {
                column.clear(len);
            }
        }
    }

    pub fn check_ticks(&mut self, check: CheckTicks) {
        let len = self.entity_count();

        for column in &mut self.columns {
            unsafe {
                column.check_ticks(len, check);
            }
        }
    }

    pub unsafe fn swap_remove(&mut self, row: TableRow) -> Option<Entity> {
        use crate::storage::VecCopyRemove;

        let removal_index = row.index();
        let last_index = self.entity_count() - 1;

        cfg::debug! { assert!(removal_index <= last_index); }

        unsafe {
            if removal_index != last_index {
                let entity = self
                    .entities
                    .copy_last_and_return_nonoverlapping(removal_index, last_index);

                for column in &mut self.columns {
                    column.swap_remove_and_drop_nonoverlapping(removal_index, last_index);
                }
                Some(entity)
            } else {
                self.entities.set_len(last_index);
                for column in &mut self.columns {
                    column.drop_last(last_index);
                }
                None
            }
        }
    }

    #[inline]
    unsafe fn alloc_columns(&mut self, new_capacity: NonZeroUsize) {
        let _abort_guard = AbortOnDrop;

        for col in &mut self.columns {
            unsafe {
                col.alloc(new_capacity);
            }
        }
        ::core::mem::forget(_abort_guard);
    }

    #[inline]
    unsafe fn realloc_columns(
        &mut self,
        current_capacity: NonZeroUsize,
        new_capacity: NonZeroUsize,
    ) {
        let _abort_guard = AbortOnDrop;

        for col in &mut self.columns {
            unsafe {
                col.realloc(current_capacity, new_capacity);
            }
        }
        ::core::mem::forget(_abort_guard);
    }

    #[cold]
    #[inline(never)]
    unsafe fn reserve_unchecked(self: &mut Table, additional: usize) {
        let old_capacity = self.capacity();
        self.entities.reserve(additional);
        let new_capacity = self.entities.capacity();

        if old_capacity == 0 {
            unsafe { self.alloc_columns(NonZeroUsize::new_unchecked(new_capacity)) };
        } else {
            unsafe {
                self.realloc_columns(
                    NonZeroUsize::new_unchecked(old_capacity),
                    NonZeroUsize::new_unchecked(new_capacity),
                );
            };
        }
    }

    #[inline(always)]
    fn reserve_one(&mut self) {
        if self.capacity() == self.entity_count() {
            unsafe {
                self.reserve_unchecked(1);
            }
        }
    }

    pub unsafe fn allocate(&mut self, entity: Entity) -> TableRow {
        let len = self.entity_count();

        cfg::debug! { assert!(len < u32::MAX as usize); }

        self.reserve_one();

        self.entities.push(entity);
        for col in &mut self.columns {
            unsafe {
                col.reset_item(len);
            }
        }

        unsafe { TableRow::new(NonMaxU32::new_unchecked(len as u32)) }
    }

    pub unsafe fn move_to_and_forget_missing(
        &mut self,
        row: TableRow,
        other: &mut Table,
    ) -> TableMoveResult {
        let src_index = row.index();
        let last_index = self.entity_count() - 1;

        cfg::debug! { assert!(src_index < self.entity_count()); }

        if src_index != last_index {
            unsafe {
                let dst_row = other.allocate(
                    self.entities
                        .swap_remove_nonoverlapping(src_index, last_index),
                );
                let dst_index = dst_row.index();

                for (id, column) in self.indices.iter().zip(self.columns.iter_mut()) {
                    if let Some(other_col) = other.get_column_mut(*id) {
                        other_col.init_item_from_nonoverlapping(
                            column, last_index, src_index, dst_index,
                        );
                    } else {
                        let _ = column.swap_remove_nonoverlapping(src_index, last_index);
                    }
                }

                TableMoveResult {
                    new_row: dst_row,
                    swapped_entity: Some(*self.entities.get_unchecked(src_index)),
                }
            }
        } else {
            unsafe {
                let dst_row = other.allocate(self.entities.remove_last(last_index));
                let dst_index = dst_row.index();

                for (id, column) in self.indices.iter().zip(self.columns.iter_mut()) {
                    if let Some(other_col) = other.get_column_mut(*id) {
                        other_col.init_last_item_from(column, last_index, dst_index);
                    } else {
                        let _ = column.remove_last(last_index);
                    }
                }

                TableMoveResult {
                    new_row: dst_row,
                    swapped_entity: None,
                }
            }
        }
    }

    pub unsafe fn move_to_and_drop_missing(
        &mut self,
        row: TableRow,
        other: &mut Table,
    ) -> TableMoveResult {
        let src_index = row.index();
        let last_index = self.entity_count() - 1;

        cfg::debug! { assert!(src_index < self.entity_count()); }

        if src_index != last_index {
            unsafe {
                let dst_row = other.allocate(
                    self.entities
                        .swap_remove_nonoverlapping(src_index, last_index),
                );
                let dst_index = dst_row.index();

                for (id, column) in self.indices.iter().zip(self.columns.iter_mut()) {
                    if let Some(other_col) = other.get_column_mut(*id) {
                        other_col.init_item_from_nonoverlapping(
                            column, last_index, src_index, dst_index,
                        );
                    } else {
                        column.swap_remove_and_drop_nonoverlapping(src_index, last_index);
                    }
                }

                TableMoveResult {
                    new_row: dst_row,
                    swapped_entity: Some(*self.entities.get_unchecked(src_index)),
                }
            }
        } else {
            unsafe {
                let dst_row = other.allocate(self.entities.remove_last(last_index));
                let dst_index = dst_row.index();

                for (id, column) in self.indices.iter().zip(self.columns.iter_mut()) {
                    if let Some(other_col) = other.get_column_mut(*id) {
                        other_col.init_last_item_from(column, last_index, dst_index);
                    } else {
                        column.drop_last(last_index);
                    }
                }

                TableMoveResult {
                    new_row: dst_row,
                    swapped_entity: None,
                }
            }
        }
    }

    pub unsafe fn move_to_superset(&mut self, row: TableRow, other: &mut Table) -> TableMoveResult {
        let src_index = row.index();
        let last_index = self.entity_count() - 1;

        cfg::debug! { assert!(src_index < self.entity_count()); }

        if src_index != last_index {
            unsafe {
                let dst_row = other.allocate(
                    self.entities
                        .swap_remove_nonoverlapping(src_index, last_index),
                );
                let dst_index = dst_row.index();

                for (id, column) in self.indices.iter().zip(self.columns.iter_mut()) {
                    let other_col = other.get_column_mut(*id).debug_checked_unwrap();
                    other_col
                        .init_item_from_nonoverlapping(column, last_index, src_index, dst_index);
                }

                TableMoveResult {
                    new_row: dst_row,
                    swapped_entity: Some(*self.entities.get_unchecked(src_index)),
                }
            }
        } else {
            unsafe {
                let dst_row = other.allocate(self.entities.remove_last(last_index));
                let dst_index = dst_row.index();

                for (id, column) in self.indices.iter().zip(self.columns.iter_mut()) {
                    let other_col = other.get_column_mut(*id).debug_checked_unwrap();
                    other_col.init_last_item_from(column, last_index, dst_index);
                }

                TableMoveResult {
                    new_row: dst_row,
                    swapped_entity: None,
                }
            }
        }
    }

    pub fn get_drop_fn_for(&self, component_id: ComponentId) -> Option<unsafe fn(OwningPtr<'_>)> {
        self.get_column(component_id)?.get_drop_fn()
    }

    pub fn get_data_slice_for<T>(&self, component_id: ComponentId) -> Option<&[UnsafeCell<T>]> {
        self.get_column(component_id)
            .map(|col| unsafe { col.get_data_slice(self.entity_count()) })
    }

    pub fn get_added_ticks_slice_for(
        &self,
        component_id: ComponentId,
    ) -> Option<&[UnsafeCell<Tick>]> {
        self.get_column(component_id)
            .map(|col| unsafe { col.get_added_ticks_slice(self.entity_count()) })
    }

    pub fn get_changed_ticks_slice_for(
        &self,
        component_id: ComponentId,
    ) -> Option<&[UnsafeCell<Tick>]> {
        self.get_column(component_id)
            .map(|col| unsafe { col.get_changed_ticks_slice(self.entity_count()) })
    }

    pub fn get_changed_by_slice_for(
        &self,
        _component_id: ComponentId,
    ) -> DebugLocation<Option<&[UnsafeCell<&'static Location<'static>>]>> {
        cfg::debug! {
            if {
                DebugLocation::untranspose(|| {
                    self.get_column(_component_id)
                        .map(|col| unsafe { col.get_changed_by_slice(self.entity_count()) })
                })
            } else {
                DebugLocation::new_with(|| None)
            }
        }
    }

    pub fn get_changed_tick(
        &self,
        component_id: ComponentId,
        row: TableRow,
    ) -> Option<&UnsafeCell<Tick>> {
        let index = row.index();
        if index < self.entity_count() {
            let ret = unsafe { self.get_column(component_id)?.get_changed_tick(index) };
            Some(ret)
        } else {
            None
        }
    }

    pub fn get_added_tick(
        &self,
        component_id: ComponentId,
        row: TableRow,
    ) -> Option<&UnsafeCell<Tick>> {
        let index = row.index();
        if index < self.entity_count() {
            let ret = unsafe { self.get_column(component_id)?.get_added_tick(index) };
            Some(ret)
        } else {
            None
        }
    }

    pub fn get_changed_by(
        &self,
        component_id: ComponentId,
        row: TableRow,
    ) -> DebugLocation<Option<&UnsafeCell<&'static Location<'static>>>> {
        cfg::debug! {
            if {
                DebugLocation::untranspose(|| {
                    let index = row.index();
                    if index < self.entity_count() {
                        let ret = unsafe {
                            self.get_column(component_id)?
                                .get_changed_by(index)
                        };
                        Some(ret)
                    } else {
                        None
                    }
                })
            } else {
                DebugLocation::new_with(|| None)
            }
        }
    }

    pub unsafe fn get_component_ticks(
        &self,
        component_id: ComponentId,
        row: TableRow,
    ) -> Option<ComponentTicks> {
        cfg::debug! { assert!(row.index() < self.entity_count()); }
        unsafe {
            self.get_column(component_id)
                .map(|col| col.get_component_ticks(row.index()))
        }
    }
}
