use core::any::TypeId;

use alloc::string::String;
use vc_os::sync::{Arc, PoisonError, RwLock, RwLockReadGuard, RwLockWriteGuard};
use vc_utils::{
    TypeIdMap,
    hash::{HashMap, HashSet},
};

use crate::{
    info::{TypeInfo, Typed},
    registry::{FromType, GetTypeMeta, TypeMeta, TypeTrait},
};

pub struct TypeRegistry {
    type_meta_table: TypeIdMap<TypeMeta>,
    type_path_to_id: HashMap<&'static str, TypeId>,
    type_name_to_id: HashMap<&'static str, TypeId>,
    ambiguous_names: HashSet<&'static str>,
}

impl Default for TypeRegistry {
    #[inline]
    fn default() -> Self {
        Self::new()
    }
}

impl TypeRegistry {
    /// Create a empty [`TypeRegistry`].
    #[inline]
    pub const fn empty() -> Self {
        Self {
            type_meta_table: TypeIdMap::new_no_op(),
            type_path_to_id: HashMap::new(),
            type_name_to_id: HashMap::new(),
            ambiguous_names: HashSet::new(),
        }
    }

    pub fn new() -> Self {
        let mut registry = Self::empty();
        registry.register::<()>();
        registry.register::<bool>();
        registry.register::<char>();
        registry.register::<u8>();
        registry.register::<u16>();
        registry.register::<u32>();
        registry.register::<u64>();
        registry.register::<u128>();
        registry.register::<usize>();
        registry.register::<i8>();
        registry.register::<i16>();
        registry.register::<i32>();
        registry.register::<i64>();
        registry.register::<i128>();
        registry.register::<isize>();
        registry.register::<f32>();
        registry.register::<f64>();
        registry.register::<String>();
        registry
    }

    // # Validity
    // The type must **not** already exist.
    fn add_new_type_indices(
        type_meta: &TypeMeta,
        type_path_to_id: &mut HashMap<&'static str, TypeId>,
        type_name_to_id: &mut HashMap<&'static str, TypeId>,
        ambiguous_names: &mut HashSet<&'static str>,
    ) {
        let ty = type_meta.ty();
        let type_name = ty.name();

        // Check for duplicate names.
        // The type should **not** already exist.
        if !ambiguous_names.contains(type_name) {
            if type_name_to_id.contains_key(type_name) {
                type_name_to_id.remove(type_name);
                ambiguous_names.insert(type_name);
            } else {
                type_name_to_id.insert(type_name, ty.id());
            }
        }

        // For new type, assuming that the full path cannot be duplicated.
        type_path_to_id.insert(ty.path(), ty.id());
    }

    // If key [`TypeId`] has already exist, the function will do nothing and return `false`.
    // If the key [`TypeId`] does not exist, the function will insert value and return `true`.
    fn register_internal(
        &mut self,
        type_id: TypeId,
        get_type_meta: impl FnOnce() -> TypeMeta,
    ) -> bool {
        use vc_utils::hash::hash_map::Entry;
        match self.type_meta_table.entry(type_id) {
            Entry::Occupied(_) => false, // duplicated
            Entry::Vacant(entry) => {
                let type_meta = get_type_meta();
                Self::add_new_type_indices(
                    &type_meta,
                    &mut self.type_path_to_id,
                    &mut self.type_name_to_id,
                    &mut self.ambiguous_names,
                );
                entry.insert(type_meta);
                true
            }
        }
    }

    /// Try add or do nothing.
    ///
    /// The function will will check if `TypeMeta.ty_id()` exists.  
    /// - If key [`TypeId`] has already exist, the function will do nothing and return `false`.
    /// - If the key [`TypeId`] does not exist, the function will insert value and return `true`.
    pub fn try_add_type_meta(&mut self, type_meta: TypeMeta) -> bool {
        self.register_internal(type_meta.ty_id(), || type_meta)
    }

