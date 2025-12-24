use alloc::boxed::Box;
use core::{
    any::{Any, TypeId},
    ops::{Deref, DerefMut},
};
use vc_utils::TypeIdMap;

use crate::{
    Reflect,
    info::{TypeInfo, Typed},
    registry::{TypeRegistry, TypeTrait},
};

/// Runtime storage for type metadata, registered into the [`TypeRegistry`].
///
/// This includes a [`TypeInfo`] and a [`TypeTrait`] table.
///
/// An instance of `TypeMeta` can be created using the [`TypeMeta::of`] method,
/// but is more often automatically generated using [`#[derive(Reflect)]`](crate::derive::Reflect)
/// which itself generates an implementation of the [`GetTypeMeta`](crate::registry::GetTypeMeta) trait.
///
/// # Example
///
/// ```
/// # use vc_reflect::registry::{TypeMeta, TypeTraitDefault, FromType};
/// let mut meta = TypeMeta::of::<String>();
/// meta.insert_trait::<TypeTraitDefault>(FromType::<String>::from_type());
///
/// let f = meta.get_trait::<TypeTraitDefault>().unwrap();
/// let s = f.default().take::<String>().unwrap();
///
/// assert_eq!(s, "");
/// ```
///
/// See the [crate-level documentation] for more information on type_meta.
///
/// [crate-level documentation]: crate
pub struct TypeMeta {
    type_info: &'static TypeInfo,
    trait_table: TypeIdMap<Box<dyn TypeTrait>>,
}

impl TypeMeta {
    /// Create a empty [`TypeMeta`] from a type.
    ///
    /// If you know the number of [`TypeTrait`] in advance,
    /// consider use [`TypeMeta::with_capacity`] for better performence,
    ///
    /// # Examples
    ///
    /// ```
    /// use vc_reflect::registry::TypeMeta;
    /// let mut meta = TypeMeta::of::<String>();
    /// ```
    #[inline]
    pub fn of<T: Typed>() -> Self {
        Self {
            trait_table: TypeIdMap::new_no_op(),
            type_info: T::type_info(),
        }
    }

    /// Create a empty [`TypeMeta`] from a type with capacity.
    #[inline]
    pub fn with_capacity<T: Typed>(capacity: usize) -> Self {
        Self {
            trait_table: TypeIdMap::with_capacity_no_op(capacity),
            type_info: T::type_info(),
        }
    }

