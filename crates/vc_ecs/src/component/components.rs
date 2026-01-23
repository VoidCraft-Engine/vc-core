#![expect(unsafe_code, reason = "get unchecked is unsafe")]

use core::any::TypeId;

use alloc::borrow::Cow;
use alloc::vec::Vec;

use vc_os::sync::{PoisonError, RwLock};
use vc_utils::extra::TypeIdMap;
use vc_utils::index::SparseIndexSet;

use crate::cfg;
use crate::component::{
    Component, ComponentDescriptor, RequiredComponents, RequiredComponentsError,
};
use crate::lifecycle::ComponentHooks;
use crate::utils::{DebugCheckedUnwrap, DebugName};

use super::{ComponentId, ComponentInfo, QueuedComponents};

#[derive(Debug)]
pub struct Components {
    pub infos: Vec<Option<ComponentInfo>>,
    pub component_indices: TypeIdMap<ComponentId>,
    pub resource_indices: TypeIdMap<ComponentId>,
    // This is kept internal and local to verify that no deadlocks can occur.
    pub queued: RwLock<QueuedComponents>,
}

impl Components {
    pub const fn empty() -> Self {
        Self {
            infos: Vec::new(),
            component_indices: TypeIdMap::new(),
            resource_indices: TypeIdMap::new(),
            queued: RwLock::new(QueuedComponents::empty()),
        }
    }

    #[inline]
    pub fn get_info(&self, id: ComponentId) -> Option<&ComponentInfo> {
        self.infos.get(id.index()).and_then(|info| info.as_ref())
    }

    #[inline]
    pub unsafe fn get_info_unchecked(&self, id: ComponentId) -> &ComponentInfo {
        // SAFETY: The caller ensures `id` is valid.
        unsafe {
            self.infos
                .get_unchecked(id.index())
                .as_ref()
                .debug_checked_unwrap()
        }
    }

    #[inline]
    pub unsafe fn get_info_unchecked_mut(&mut self, id: ComponentId) -> &mut ComponentInfo {
        // SAFETY: The caller ensures `id` is valid.
        unsafe {
            self.infos
                .get_unchecked_mut(id.index())
                .as_mut()
                .debug_checked_unwrap()
        }
    }

