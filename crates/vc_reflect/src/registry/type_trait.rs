use core::any::{Any, TypeId};

use alloc::boxed::Box;

use crate::{info::DynamicTypePath, registry::TypeRegistry};

/// A trait used to type-erase type metadata.
///
/// `TypeTrait` can be registered to the [`TypeRegistry`] and stored on a type's [`TypeTraits`].
///
/// While type trait is often generated using the [`#[reflect_trait]`](crate::derive::reflect_trait) macro,
/// almost any type that implements [`Clone`] can be considered "type trait".
/// This is because it has a blanket implementation over all `T` where `T: Clone + Send + Sync + 'static`.
///
/// See the [crate-level documentation] for more information on type_trait and type_traits.
///
/// [`TypeRegistry`]: crate::registry::TypeRegistry
/// [crate-level documentation]: crate
pub trait TypeTrait: DynamicTypePath + Any + Send + Sync {
    fn clone_type_trait(&self) -> Box<dyn TypeTrait>;
    fn register_dependencies(_registry: &mut TypeRegistry)
    where
        Self: Sized,
    {
    }
}

impl<T: Clone + Any + Send + Sync + DynamicTypePath> TypeTrait for T {
    #[inline]
    fn clone_type_trait(&self) -> Box<dyn TypeTrait> {
        Box::new(self.clone())
    }
}

impl dyn TypeTrait {
    /// Returns `true` if the underlying value is of type `T`.
    ///
    /// Note that this is this is a comparison of its own type,
    /// not the target type for implementing the trait.
    #[inline]
    pub fn is<T: Any>(&self) -> bool {
        self.type_id() == TypeId::of::<T>()
    }

    /// Downcasts the value to type `T` by reference.
    #[inline]
    pub fn downcast_ref<T: Any>(&self) -> Option<&T> {
        <dyn Any>::downcast_ref(self)
    }

    /// Downcasts the value to type `T` by mutable reference.
    #[inline]
    pub fn downcast_mut<T: Any>(&mut self) -> Option<&mut T> {
        <dyn Any>::downcast_mut(self)
    }
}

impl core::fmt::Debug for dyn TypeTrait {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.write_str(DynamicTypePath::reflect_type_name(self))
    }
}
