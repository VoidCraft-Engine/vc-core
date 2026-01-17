#![expect(unsafe_code)]

mod id;

pub use id::BundleId;

#[derive(Copy, Clone, PartialEq, Eq)]
pub enum ComponentStatus {
    Added,
    Existing,
}

pub trait BundleComponentStatus {
    unsafe fn get_status(&self, index: usize) -> ComponentStatus;
}

pub struct SpawnBundleStatus;

impl BundleComponentStatus for SpawnBundleStatus {
    #[inline(always)]
    unsafe fn get_status(&self, _index: usize) -> ComponentStatus {
        // Components inserted during a spawn call are always treated as added.
        ComponentStatus::Added
    }
}
