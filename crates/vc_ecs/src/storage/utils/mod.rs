// -----------------------------------------------------------------------------
// Modules

mod abort_on_drop;

mod blob_array;
mod column;

mod thin_array;
mod vec_extension;

// -----------------------------------------------------------------------------
// Internal API

use thin_array::ThinArray;

pub(crate) use blob_array::BlobArray;

pub use column::Column;

pub(crate) use abort_on_drop::AbortOnDrop;

pub(crate) use vec_extension::{VecCopyRemove, VecSwapRemove};
