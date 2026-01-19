// -----------------------------------------------------------------------------
// StorageType

#[derive(Debug, Copy, Clone, Default, Eq, PartialEq)]
pub enum StorageType {
    #[default]
    Table,
    SparseSet,
}

// -----------------------------------------------------------------------------
// StorageIndex

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
#[repr(align(4))]
pub struct StorageIndex {
    storage_type: StorageType,
    raw_index: u32,
}

impl StorageIndex {
    #[inline(always)]
    pub const fn new(storage_type: StorageType, raw_index: u32) -> StorageIndex {
        Self {
            storage_type,
            raw_index,
        }
    }

    #[inline(always)]
    pub const fn raw_index(self) -> u32 {
        self.raw_index
    }

    #[inline(always)]
    pub const fn storage_type(self) -> StorageType {
        self.storage_type
    }
}