    /// Returns the [`TypeInfo`] .
    #[inline(always)]
    pub fn type_info(&self) -> &'static TypeInfo {
        self.type_info
    }

    /// Returns the [`Type`](crate::info::Type) .
    #[inline]
    pub fn ty(&self) -> &'static crate::info::Type {
        self.type_info.ty()
    }

    /// Returns the [`TypePathTable`](crate::info::TypePathTable).
    #[inline]
    pub fn type_path_table(&self) -> &'static crate::info::TypePathTable {
        &self.type_info.ty().path_table()
    }

    /// Returns the [`TypeId`].
    #[inline]
    pub fn ty_id(&self) -> TypeId {
        self.type_info.ty().id()
    }

    /// Check if the given type matches this one.
    ///
    /// This only compares the `TypeId` of the types.
    #[inline]
    pub fn type_is<T: Any>(&self) -> bool {
        self.type_info.ty().id() == TypeId::of::<T>()
    }

    /// Returns the type path.
    #[inline]
    pub fn type_path(&self) -> &'static str {
        self.type_info.ty().path()
    }

    /// Returns the type name.
    #[inline]
    pub fn type_name(&self) -> &'static str {
        self.type_info.ty().name()
    }

    /// Returns the type ident.
    #[inline]
    pub fn type_ident(&self) -> &'static str {
        self.type_info.ty().ident()
    }

    /// Returns the module path.
    #[inline]
    pub fn module_path(&self) -> Option<&'static str> {
        self.type_info.ty().module_path()
    }

    /// Returns the crate name.
    #[inline]
    pub fn crate_name(&self) -> Option<&'static str> {
        self.type_info.ty().crate_name()
    }

    /// Returns the [`Generics`](crate::info::Generics) .
    #[inline]
    pub fn generics(&self) -> &'static crate::info::Generics {
        self.type_info.generics()
    }

    /// Return the docs.
    ///
    /// If reflect_docs feature is not enabled, this function always return `None`.
    /// So you can use this without worrying about compilation options.
    #[inline]
    pub fn docs(&self) -> Option<&'static str> {
        self.type_info.docs()
    }

    /// Returns the [`CustomAttributes`](crate::info::CustomAttributes) .
    #[inline]
    pub fn custom_attributes(&self) -> &'static crate::info::CustomAttributes {
        self.type_info.custom_attributes()
    }

    /// Returns the attribute of type `T`, if present.
    pub fn get_attribute<T: Reflect>(&self) -> Option<&'static T> {
        self.custom_attributes().get::<T>()
    }

    /// Returns the attribute with the given `TypeId`, if present.
    pub fn get_attribute_by_id(
        &self,
        type_id: ::core::any::TypeId,
    ) -> Option<&'static dyn Reflect> {
        self.custom_attributes().get_by_id(type_id)
    }

    /// Returns `true` if it contains the given attribute type.
    pub fn has_attribute<T: Reflect>(&self) -> bool {
        self.custom_attributes().contains::<T>()
    }

    /// Returns `true` if it contains the attribute with the given `TypeId`.
    pub fn has_attribute_by_id(&self, type_id: ::core::any::TypeId) -> bool {
        self.custom_attributes().contains_by_id(type_id)
    }

    /// Insert a new [`TypeTrait`].
    #[inline]
    pub fn insert_trait<T: TypeTrait>(&mut self, data: T) {
        self.trait_table.insert(TypeId::of::<T>(), Box::new(data));
    }

    /// Removes a [`TypeTrait`] from the meta.
    pub fn remove_trait<T: TypeTrait>(&mut self) -> Option<Box<T>> {
        self.trait_table
            .remove(&TypeId::of::<T>())
            .map(|val| <Box<dyn Any>>::downcast::<T>(val).unwrap())
    }

    /// Removes a [`TypeTrait`] from the meta.
    pub fn remove_trait_by_id(&mut self, type_id: TypeId) -> Option<Box<dyn TypeTrait>> {
        self.trait_table.remove(&type_id)
    }

    /// Get a [`TypeTrait`] reference, or return `None` if it's doesn't exist.
    pub fn get_trait<T: TypeTrait>(&self) -> Option<&T> {
        self.trait_table
            .get(&TypeId::of::<T>())
            .and_then(|val| val.downcast_ref::<T>())
    }

    /// Get a [`TypeTrait`] reference, or return `None` if it's doesn't exist.
    pub fn get_trait_by_id(&self, type_id: TypeId) -> Option<&dyn TypeTrait> {
        self.trait_table.get(&type_id).map(Deref::deref)
    }

    /// Get a mutable [`TypeTrait`] reference, or return `None` if it's doesn't exist.
    pub fn get_trait_mut<T: TypeTrait>(&mut self) -> Option<&mut T> {
        self.trait_table
            .get_mut(&TypeId::of::<T>())
            .and_then(|val| val.downcast_mut())
    }

    /// Get a mutable [`TypeTrait`] reference, or return `None` if it's doesn't exist.
    pub fn get_trait_mut_by_id(&mut self, type_id: TypeId) -> Option<&mut dyn TypeTrait> {
        self.trait_table.get_mut(&type_id).map(DerefMut::deref_mut)
    }

    /// Return true if specific [`TypeTrait`] is exist.
    pub fn has_trait<T: TypeTrait>(&self) -> bool {
        self.trait_table.contains_key(&TypeId::of::<T>())
    }

    /// Return true if specific [`TypeTrait`] is exist.
    pub fn has_trait_by_id(&self, type_id: TypeId) -> bool {
        self.trait_table.contains_key(&type_id)
    }

    /// Return the number of [`TypeTrait`].
    #[inline]
    pub fn trait_len(&self) -> usize {
        self.trait_table.len()
    }

    /// An iterator visiting all `TypeId - &dyn TypeTrait` pairs in arbitrary order.
    pub fn trait_iter(&self) -> impl ExactSizeIterator<Item = (TypeId, &dyn TypeTrait)> {
        self.trait_table
            .iter()
            .map(|(key, val)| (*key, val.deref()))
    }

    /// An iterator visiting all `TypeId - &mut dyn TypeTrait` pairs in arbitrary order.
    pub fn trait_iter_mut(
        &mut self,
    ) -> impl ExactSizeIterator<Item = (TypeId, &mut dyn TypeTrait)> {
        self.trait_table
            .iter_mut()
            .map(|(key, val)| (*key, val.deref_mut()))
    }

    /// Reserves capacity for at least additional more elements
    /// to be inserted in the trait table.
    pub fn reserve_trait_table(&mut self, additional: usize) {
        self.trait_table.reserve(additional);
    }

    /// Shrinks the capacity of the trait table as much as possible.
    pub fn shrink_trait_table(&mut self) {
        self.trait_table.shrink_to_fit();
    }
}

