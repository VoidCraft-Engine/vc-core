use alloc::boxed::Box;

use crate::{FromReflect, Reflect, info::Typed, registry::FromType};

/// See [`FromReflect`]
#[derive(Clone)]
pub struct TypeTraitFromReflect {
    func: fn(&dyn Reflect) -> Option<Box<dyn Reflect>>,
}

impl TypeTraitFromReflect {
    /// Call T's [`Reflect`]
    ///
    /// [`TypeTraitFromReflect`] does not have a type flag,
    /// but the functions used internally are type specific.
    #[inline(always)]
    pub fn from_reflect(&self, param_1: &dyn Reflect) -> Option<Box<dyn Reflect>> {
        (self.func)(param_1)
    }
}

impl<T: Typed + FromReflect> FromType<T> for TypeTraitFromReflect {
    fn from_type() -> Self {
        Self {
            func: |param_1| T::from_reflect(param_1).map(Reflect::into_boxed_reflect),
        }
    }
}

crate::derive::impl_type_path!(::vc_reflect::registry::TypeTraitFromReflect);
