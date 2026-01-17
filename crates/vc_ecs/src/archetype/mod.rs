// -----------------------------------------------------------------------------
// Modules

mod archetype;
mod archetypes;
mod bundle;
mod entity;
mod flags;
mod id;

// -----------------------------------------------------------------------------
// Exports

pub use archetype::{Archetype, ArchetypeSwapRemoveResult};
pub use archetypes::Archetypes;
pub use bundle::{ArchetypeInsertedBundle, Edges};
pub use entity::ArchetypeEntity;
pub use flags::ArchetypeFlags;
pub use id::{ArchetypeId, ArchetypeRow};