impl Clone for TypeMeta {
    fn clone(&self) -> Self {
        let mut new_map = TypeIdMap::with_capacity_no_op(self.trait_len());
        for (id, type_trait) in &self.trait_table {
            new_map.insert(*id, (**type_trait).clone_type_trait());
        }

        Self {
            trait_table: new_map,
            type_info: self.type_info,
        }
    }
}

impl core::fmt::Debug for TypeMeta {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_struct("TypeMeta")
            .field("type_info", &self.type_info)
            .field("trait_table", &self.trait_table)
            .finish()
    }
}

/// A trait which allows a type to generate its [`TypeMeta`]
/// for registration into the [`TypeRegistry`].
///
/// This trait is automatically implemented for items using [`#[derive(Reflect)]`](crate::derive::Reflect).
/// The macro also allows [`TypeTrait`] to be more easily registered.
///
/// # Implementation
///
/// Use [`#[derive(`Reflect`)]`](crate::derive::Reflect):
///
/// ```
/// use vc_reflect::{derive::Reflect, registry::GetTypeMeta};
///
/// #[derive(Reflect)]
/// struct A;
///
/// let meta = A::get_type_meta();
/// ```
///
/// Add additional [`TypeTrait`]:
///
/// ```
/// use vc_reflect::{derive::{Reflect, reflect_trait}, registry::GetTypeMeta};
///
/// #[reflect_trait]
/// trait MyDisplay {
///     fn display(&self) { /* ... */ }
/// }
///
/// impl MyDisplay for A{}
///
/// #[derive(Reflect)]
/// #[reflect(type_trait = ReflectMyDisplay)]
/// struct A;
///
/// let meta = A::get_type_meta();
///
/// assert!(meta.has_trait::<ReflectMyDisplay>());
/// ```
///
/// See more infomation in [`derive::reflect_trait`](crate::derive::reflect_trait).
///
/// ## Manually
///
/// ```
/// use vc_reflect::{derive::{Reflect, reflect_trait}, registry::{GetTypeMeta, FromType, TypeMeta}};
///
/// #[reflect_trait]
/// trait MyDisplay {
///     fn display(&self) { /* ... */ }
/// }
///
/// impl MyDisplay for A{}
///
/// #[derive(Reflect)]
/// #[reflect(GetTypeMeta = false)]
/// struct A;
///
/// impl GetTypeMeta for A {
///     fn get_type_meta() -> TypeMeta {
///         let mut meta = TypeMeta::of::<Self>();
///         meta.insert_trait::<ReflectMyDisplay>(FromType::<Self>::from_type());
///         meta
///     }
/// }
///
/// let meta = A::get_type_meta();
/// assert!(meta.has_trait::<ReflectMyDisplay>());
/// ```
///
/// [`TypeTrait`]: crate::registry::TypeTrait
/// [crate-level documentation]: crate
#[diagnostic::on_unimplemented(
    message = "`{Self}` does not implement `GetTypeMeta` so cannot provide type registration information",
    note = "consider annotating `{Self}` with `#[derive(Reflect)]`"
)]
pub trait GetTypeMeta: Typed {
    /// Returns the **default** [`TypeMeta`] for this type.
    fn get_type_meta() -> TypeMeta;

    /// Registers other types needed by this type.
    /// **Allow** not to register oneself.
    fn register_dependencies(_registry: &mut TypeRegistry) {}
}
