// -----------------------------------------------------------------------------
// Modules

mod id;

mod allocator;
mod entities;
mod entity;
mod error;
mod location;

// -----------------------------------------------------------------------------
// Exports

pub use allocator::EntityAllocator;
pub use entities::Entities;
pub use entity::Entity;
pub use error::{InvalidEntityError, NotSpawnedError, SpawnError, ValidEntityButNotSpawnedError};
pub use id::{EntityGeneration, EntityId};
pub use location::EntityLocation;
