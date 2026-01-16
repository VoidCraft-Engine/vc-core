use vc_os::sync::atomic::{AtomicU64, Ordering};

// -----------------------------------------------------------------------------
// WorldId

#[derive(Copy, Clone, PartialEq, Eq, Debug, Hash)]
pub struct WorldId(u64);

static WORLD_ID_GENERATOR: AtomicU64 = AtomicU64::new(0);

impl WorldId {
    pub fn new() -> Self {
        let lower = WORLD_ID_GENERATOR.fetch_add(1, Ordering::Relaxed);
        assert!(lower < u64::MAX);
        Self(lower)
    }
}
