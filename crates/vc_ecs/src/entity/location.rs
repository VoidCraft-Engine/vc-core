use crate::archetype::{ArchetypeId, ArchetypeRow};
use crate::storage::{TableId, TableRow};

// -----------------------------------------------------------------------------
// EntityLocation

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct EntityLocation {
    pub archetype_id: ArchetypeId,
    pub archetype_row: ArchetypeRow,
    pub table_id: TableId,
    pub table_row: TableRow,
}
