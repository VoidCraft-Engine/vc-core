//! Provide hash containers, re-exports *[hashbrown]* and *[foldhash]*.

// -----------------------------------------------------------------------------
// Modules

mod hasher;

pub mod hash_map;
pub mod hash_set;
pub mod hash_table;

mod pre_hashed;

// -----------------------------------------------------------------------------
// Exports

pub use hasher::{FixedHashState, FixedHasher};
pub use hasher::{NoOpHashState, NoOpHasher};
pub use hasher::{SparseHashState, SparseHasher};

pub use hash_map::{HashMap, NoOpHashMap, SparseHashMap};
pub use hash_set::{HashSet, NoOpHashSet, SparseHashSet};
pub use hash_table::HashTable;

pub use pre_hashed::{Hashed, PreHashMap};

// -----------------------------------------------------------------------------
// Re-export crates

pub use foldhash;
pub use hashbrown;
