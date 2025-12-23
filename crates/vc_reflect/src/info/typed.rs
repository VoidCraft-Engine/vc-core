use crate::info::{TypeInfo, TypePath};

/// A static accessor to compile-time type information.
///
/// Automatically implemented by [`#[derive(Reflect)]`](crate::derive::Reflect),
/// allowing access to type information without an instance of the type.
pub trait Typed: TypePath {
    fn type_info() -> &'static TypeInfo;
}

/// Provides dynamic dispatch for types that implement [`Typed`].
pub trait DynamicTyped {
    /// See [`Typed::type_info`].
    fn reflect_type_info(&self) -> &'static TypeInfo;
}

impl<T: Typed> DynamicTyped for T {
    #[inline]
    fn reflect_type_info(&self) -> &'static TypeInfo {
        Self::type_info()
    }
}
