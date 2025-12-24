use crate::info::Typed;

/// Trait used to generate [`TypeTrait`] for trait reflection.
///
/// This is used by the `#[derive(Reflect)]` macro to generate an implementation
/// of [`TypeTrait`] to pass to [`TypeMeta::insert_trait`].
///
/// [`TypeTrait`]: crate::registry::TypeTrait
/// [`TypeMeta::insert_trait`]: crate::registry::TypeMeta::insert_trait
pub trait FromType<T: Typed> {
    fn from_type() -> Self;
}
