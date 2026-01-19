// -----------------------------------------------------------------------------
// Modules

mod index;
mod resource;
mod sparse;
mod table;
mod utils;

// -----------------------------------------------------------------------------
// Internal API

use utils::{AbortOnDrop, BlobArray, VecCopyRemove, VecSwapRemove};

// -----------------------------------------------------------------------------
// Exports

pub use index::{StorageIndex, StorageType};
pub use resource::{NoSendResourceData, NoSendResources, ResourceData, Resources};
pub use sparse::SparseIndex;
pub use sparse::{FixedSparseMap, SparseMap};
pub use sparse::{SparseComponent, SparseSet, SparseSets};
pub use table::{Table, TableBuilder, TableId, TableMoveResult, TableRow, Tables};
pub use utils::Column;

// -----------------------------------------------------------------------------
// Inline-Exports

pub struct Storages {
    pub sparse_sets: SparseSets,
    pub tables: Tables,
    pub resources: Resources,
    pub non_send_resources: NoSendResources,
}

impl Storages {
    pub fn prepare_component(&mut self, component: &crate::component::ComponentInfo) {
        match component.storage_type() {
            StorageType::Table => {
                // table needs no preparation
            }
            StorageType::SparseSet => {
                self.sparse_sets.prepare_component(component);
            }
        }
    }
}
