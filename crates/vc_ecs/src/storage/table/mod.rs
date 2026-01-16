// -----------------------------------------------------------------------------
// Modules

mod id;
mod table;
mod tables;

// -----------------------------------------------------------------------------
// Exports

pub use id::{TableId, TableRow};

pub use table::{Table, TableBuilder, TableMoveResult};
pub use tables::Tables;
