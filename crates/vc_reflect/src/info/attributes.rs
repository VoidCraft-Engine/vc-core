use core::any::TypeId;

use alloc::boxed::Box;
use vc_utils::TypeIdMap;

use crate::Reflect;

/// A collection of custom attributes for a type, field, or variant.
///
/// These attributes can be created with the [`Reflect` derive macro].
///
/// Attributes are stored by their [`TypeId`].
/// Because of this, there can only be one attribute per type.
///
/// # Example
///
/// ```
/// # use vc_reflect::{derive::Reflect, info::{Typed, TypeInfo}};
///
/// #[derive(Reflect)]
/// struct Slider {
///   #[reflect(@10.0f32)]
///   value: f32
/// }
///
/// let info = <Slider as Typed>::type_info().as_struct().unwrap();
///
/// let val = info.field("value").unwrap().get_attribute::<f32>().unwrap();
/// assert_eq!(10.0f32, *val);
/// ```
///
/// [`Reflect` derive macro]: crate::derive::Reflect
#[derive(Default)]
#[repr(transparent)]
pub struct CustomAttributes {
    attributes: TypeIdMap<Box<dyn Reflect>>,
}

impl CustomAttributes {
    /// A static reference to an empty [`CustomAttributes`].
    ///
    /// `TypeInfo` stores custom attributes as `Option<Arc<..>>` to avoid heap
    /// allocations when there are no attributes.
    ///
    /// To avoid returning `None`, we provide this const empty instance.
    pub const EMPTY: &'static Self = &Self::new();

    /// Creates an empty [`CustomAttributes`].
    ///
    /// Equivalent to [`Default`], but this is const function.
    #[inline]
    pub const fn new() -> Self {
        Self {
            attributes: TypeIdMap::new_no_op(),
        }
    }

    /// Creates an empty [`CustomAttributes`] with specific capacity.
    #[inline]
    pub fn with_capacity(capacity: usize) -> Self {
        Self {
            attributes: TypeIdMap::with_capacity_no_op(capacity),
        }
    }

    /// Adds an attribute.
    ///
    /// Attributes are keyed by their concrete type; later insertions for the
    /// same type overwrite earlier values.
    #[inline]
    pub fn with_attribute<T: Reflect>(mut self, value: T) -> Self {
        self.attributes.insert(TypeId::of::<T>(), Box::new(value));
        self
    }

    /// Returns an iterator over the stored attributes.
    #[inline]
    pub fn iter(&self) -> impl ExactSizeIterator<Item = (&TypeId, &dyn Reflect)> {
        self.attributes.iter().map(|(key, val)| (key, &**val))
    }

    /// Returns `true` if an attribute of type `T` is present.
    #[inline]
    pub fn contains<T: Reflect>(&self) -> bool {
        self.attributes.contains_key(&TypeId::of::<T>())
    }

    /// Returns `true` if it contains the attribute with the given `TypeId`.
    #[inline]
    pub fn contains_by_id(&self, id: TypeId) -> bool {
        self.attributes.contains_key(&id)
    }

    /// Returns the attribute of type `T`, if present.
    #[inline]
    pub fn get<T: Reflect>(&self) -> Option<&T> {
        self.attributes.get(&TypeId::of::<T>())?.downcast_ref::<T>()
    }

    /// Returns the attribute with the given `TypeId`, if present.
    #[inline]
    pub fn get_by_id(&self, id: TypeId) -> Option<&dyn Reflect> {
        Some(self.attributes.get(&id)?.as_ref())
    }

    /// Returns the number of stored attributes.
    #[inline]
    pub fn len(&self) -> usize {
        self.attributes.len()
    }

    /// Returns `true` if no attributes are stored.
    #[inline]
    pub fn is_empty(&self) -> bool {
        self.attributes.is_empty()
    }
}

impl core::fmt::Debug for CustomAttributes {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_set().entries(self.attributes.values()).finish()
    }
}

macro_rules! impl_custom_attributes_fn {
    ($field:ident) => {
        /// Returns the attribute of type `T`, if present.
        #[inline]
        pub fn custom_attributes(&self) -> &$crate::info::CustomAttributes {
            match &self.$field {
                Some(ptr) => &**ptr,
                None => $crate::info::CustomAttributes::EMPTY,
            }
        }

        $crate::info::impl_custom_attributes_fn!();
    };
    () => {
        /// Returns the attribute of type `T`, if present.
        pub fn get_attribute<T: $crate::Reflect>(&self) -> Option<&T> {
            self.custom_attributes().get::<T>()
        }

        /// Returns the attribute with the given `TypeId`, if present.
        pub fn get_attribute_by_id(
            &self,
            type_id: ::core::any::TypeId,
        ) -> Option<&dyn $crate::Reflect> {
            self.custom_attributes().get_by_id(type_id)
        }

        /// Returns `true` if it contains the given attribute type.
        pub fn has_attribute<T: $crate::Reflect>(&self) -> bool {
            self.custom_attributes().contains::<T>()
        }

        /// Returns `true` if it contains the attribute with the given `TypeId`.
        pub fn has_attribute_by_id(&self, type_id: ::core::any::TypeId) -> bool {
            self.custom_attributes().contains_by_id(type_id)
        }
    };
}

macro_rules! impl_with_custom_attributes {
    ($field:ident) => {
        /// Replaces stored attributes (overwrite, do not merge).
        ///
        /// Used by the proc-macro crate.
        pub fn with_custom_attributes(self, attributes: CustomAttributes) -> Self {
            if attributes.is_empty() {
                Self {
                    $field: None,
                    ..self
                }
            } else {
                Self {
                    $field: Some(Arc::new(attributes)),
                    ..self
                }
            }
        }
    };
}

pub(crate) use impl_custom_attributes_fn;
pub(crate) use impl_with_custom_attributes;
