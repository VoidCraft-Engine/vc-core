#![expect(unsafe_code, reason = "EntityEquivalent is unsafe")]

// -----------------------------------------------------------------------------
// Modules

mod equivalent;

mod hash;

mod map_entities;

mod entity_set;

pub mod unique_array;
pub mod unique_iter;
pub mod unique_slice;
pub mod unique_vec;

// -----------------------------------------------------------------------------
// Exports

pub use entity_set::{EntitySet, EntitySetIterator, FromEntitySet};
pub use equivalent::{ContainsEntity, EntityEquivalent};

pub use hash::{EntityHashMap, EntityHashSet};
pub use hash::{EntityIndexMap, EntityIndexSet};

pub use map_entities::{EntityMapper, MapEntities, SceneEntityMapper};

pub use unique_array::{UniqueEntityArray, UniqueEntityEquivalentArray};
pub use unique_iter::UniqueEntityIter;
pub use unique_slice::{UniqueEntityEquivalentSlice, UniqueEntitySlice};
pub use unique_vec::{UniqueEntityEquivalentVec, UniqueEntityVec};
