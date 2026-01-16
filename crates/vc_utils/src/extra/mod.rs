//! Provide some extensions of `alloc::collections`.

// -----------------------------------------------------------------------------
// Modules

mod array_deque;
mod block_list;
mod short_name;
mod typeid_map;

// -----------------------------------------------------------------------------
// Exports

pub use array_deque::ArrayDeque;
pub use block_list::BlockList;
pub use short_name::ShortName;
pub use typeid_map::TypeIdMap;
