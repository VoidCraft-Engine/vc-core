// -----------------------------------------------------------------------------
// Modules

mod deferred;
mod id;
mod world;
mod world_cell;

// -----------------------------------------------------------------------------
// Exports

pub use deferred::DeferredWorld;
pub use id::WorldId;
pub use world::World;
pub use world_cell::UnsafeWorldCell;
