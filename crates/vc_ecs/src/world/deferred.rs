use super::UnsafeWorldCell;

// -----------------------------------------------------------------------------
// DeferredWorld

pub struct DeferredWorld<'w> {
    _world: UnsafeWorldCell<'w>,
}