    /// Insert or **Overwrite** inner TypeTraits.
    ///
    /// The function will will check if `TypeMeta.ty_id()` exists.  
    /// - If key [`TypeId`] has already exist, the value will be overwritten.
    ///   But full_path and type_name table will not be modified.  
    /// - If the key [`TypeId`] does not exist, the value will be inserted.
    ///   And type path will be inserted to full_path and type_name table.
    pub fn insert_type_meta(&mut self, type_meta: TypeMeta) {
        use vc_utils::hash::hash_map::Entry;
        match self.type_meta_table.entry(type_meta.type_info().ty_id()) {
            Entry::Occupied(mut entry) => {
                *entry.get_mut() = type_meta;
            }
            Entry::Vacant(entry) => {
                Self::add_new_type_indices(
                    &type_meta,
                    &mut self.type_path_to_id,
                    &mut self.type_name_to_id,
                    &mut self.ambiguous_names,
                );
                entry.insert(type_meta);
            }
        }
    }

    pub fn register<T: GetTypeMeta>(&mut self) {
        if self.register_internal(TypeId::of::<T>(), T::get_type_meta) {
            T::register_dependencies(self);
        }
    }

    /// Automatically registers all non-generic types annotated with `#[reflect(auto_register)]`
    /// or declared via `impl_auto_register!`.
    ///
    /// This method is equivalent to calling [`register`](Self::register) for each qualifying type.
    /// Repeated calls are cheap and will not insert duplicates.
    ///
    /// ## Return Value
    ///
    /// Returns `true` if automatic registration succeeded on the current platform; otherwise, `false`.
    /// Successful registrations remain `true` on subsequent calls, allowing you to detect platform support.
    ///
    /// ## Feature Dependency
    ///
    /// This method requires the `auto_register` feature. When disabled, it always do nothing and
    /// returns `false`.
    ///
    /// ## Platform Support
    ///
    /// Supported platforms include Linux, macOS, Windows, iOS, Android, and Web, enabled by
    /// the `inventory` crate. On unsupported platforms, this method becomes a no-op.
    ///
    /// # Examples
    ///
    /// ```
    /// # use std::any::TypeId;
    /// # use vc_reflect::{derive::Reflect, registry::{TypeRegistry, TypeTraitDefault}};
    /// #[derive(Reflect, Default)]
    /// #[reflect(default, auto_register)]
    /// struct Foo {
    ///     name: Option<String>,
    ///     value: i32,
    /// }
    ///
    /// let mut type_registry = TypeRegistry::empty();
    /// let successful = type_registry.auto_register();
    ///
    /// assert!(successful);
    ///
    /// // Main type is registered
    /// assert!(type_registry.contains(TypeId::of::<Foo>()));
    ///
    /// // Type dependencies are also registered
    /// assert!(type_registry.contains(TypeId::of::<Option<String>>()));
    /// assert!(type_registry.contains(TypeId::of::<i32>()));
    ///
    /// // Associated type data is available
    /// assert!(type_registry
    ///     .get_type_trait::<TypeTraitDefault>(TypeId::of::<Foo>())
    ///     .is_some());
    /// ```
    #[inline]
    pub fn auto_register(&mut self) -> bool {
        crate::cfg::auto_register! {
            if {
                use crate::__macro_exports::auto_register;
                // Reduce the cost of duplicate registrations.
                if self.contains(TypeId::of::<auto_register::__AvailFlag>()) {
                    return true;
                }
                auto_register::__register_types(self);
                self.contains(TypeId::of::<auto_register::__AvailFlag>())
            } else {
                false
            }
        }
    }

    #[inline(always)]
    pub fn register_by_val<T: GetTypeMeta>(&mut self, _: &T) {
        self.register::<T>();
    }

    pub fn register_type_trait<T: Typed, D: TypeTrait + FromType<T>>(&mut self) {
        match self.type_meta_table.get_mut(&TypeId::of::<T>()) {
            Some(type_meta) => type_meta.insert_trait(D::from_type()),
            None => panic!(
                "Called `TypeRegistry::register_type_trait`, but the type `{}` of type_trait `{}` without registering",
                T::type_path(),
                core::any::type_name::<D>(),
            ),
        }
    }

