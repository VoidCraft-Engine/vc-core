mod accessor;

pub use accessor::RelationshipAccessor;

#[derive(Copy, Clone, Debug)]
pub enum RelationshipHookMode {
    Run,
    RunIfNotLinked,
    Skip,
}
