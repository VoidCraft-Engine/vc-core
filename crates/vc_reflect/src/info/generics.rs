use alloc::boxed::Box;
use core::{any::Any, ops::Deref};

use crate::info::{ConstParamData, Type, TypePath, impl_type_fn};

/// Information about generic type parameters.
#[derive(Clone, Debug)]
pub struct TypeParamInfo {
    ty: Type,
    name: &'static str,
    default: Option<Type>,
}

impl TypeParamInfo {
    impl_type_fn!(ty);

    /// Create a new [`TypeParamInfo`].
    #[inline]
    pub const fn new<T: TypePath + ?Sized>(name: &'static str) -> Self {
        Self {
            ty: Type::of::<T>(),
            name: name,
            default: None,
        }
    }

    /// Returns the generic parameter name.
    #[inline]
    pub const fn name(&self) -> &'static str {
        &self.name
    }

    /// Returns the default type for this parameter, if present.
    #[inline]
    pub const fn default(&self) -> Option<&Type> {
        self.default.as_ref()
    }

    /// Set the default type for this parameter.
    #[inline]
    pub const fn with_default<T: TypePath + ?Sized>(mut self) -> Self {
        self.default = Some(Type::of::<T>());
        self
    }
}

/// Information about a const generic parameter.
#[derive(Clone, Debug)]
pub struct ConstParamInfo {
    ty: Type,
    name: &'static str,
    default: Option<ConstParamData>,
}

impl ConstParamInfo {
    impl_type_fn!(ty);

    /// Create a new [`ConstParamInfo`].
    #[inline]
    pub const fn new<T: TypePath + Into<ConstParamData>>(name: &'static str) -> Self {
        Self {
            ty: Type::of::<T>(),
            name,
            default: None,
        }
    }

    /// Returns the generic parameter name.
    #[inline]
    pub const fn name(&self) -> &'static str {
        &self.name
    }

    /// Returns the default const value for this parameter, if present.
    #[inline]
    pub const fn default(&self) -> Option<ConstParamData> {
        self.default
    }

    /// Sets the default const value.
    ///
    /// # Panics
    ///
    /// Panics in debug builds if the provided value's type does not match the
    /// parameter's expected type.
    pub fn with_default<T: Any + Into<ConstParamData>>(mut self, default: T) -> Self {
        #[cfg(debug_assertions)]
        if !self.type_is::<T>() {
            panic!("default const value has incorrect type for this parameter");
        }
        self.default = Some(default.into());
        self
    }
}

/// A single generic parameter (either a type or a const).
#[derive(Clone, Debug)]
pub enum GenericInfo {
    Type(TypeParamInfo),
    Const(ConstParamInfo),
}

impl From<TypeParamInfo> for GenericInfo {
    #[inline]
    fn from(value: TypeParamInfo) -> Self {
        Self::Type(value)
    }
}

impl From<ConstParamInfo> for GenericInfo {
    #[inline]
    fn from(value: ConstParamInfo) -> Self {
        Self::Const(value)
    }
}

impl GenericInfo {
    impl_type_fn!(self => match self {
        Self::Type(info) => info.ty(),
        Self::Const(info) => info.ty(),
    });

    /// Returns the parameter name.
    #[inline]
    pub const fn name(&self) -> &'static str {
        match self {
            Self::Type(info) => info.name(),
            Self::Const(info) => info.name(),
        }
    }

    /// Returns `true` if this parameter is a const parameter.
    #[inline]
    pub const fn is_const(&self) -> bool {
        match self {
            Self::Type(_) => false,
            Self::Const(_) => true,
        }
    }
}

/// A container for a list of generic parameters.
///
/// This is automatically generated via the `Reflect` derive macro
/// and stored on the [`TypeInfo`] returned by [`Typed::type_info`]
/// for types that have generics.
///
/// It supports both type parameters and const parameters
/// so long as they implement [`TypePath`].
///
/// If the type has no generics, this will be empty.
///
/// [`TypeInfo`]: vc_reflect::info::TypeInfo
/// [`Typed::type_info`]: vc_reflect::info::Typed::type_info
#[derive(Clone, Default, Debug)]
pub struct Generics(Option<Box<[GenericInfo]>>);

impl Generics {
    /// Create a new, empty `Generics` container.
    #[inline(always)]
    pub const fn new() -> Self {
        // We use `Option` to enable compile time `new`.
        // The pointer cannot be null, which ensures that
        // the `Option` does not change the type size.
        Self(None)
    }

    /// Returns the `GenericInfo` for the parameter with the given `name`,
    /// if present.
    ///
    /// Complexity: O(n) in the number of parameters.
    pub fn get(&self, name: &str) -> Option<&GenericInfo> {
        match &self.0 {
            Some(val) => val.iter().find(|info| info.name() == name),
            None => None,
        }
    }

    pub fn from_iter<I: IntoIterator<Item = GenericInfo>>(iter: I) -> Self {
        // Typically this is constructed from a fixed-size array emitted by
        // proc-macros; the resulting `Box<[]>` will be compact with minimal
        // overhead.
        Self(Some(iter.into_iter().collect()))
    }
}

impl Deref for Generics {
    type Target = [GenericInfo];
    #[inline]
    fn deref(&self) -> &Self::Target {
        static EMPTY: [GenericInfo; 0] = [];
        match &self.0 {
            Some(v) => v,
            None => &EMPTY,
        }
    }
}

/// Implement `with_generics` and `generics` helper functions for a
/// containing type.
macro_rules! impl_generic_fn {
    ($field:ident) => {
        $crate::info::generics::impl_generic_fn!(self => &self.$field);

        /// Replace its own generic information
        #[inline]
        pub fn with_generics(
            mut self,
            generics: $crate::info::Generics
        ) -> Self {
            self.$field = generics;
            self
        }
    };
    ($self:ident => $expr:expr) => {
        /// Get generics from self based on expressions
        #[inline]
        pub const fn generics($self: &Self) -> &$crate::info::Generics {
            $expr
        }
    };
}

pub(crate) use impl_generic_fn;