    pub fn iter_registered(&self) -> impl Iterator<Item = &ComponentInfo> + '_ {
        self.infos.iter().filter_map(Option::as_ref)
    }

    #[inline]
    pub fn num_registered(&self) -> usize {
        self.infos.len()
    }

    #[inline]
    pub fn any_registered(&self) -> bool {
        self.num_registered() != 0
    }

    // -----------------------------------------------------
    // queued

    #[inline]
    pub fn num_queued(&self) -> usize {
        let queued = self.queued.read().unwrap_or_else(PoisonError::into_inner);

        queued.components.len() + queued.dynamic_registrations.len() + queued.resources.len()
    }

    #[inline]
    pub fn any_queued(&self) -> bool {
        self.num_queued() != 0
    }

    #[inline]
    pub fn num_all(&self) -> usize {
        self.num_queued() + self.num_registered()
    }

    /// A faster version of [`Self::num_queued`].
    #[inline]
    pub fn num_queued_mut(&mut self) -> usize {
        let queued = self
            .queued
            .get_mut()
            .unwrap_or_else(PoisonError::into_inner);

        queued.components.len() + queued.dynamic_registrations.len() + queued.resources.len()
    }

    /// A faster version of [`Self::any_queued`].
    #[inline]
    pub fn any_queued_mut(&mut self) -> bool {
        self.num_queued_mut() != 0
    }

    #[inline(always)]
    pub fn get_queue_mut(&mut self) -> &mut QueuedComponents {
        self.queued
            .get_mut()
            .unwrap_or_else(PoisonError::into_inner)
    }

    /// Gets the [`ComponentDescriptor`] of the component with this [`ComponentId`] if it is present.
    /// This will return `None` only if the id is neither registered nor queued to be registered.
    ///
    /// Currently, the [`Cow`] will be [`Cow::Owned`] if and only if the component is queued. It will be [`Cow::Borrowed`] otherwise.
    ///
    /// This will return an incorrect result if `id` did not come from the same world as `self`. It may return `None` or a garbage value.
    #[inline]
    pub fn get_descriptor(&self, id: ComponentId) -> Option<Cow<'_, ComponentDescriptor>> {
        self.get_info(id)
            .map(|info| Cow::Borrowed(&info.descriptor))
            .or_else(|| {
                let queued = self.queued.read().unwrap_or_else(PoisonError::into_inner);

                queued
                    .find_by_id(id)
                    .map(|comp| Cow::Owned(comp.descriptor.clone()))
            })
    }

    #[inline]
    pub fn get_debug_name(&self, id: ComponentId) -> DebugName {
        self.get_info(id)
            .map(|info| info.debug_name().clone())
            .or_else(|| {
                let queued = self.queued.read().unwrap_or_else(PoisonError::into_inner);

                queued
                    .find_by_id(id)
                    .map(|comp| comp.descriptor.debug_name().clone())
            })
            .into()
    }

    #[inline]
    pub fn get_hooks_mut(&mut self, id: ComponentId) -> Option<&mut ComponentHooks> {
        self.infos
            .get_mut(id.index())
            .and_then(|info| info.as_mut().map(|info| &mut info.hooks))
    }

    #[inline]
    pub fn get_required_components(&self, id: ComponentId) -> Option<&RequiredComponents> {
        self.infos
            .get(id.index())
            .and_then(|info| info.as_ref().map(|info| &info.required_components))
    }

    #[inline]
    pub fn get_required_components_mut(
        &mut self,
        id: ComponentId,
    ) -> Option<&mut RequiredComponents> {
        self.infos
            .get_mut(id.index())
            .and_then(|info| info.as_mut().map(|info| &mut info.required_components))
    }

    #[inline]
    pub unsafe fn get_required_components_unchecked(&self, id: ComponentId) -> &RequiredComponents {
        unsafe {
            &self
                .infos
                .get_unchecked(id.index())
                .as_ref()
                .debug_checked_unwrap()
                .required_components
        }
    }

    #[inline]
    pub unsafe fn get_required_components_unchecked_mut(
        &mut self,
        id: ComponentId,
    ) -> &mut RequiredComponents {
        unsafe {
            &mut self
                .infos
                .get_unchecked_mut(id.index())
                .as_mut()
                .debug_checked_unwrap()
                .required_components
        }
    }

    pub fn get_required_by(&self, id: ComponentId) -> Option<&SparseIndexSet<ComponentId>> {
        self.infos
            .get(id.index())
            .and_then(|info| info.as_ref().map(|info| &info.required_by))
    }

    pub fn get_required_by_mut(
        &mut self,
        id: ComponentId,
    ) -> Option<&mut SparseIndexSet<ComponentId>> {
        self.infos
            .get_mut(id.index())
            .and_then(|info| info.as_mut().map(|info| &mut info.required_by))
    }

    #[inline]
    pub unsafe fn get_required_by_unchecked(
        &self,
        id: ComponentId,
    ) -> &SparseIndexSet<ComponentId> {
        unsafe {
            &self
                .infos
                .get_unchecked(id.index())
                .as_ref()
                .debug_checked_unwrap()
                .required_by
        }
    }

    #[inline]
    pub unsafe fn get_required_by_unchecked_mut(
        &mut self,
        id: ComponentId,
    ) -> &mut SparseIndexSet<ComponentId> {
        unsafe {
            &mut self
                .infos
                .get_unchecked_mut(id.index())
                .as_mut()
                .debug_checked_unwrap()
                .required_by
        }
    }

    pub fn get_component_id(&self, type_id: TypeId) -> Option<ComponentId> {
        self.component_indices.get(&type_id).copied().or_else(|| {
            self.queued
                .read()
                .unwrap_or_else(PoisonError::into_inner)
                .components
                .get(&type_id)
                .map(|queued| queued.component_id)
        })
    }

    #[inline]
    pub fn component_id_of<T: Component>(&self) -> Option<ComponentId> {
        self.get_component_id(TypeId::of::<T>())
    }

    pub fn get_resource_id(&self, type_id: TypeId) -> Option<ComponentId> {
        self.resource_indices.get(&type_id).copied().or_else(|| {
            self.queued
                .read()
                .unwrap_or_else(PoisonError::into_inner)
                .resources
                .get(&type_id)
                .map(|queued| queued.component_id)
        })
    }

    #[inline]
    pub fn resource_id_of<T: crate::resource::Resource>(&self) -> Option<ComponentId> {
        self.get_resource_id(TypeId::of::<T>())
    }

    #[inline]
    pub fn is_id_registered(&self, id: ComponentId) -> bool {
        self.infos.get(id.index()).is_some_and(Option::is_some)
    }

    #[inline]
    pub fn get_valid_component_id(&self, type_id: TypeId) -> Option<ComponentId> {
        self.component_indices.get(&type_id).copied()
    }

    #[inline]
    pub fn get_valid_resource_id(&self, type_id: TypeId) -> Option<ComponentId> {
        self.resource_indices.get(&type_id).copied()
    }

    #[inline]
    pub fn valid_component_id<T: Component>(&self) -> Option<ComponentId> {
        self.get_valid_component_id(TypeId::of::<T>())
    }

    #[inline]
    pub fn valid_resource_id<T: crate::resource::Resource>(&self) -> Option<ComponentId> {
        self.get_valid_resource_id(TypeId::of::<T>())
    }

    // -----------------------------------------------------
    // register

    /// # Safety
    /// - The [`ComponentId`] must be unique and not be registered or queued.
    #[inline(always)]
    pub unsafe fn register_dynamic(&mut self, id: ComponentId, descriptor: ComponentDescriptor) {
        let info = ComponentInfo::new(id, descriptor);
        let index = id.index();

        let least_len = index + 1;
        if least_len > self.infos.len() {
            self.infos.resize_with(least_len, || None);
            // we ensure `len` is the correct number of component kind.
            // cannot resize_with_capacity
        }

        // SAFETY: We just extended the vec to make this index valid.
        let slot = unsafe { self.infos.get_unchecked_mut(index) };

        cfg::debug! {
            // Caller ensures id is unique
            assert!(slot.is_none());
        }

        *slot = Some(info);
    }

    /// # Safety
    /// - The [`ComponentDescriptor`] must match the [`TypeId`].
    /// - The [`ComponentId`] must be unique.
    /// - The [`TypeId`] and [`ComponentId`] must not be registered or queued.
    #[inline(always)]
    pub unsafe fn register_component(
        &mut self,
        type_id: TypeId,
        component_id: ComponentId,
        descriptor: ComponentDescriptor,
    ) {
        // SAFETY: ensured by caller
        unsafe {
            self.register_dynamic(component_id, descriptor);
        }
        let prev = self.component_indices.insert(type_id, component_id);

        debug_assert!(prev.is_none());
    }

    /// # Safety
    /// - The [`ComponentDescriptor`] must match the [`TypeId`].
    /// - The [`ComponentId`] must be unique.
    /// - The [`TypeId`] and [`ComponentId`] must not be registered or queued.
    #[inline(always)]
    pub unsafe fn register_resource(
        &mut self,
        type_id: TypeId,
        component_id: ComponentId,
        descriptor: ComponentDescriptor,
    ) {
        // SAFETY: ensured by caller
        unsafe {
            self.register_dynamic(component_id, descriptor);
        }
        let prev = self.resource_indices.insert(type_id, component_id);

        debug_assert!(prev.is_none());
    }

    /// # Safety:
    ///
    /// - `requiree` must have been registered in `self`
    /// - all components in `required_components` must have been registered in `self`;
    /// - this is called with `requiree` before being called on any component requiring `requiree`.
    #[inline(always)]
    pub unsafe fn register_required_by(
        &mut self,
        requiree: ComponentId,
        required_components: &RequiredComponents,
    ) {
        for &required in required_components.all.keys() {
            cfg::debug! {
                assert!(required.index() < self.infos.len());
            }

            // SAFETY: the caller guarantees that all components in `required_components` have been registered in `self`.
            let infos = unsafe { self.get_info_unchecked_mut(required) };
            // This preserves the invariant of `required_by` because:
            // - components requiring `required` and required by `requiree` are already initialized at this point
            //   and hence registered in `required_by` before `requiree`;
            // - components requiring `requiree` cannot exist yet, as this is called on `requiree` before them.
            infos.required_by.insert(requiree);
        }
    }

    unsafe fn required_components_scope<R>(
        &mut self,
        component_id: ComponentId,
        f: impl FnOnce(&mut Self, &mut RequiredComponents) -> R,
    ) -> R {
        struct DropGuard<'a> {
            components: &'a mut Components,
            component_id: ComponentId,
            required_components: RequiredComponents,
        }

        impl Drop for DropGuard<'_> {
            fn drop(&mut self) {
                // SAFETY: The caller ensures that the `component_id` is valid.
                let required_components = unsafe {
                    self.components
                        .get_required_components_unchecked_mut(self.component_id)
                };

                cfg::debug! {
                    assert!(required_components.direct.is_empty());
                    assert!(required_components.all.is_empty());
                }

                *required_components = core::mem::take(&mut self.required_components);
            }
        }

        let mut guard = DropGuard {
            component_id,
            // SAFETY: The caller ensures that the `component_id` is valid.
            required_components: core::mem::take(unsafe {
                self.get_required_components_unchecked_mut(component_id)
            }),
            components: self,
        };

        f(guard.components, &mut guard.required_components)
    }

    #[inline(never)]
    fn assert_get_old_required_count(
        &mut self,
        requiree: ComponentId,
        required: ComponentId,
    ) -> Result<usize, RequiredComponentsError> {
        cfg::debug! {
            assert!(required.index() < self.infos.len());
            assert!(requiree.index() < self.infos.len());
        }
        let required_required_components =
            unsafe { self.get_required_components_unchecked(required) };

        if required_required_components.all.contains_key(&requiree) {
            return Err(RequiredComponentsError::CyclicRequirement(
                requiree, required,
            ));
        }

        let requiree_required_components =
            unsafe { self.get_required_components_unchecked_mut(requiree) };

        if requiree_required_components.direct.contains_key(&required) {
            return Err(RequiredComponentsError::DuplicateRegistration(
                requiree, required,
            ));
        }

        Ok(requiree_required_components.all.len())
    }

    pub unsafe fn register_required_components<R: Component>(
        &mut self,
        requiree: ComponentId,
        required: ComponentId,
        constructor: fn() -> R,
    ) -> Result<(), RequiredComponentsError> {
        let old_required_count = Self::assert_get_old_required_count(self, requiree, required)?;

        unsafe {
            self.required_components_scope(requiree, |this, requiree_requireds| {
                // SAFETY: the caller guarantees that `required` is valid for type `R` in `self`
                requiree_requireds.register_by_id(required, this, constructor);
            });
        }

        let required_components = unsafe { self.get_required_components_unchecked_mut(requiree) };

        let new_required_components = required_components.all[old_required_count..]
            .keys()
            .copied()
            .collect::<Vec<ComponentId>>();

        let requiree_required_by = unsafe { self.get_required_by_unchecked(requiree) };

        let new_requirees = [requiree]
            .into_iter()
            .chain(requiree_required_by.iter().copied())
            .collect::<SparseIndexSet<ComponentId>>();

        for &indirect_requiree in &new_requirees[1..] {
            // SAFETY: `indirect_requiree` comes from `self` so it must be valid.
            unsafe {
                self.required_components_scope(indirect_requiree, |this, indirect_requireds| {
                    // Rebuild the inherited required components.
                    // SAFETY: `required_components` comes from `self`, so all its components must have be valid in `self`.
                    indirect_requireds.rebuild_inherited_required_components(this);
                });
            }
        }

        for &indirect_required in &new_required_components {
            // SAFETY: `indirect_required` comes from `self`, so it must be valid.
            let required_by = unsafe { self.get_required_by_unchecked_mut(indirect_required) };

            // Remove and re-add all the components in `new_requirees`
            // This preserves the invariant of `required_by` because `new_requirees`
            // satisfies its invariant, due to being `requiree` followed by its `required_by` components,
            // and because any component not in `new_requirees` cannot require a component in it,
            // since if that was the case it would appear in the `required_by` for `requiree`.
            required_by.retain(|id| !new_requirees.contains(id));
            required_by.extend(&new_requirees);
        }

        Ok(())
    }
}