    #[inline]
    pub fn contains(&self, type_id: TypeId) -> bool {
        self.type_meta_table.contains_key(&type_id)
    }

    #[inline]
    pub fn get(&self, type_id: TypeId) -> Option<&TypeMeta> {
        self.type_meta_table.get(&type_id)
    }

    #[inline]
    pub fn get_mut(&mut self, type_id: TypeId) -> Option<&mut TypeMeta> {
        self.type_meta_table.get_mut(&type_id)
    }

    pub fn get_with_type_path(&self, type_path: &str) -> Option<&TypeMeta> {
        // Manual inline
        match self.type_path_to_id.get(type_path) {
            Some(id) => self.get(*id),
            None => None,
        }
    }

    pub fn get_with_type_path_mut(&mut self, type_path: &str) -> Option<&mut TypeMeta> {
        // Manual inline
        match self.type_path_to_id.get(type_path) {
            Some(id) => self.get_mut(*id),
            None => None,
        }
    }

    pub fn get_with_type_name(&self, type_name: &str) -> Option<&TypeMeta> {
        match self.type_name_to_id.get(type_name) {
            Some(id) => self.get(*id),
            None => None,
        }
    }

    pub fn get_with_type_name_mut(&mut self, type_name: &str) -> Option<&mut TypeMeta> {
        match self.type_name_to_id.get(type_name) {
            Some(id) => self.get_mut(*id),
            None => None,
        }
    }

    pub fn is_ambiguous(&self, type_name: &str) -> bool {
        self.ambiguous_names.contains(type_name)
    }

    pub fn get_type_trait<T: TypeTrait>(&self, type_id: TypeId) -> Option<&T> {
        // Manual inline
        match self.get(type_id) {
            Some(type_meta) => type_meta.get_trait::<T>(),
            None => None,
        }
    }

    pub fn get_type_trait_mut<T: TypeTrait>(&mut self, type_id: TypeId) -> Option<&mut T> {
        // Manual inline
        match self.get_mut(type_id) {
            Some(type_meta) => type_meta.get_trait_mut::<T>(),
            None => None,
        }
    }

    pub fn get_type_info(&self, type_id: TypeId) -> Option<&'static TypeInfo> {
        self.get(type_id).map(TypeMeta::type_info)
    }

    pub fn iter(&self) -> impl Iterator<Item = &TypeMeta> {
        self.type_meta_table.values()
    }

    pub fn iter_mut(&mut self) -> impl Iterator<Item = &mut TypeMeta> {
        self.type_meta_table.values_mut()
    }

    pub fn iter_with_trait<T: TypeTrait>(&self) -> impl Iterator<Item = (&TypeMeta, &T)> {
        self.type_meta_table.values().filter_map(|item| {
            let type_trait = item.get_trait::<T>();
            type_trait.map(|t| (item, t))
        })
    }
}

#[derive(Clone, Default)]
pub struct TypeRegistryArc {
    /// The wrapped [`TypeRegistry`].
    pub internal: Arc<RwLock<TypeRegistry>>,
}

impl TypeRegistryArc {
    /// Takes a read lock on the underlying [`TypeRegistry`].
    pub fn read(&self) -> RwLockReadGuard<'_, TypeRegistry> {
        self.internal.read().unwrap_or_else(PoisonError::into_inner)
    }

    /// Takes a write lock on the underlying [`TypeRegistry`].
    pub fn write(&self) -> RwLockWriteGuard<'_, TypeRegistry> {
        self.internal
            .write()
            .unwrap_or_else(PoisonError::into_inner)
    }
}

impl core::fmt::Debug for TypeRegistryArc {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        self.internal
            .read()
            .unwrap_or_else(PoisonError::into_inner)
            .type_path_to_id
            .keys()
            .fmt(f)
    }
}
