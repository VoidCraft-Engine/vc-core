// -----------------------------------------------------------------------------
// Modules

mod fixed;
mod noop;

// -----------------------------------------------------------------------------
// Re-Exports

use hashbrown::hash_map as hb;

pub use hb::{Drain, Entry, ExtractIf, IntoIter, Iter, IterMut};
pub use hb::{EntryRef, OccupiedEntry, OccupiedError, VacantEntry};
pub use hb::{IntoKeys, IntoValues, Keys, Values, ValuesMut};
pub use hb::{RawEntryBuilder, RawEntryBuilderMut, RawEntryMut, RawOccupiedEntryMut};

// -----------------------------------------------------------------------------
// Exports

pub use fixed::HashMap;
pub use noop::NoOpHashMap;
