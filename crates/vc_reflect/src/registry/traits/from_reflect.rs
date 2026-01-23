use alloc::boxed::Box;

use crate::info::{TypePath, Typed};
use crate::registry::FromType;
use crate::{FromReflect, Reflect};

/// A function pointer container that enables dynamic conversion from reflected types.
///
/// While [`FromReflect`] allows conversion when the target type is statically known,
/// this container enables dynamic lookup and invocation using only type identifiers.
///
/// Primarily used in reflection-based deserialization where the target type must be
/// determined at runtime based on type paths or identifiers retrieved from serialized data.
///
/// # Examples
///
/// ```
/// # use vc_reflect::{Reflect, registry::{TypeRegistry, TypeTraitFromReflect}};
/// let s: Box<dyn Reflect> = Box::new("123".to_string());
///
/// let registry = TypeRegistry::new(); // `new` will register some basic type
/// let meta = registry.get_with_type_name("String").unwrap();
/// let from_reflect = meta.get_trait::<TypeTraitFromReflect>().unwrap();
///
/// let s2 = from_reflect.from_reflect(&*s).unwrap();
/// assert_eq!(s2.take::<String>().unwrap(), "123");
/// ```
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

// Explicitly implemented here so that code readers do not need
// to ponder the principles of proc-macros in advance.
impl TypePath for TypeTraitFromReflect {
    #[inline(always)]
    fn type_path() -> &'static str {
        "vc_reflect::registry::TypeTraitFromReflect"
    }

    #[inline(always)]
    fn type_name() -> &'static str {
        "TypeTraitFromReflect"
    }

    #[inline(always)]
    fn type_ident() -> &'static str {
        "TypeTraitFromReflect"
    }

    #[inline(always)]
    fn module_path() -> Option<&'static str> {
        Some("vc_reflect::registry")
    }
}

// -----------------------------------------------------------------------------
// Tests

#[cfg(test)]
mod tests {
    use super::TypeTraitFromReflect;
    use crate::info::TypePath;

    #[test]
    fn type_path() {
        assert!(TypeTraitFromReflect::type_path() == "vc_reflect::registry::TypeTraitFromReflect");
        assert!(TypeTraitFromReflect::module_path() == Some("vc_reflect::registry"));
        assert!(TypeTraitFromReflect::type_ident() == "TypeTraitFromReflect");
        assert!(TypeTraitFromReflect::type_name() == "TypeTraitFromReflect");
    }
}
