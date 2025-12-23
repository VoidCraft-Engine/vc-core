use crate::info::Typed;

pub trait FromType<T: Typed> {
    fn from_type() -> Self;
}
