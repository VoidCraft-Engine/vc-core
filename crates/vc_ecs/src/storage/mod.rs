// -----------------------------------------------------------------------------
// Modules

mod resource;
mod sparse;
mod table;
mod utils;

// -----------------------------------------------------------------------------
// Internal API

use utils::{AbortOnDrop, BlobArray, VecCopyRemove, VecSwapRemove};

// -----------------------------------------------------------------------------
// Exports

pub use resource::{ResourceData, Resources};
pub use sparse::SparseIndex;
pub use sparse::{FixedSparseMap, SparseMap};
pub use sparse::{SparseComponent, SparseSet, SparseSets};
pub use table::{Table, TableBuilder, TableId, TableMoveResult, TableRow, Tables};
pub use utils::Column;

// -----------------------------------------------------------------------------
// Inline-Exports

#[derive(Debug, Copy, Clone, Default, Eq, PartialEq)]
pub enum StorageType {
    #[default]
    Table,
    SparseSet,
}

pub struct Storages {
    pub sparse_sets: SparseSets,
    pub tables: Tables,
    pub resources: Resources<true>,
    pub non_send_resources: Resources<false>,
}
