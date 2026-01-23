use alloc::boxed::Box;

use crate::Reflect;
use crate::info::{TypePath, Typed};
use crate::registry::FromType;

/// A container providing [`Default`] support for reflected types.
///
/// Then, you can create a reflect value using [`TypeRegistry`] and [`TypeId`] (or [`TypePath`]).
///
/// # Examples
///
/// ```
/// use vc_reflect::{Reflect, registry::{TypeRegistry, TypeTraitDefault}};
///
/// let registry = TypeRegistry::new(); // `new` will register some basic type
///
/// let generator = registry
///     .get_with_type_name("String").unwrap()
///     .get_trait::<TypeTraitDefault>().unwrap();
///
/// let s: Box<dyn Reflect> = generator.default();
///
/// assert_eq!(s.take::<String>().unwrap(), "");
/// ```
///
/// [`TypePath`]: crate::info::TypePath::type_path
/// [`TypeRegistry`]: crate::registry::TypeRegistry
/// [`TypeId`]: core::any::TypeId
#[derive(Clone)]
pub struct TypeTraitDefault {
    func: fn() -> Box<dyn Reflect>,
}

impl TypeTraitDefault {
    /// Call T's [`Default`]
    ///
    /// [`TypeTraitDefault`] does not have a type flag,
    /// but the functions used internally are type specific.
    #[inline(always)]
    pub fn default(&self) -> Box<dyn Reflect> {
        (self.func)()
    }
}

impl<T: Default + Typed + Reflect> FromType<T> for TypeTraitDefault {
    fn from_type() -> Self {
        Self {
            func: || Box::<T>::default(),
        }
    }
}

// Explicitly implemented here so that code readers do not need
// to ponder the principles of proc-macros in advance.
impl TypePath for TypeTraitDefault {
    #[inline(always)]
    fn type_path() -> &'static str {
        "vc_reflect::registry::TypeTraitDefault"
    }

    #[inline(always)]
    fn type_name() -> &'static str {
        "TypeTraitDefault"
    }

    #[inline(always)]
    fn type_ident() -> &'static str {
        "TypeTraitDefault"
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
    use super::TypeTraitDefault;
    use crate::info::TypePath;

    #[test]
    fn type_path() {
        assert!(TypeTraitDefault::type_path() == "vc_reflect::registry::TypeTraitDefault");
        assert!(TypeTraitDefault::module_path() == Some("vc_reflect::registry"));
        assert!(TypeTraitDefault::type_ident() == "TypeTraitDefault");
        assert!(TypeTraitDefault::type_name() == "TypeTraitDefault");
    }
}
