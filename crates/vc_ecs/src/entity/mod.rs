// -----------------------------------------------------------------------------
// Modules

mod id;

mod allocator;
mod entities;
mod entity;
mod location;
mod utils;

pub mod error;

// -----------------------------------------------------------------------------
// Exports

pub use utils::*;

pub use allocator::EntityAllocator;
pub use entities::Entities;
pub use entity::Entity;
pub use id::{EntityGeneration, EntityId};
pub use location::EntityLocation;
