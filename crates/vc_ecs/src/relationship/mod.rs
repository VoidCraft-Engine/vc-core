mod accessor;

pub use accessor::{ComponentRelationshipAccessor, RelationshipAccessor};

#[derive(Copy, Clone, Debug)]
pub enum RelationshipHookMode {
    Run,
    RunIfNotLinked,
    Skip,
}
