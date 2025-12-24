use core::any::{Any, TypeId};

/// A static accessor to type paths and names.
///
/// Provides a stable and flexible alternative to [`core::any::type_name`]
/// that works across compiler versions and survives code refactoring.
///
/// # Methods
///
/// - [`type_path`]: The unique identifier of the type, cannot be duplicated.
/// - [`type_name`]: Type name without module path, may be duplicated.
/// - [`type_ident`]: The shortest type name without module path and generics.
/// - [`module_path`]: Optional module path.
///
/// We guarantee that these names do not have the prefix `::`.
/// Users should also ensure this when manually implementing it.
///
/// We did not provide A to reduce compilation time and the size of [`TypePathTable`].
/// But we provide the [`TypePathTable::crate_name`] method to quickly parse it from module_path.
///
/// # Implementation
///
/// ## Use [`#[derive(`Reflect`)]`](crate::derive::Reflect)
///
/// ```
/// use vc_reflect::derive::Reflect;
///
/// // This type path will not change with compiler versions or recompiles,
/// // although it will not be the same if the definition is moved.
/// #[derive(Reflect)]
/// struct NonStableTypePath;
///
/// // This type path will never change, even if the definition is moved.
/// #[derive(Reflect)]
/// #[reflect(type_path = "my_crate::foo::StableTypePath")]
/// struct StableTypePath;
///
/// // Type paths can have any number of path segments.
/// #[derive(Reflect)]
/// #[reflect(type_path = "my_crate::foo::bar::baz")]
/// struct DeeplyNestedStableTypePath;
///
/// // Generics are also supported, will be recognized by macro automatically.
/// // Should not not manually mark.
/// #[derive(Reflect)]
/// #[reflect(type_path = "my_crate::foo::StableGenericTypePath")]
/// struct StableGenericTypePath<T, const N: usize>([T; N]);
///
/// // All other trait can be disabled, only implementing TypePath.
/// #[derive(Reflect)]
/// #[reflect(Typed = false, Reflect = false)]
/// #[reflect(FromReflect = false, GetTypeMeta = false)]
/// #[reflect(Struct = false, TupleStruct = false, Enum = false)] // optional
/// struct TypePathOnly;
/// ```
///
/// ## Use [`impl_type_path!`](crate::derive::impl_type_path)
///
/// ```ignore
/// // impl for primitive type.
/// impl_type_path!(u64);
///
/// // Implement for specified type.
/// impl_type_path!(::alloc::string::String);
/// // The prefix `::` will be removed by the macro, but it's required.
/// // This indicates that it's a complete path.
///
/// // Generics are also supported.
/// impl_type_path!(::utils::One<T>);
///
/// // Custom module path for specified type.
/// // then, it's type_path is `core::time::Instant`
/// impl_type_path!((in core::time) Instant);
///
/// // Custom module and ident for specified type.
/// // then, it's type_path is `core::time::Ins`
/// impl_type_path!((in core::time as Ins) Instant);
/// ```
///
/// ## Manually
///
/// We guarantee that these names do not have the prefix `::`.
/// Users should also ensure this when manually implementing it.
///
/// For non generic types, implementation is simple.
///
/// ```
/// use vc_reflect::info::TypePath;
///
/// struct Foo;
///
/// impl TypePath for Foo {
///     fn type_path() -> &'static str { "my_crate::foo::Foo" }
///     fn type_name() -> &'static str { "Foo" }
///     fn type_ident() -> &'static str { "Foo" }
///     fn module_path() -> Option<&'static str> { Some("my_crate::foo") }
/// }
/// ```
///
/// For generic types, we provide [`GenericTypePathCell`] to simplify it.
///
/// ```
/// use vc_reflect::info::TypePath;
/// use vc_reflect::impls::{concat, GenericTypePathCell};
///
/// struct Foo<T>(T);
///
/// impl<T: TypePath> TypePath for Foo<T> {
///     fn type_path() -> &'static str {
///         static CELL: GenericTypePathCell = GenericTypePathCell::new();
///         CELL.get_or_insert::<Self>(||{
///             concat(&["my_crate::foo::Foo", "<", T::type_path(), ">"])
///         })
///     }
///     fn type_name() -> &'static str {
///         static CELL: GenericTypePathCell = GenericTypePathCell::new();
///         CELL.get_or_insert::<Self>(||{
///             concat(&["Foo", "<", T::type_name(), ">"])
///         })
///     }
///     fn type_ident() -> &'static str { "Foo" }
///     fn module_path() -> Option<&'static str> { Some("my_crate::foo") }
/// }
/// ```
///
/// [`type_path`]: TypePath::type_path
/// [`type_name`]: TypePath::type_name
/// [`type_ident`]: TypePath::type_ident
/// [`module_path`]: TypePath::module_path
/// [`GenericTypePathCell`]: crate::impls::GenericTypePathCell
pub trait TypePath: 'static {
    /// Returns the fully qualified path with generics of the underlying type.
    ///
    /// This is the complete identifier of a type,
    /// and different types should **not** have the same type path
    ///
    /// For `Option<Vec<usize>>`, this is `"core::option::Option<alloc::vec::Vec<usize>>"`.
    fn type_path() -> &'static str;

    /// Returns a short, pretty-print enabled path to the type.
    ///
    /// Note that this is different from [`core::any::type_name`].
    ///
    /// For `Option<Vec<usize>>`, this is `"Option<Vec<usize>>"`.
    fn type_name() -> &'static str;

    /// Returns the short name of the type, without generics.
    ///
    /// For `Option<Vec<usize>>`, this is `"Option"`.
    fn type_ident() -> &'static str;

    /// Optional module path where the type is defined.
    ///
    /// Primitive built-in types may return `None`.
    fn module_path() -> Option<&'static str> {
        None
    }
}

/// Provides dynamic dispatch for types that implement [`TypePath`].
///
/// Auto impl for all types that implemented [`TypePath`].
pub trait DynamicTypePath {
    /// Returns the fully qualified path with generics of the underlying type.
    ///
    /// See [`TypePath::type_path`].
    fn reflect_type_path(&self) -> &'static str;

    /// Returns a short, pretty-print enabled path to the type.
    ///
    /// See [`TypePath::type_name`].
    fn reflect_type_name(&self) -> &'static str;

    /// Returns the short name of the type, without generics.
    ///
    /// See [`TypePath::type_ident`].
    fn reflect_type_ident(&self) -> &'static str;

    /// Optional module path where the type is defined.
    ///
    /// See [`TypePath::module_path`].
    fn reflect_module_path(&self) -> Option<&'static str>;
}

impl<T: TypePath> DynamicTypePath for T {
    #[inline]
    fn reflect_type_path(&self) -> &'static str {
        Self::type_path()
    }

    #[inline]
    fn reflect_type_name(&self) -> &'static str {
        Self::type_name()
    }

    #[inline]
    fn reflect_type_ident(&self) -> &'static str {
        Self::type_ident()
    }

    #[inline]
    fn reflect_module_path(&self) -> Option<&'static str> {
        Self::module_path()
    }
}

/// Lightweight vtable providing dynamic access to [`TypePath`] APIs.
///
/// This struct stores function pointers to a type's `TypePath` implementations,
/// keeping initialization minimal for types that are rarely queried.
#[derive(Clone, Copy)]
pub struct TypePathTable {
    type_path: fn() -> &'static str,
    type_name: fn() -> &'static str,
    type_ident: fn() -> &'static str,
    module_path: fn() -> Option<&'static str>,
}

impl TypePathTable {
    /// Creates a new table from a type.
    #[inline]
    pub const fn of<T: TypePath + ?Sized>() -> Self {
        Self {
            type_path: T::type_path,
            type_name: T::type_name,
            type_ident: T::type_ident,
            module_path: T::module_path,
        }
    }

    /// See [`TypePath::type_path`]
    #[inline(always)]
    pub fn path(&self) -> &'static str {
        (self.type_path)()
    }

    /// See [`TypePath::type_name`]
    #[inline(always)]
    pub fn name(&self) -> &'static str {
        (self.type_name)()
    }

    /// See [`TypePath::type_ident`]
    #[inline(always)]
    pub fn ident(&self) -> &'static str {
        (self.type_ident)()
    }

    /// See [`TypePath::module_path`]
    #[inline(always)]
    pub fn module_path(&self) -> Option<&'static str> {
        (self.module_path)()
    }

    /// Parse `crate_name` from `module_path`.
    #[inline(never)]
    pub fn crate_name(&self) -> Option<&'static str> {
        let s = (self.module_path)()?;
        match s.find(':') {
            Some(index) => Some(&s[0..index]),
            None => Some(s),
        }
    }
}

impl core::fmt::Debug for TypePathTable {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_struct("TypePathTable")
            .field("type_path", &self.path())
            .field("type_name", &self.name())
            .field("type_ident", &self.ident())
            .field("module_path", &self.module_path())
            .field("crate_name", &self.crate_name())
            .finish()
    }
}

/// The base representation of a Rust type.
///
/// Includes a [`TypeId`] and a [`TypePathTable`].
#[derive(Copy, Clone)]
pub struct Type {
    type_path_table: TypePathTable,
    type_id: TypeId,
}

impl Type {
    /// Creates a new [`Type`] from a type that implements [`TypePath`].
    #[inline]
    pub const fn of<T: TypePath + ?Sized>() -> Self {
        Self {
            type_path_table: TypePathTable::of::<T>(),
            type_id: TypeId::of::<T>(),
        }
    }

    /// Returns the [`TypeId`] of the type.
    #[inline(always)]
    pub const fn id(&self) -> TypeId {
        self.type_id
    }

    /// Check if the given type matches this one.
    ///
    /// This only compares the [`TypeId`] of the types.
    #[inline(always)]
    pub fn is<T: Any>(&self) -> bool {
        TypeId::of::<T>() == self.type_id
    }

    /// Returns the [`TypePathTable`] of the type.
    ///
    /// It is usually recommended to directly use the re-export methos by [`Type`].
    ///
    /// Unless it is necessary to copy the TypePathTable`
    #[inline(always)]
    pub const fn path_table(&self) -> &TypePathTable {
        &self.type_path_table
    }

    /// See [`TypePath::type_path`].
    #[inline]
    pub fn path(&self) -> &'static str {
        self.type_path_table.path()
    }

    /// See [`TypePath::type_name`].
    #[inline]
    pub fn name(&self) -> &'static str {
        self.type_path_table.name()
    }

    /// See [`TypePath::type_ident`].
    #[inline]
    pub fn ident(&self) -> &'static str {
        self.type_path_table.ident()
    }

    /// See [`TypePath::module_path`].
    #[inline]
    pub fn module_path(&self) -> Option<&'static str> {
        self.type_path_table.module_path()
    }

    /// Parse `crate_name` from `module_path`.
    #[inline]
    pub fn crate_name(&self) -> Option<&'static str> {
        self.type_path_table.crate_name()
    }
}

/// This implementation purely relies on the [`TypeId`] of the type,
impl PartialEq for Type {
    #[inline]
    fn eq(&self, other: &Self) -> bool {
        self.type_id == other.type_id
    }
}

impl Eq for Type {}

/// This implementation purely relies on the [`TypeId`] of the type,
impl core::hash::Hash for Type {
    #[inline]
    fn hash<H: core::hash::Hasher>(&self, state: &mut H) {
        self.type_id.hash(state);
    }
}

/// This implementation will only output the [`TypePath`] of the type.
impl core::fmt::Debug for Type {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.write_str(self.path())
    }
}

macro_rules! impl_type_fn {
    ($field:ident) => {
        /// Returns the underlying `Type`.
        #[inline(always)]
        pub const fn ty(&self) -> &$crate::info::Type {
            &self.$field
        }
        $crate::info::impl_type_fn!();
    };
    ($self:ident => $expr:expr) => {
        /// Returns the underlying `Type`.
        #[inline(never)]
        pub const fn ty($self: &Self) -> &$crate::info::Type {
            $expr
        }
        $crate::info::impl_type_fn!();
    };
    () => {
        /// Returns the `TypePathTable`.
        #[inline]
        pub const fn type_path_table(&self) -> &$crate::info::TypePathTable {
            &self.ty().path_table()
        }

        /// Returns the `TypeId`.
        #[inline]
        pub const fn ty_id(&self) -> ::core::any::TypeId {
            self.ty().id()
        }

        /// Check if the given type matches this one.
        #[inline]
        pub fn type_is<T: ::core::any::Any>(&self) -> bool {
            self.ty().id() == ::core::any::TypeId::of::<T>()
        }

        /// Returns the type path.
        #[inline]
        pub fn type_path(&self) -> &'static str {
            self.ty().path()
        }

        /// Returns the type name.
        #[inline]
        pub fn type_name(&self) -> &'static str {
            self.ty().name()
        }

        /// Returns the type ident.
        #[inline]
        pub fn type_ident(&self) -> &'static str {
            self.ty().ident()
        }

        /// Returns the module path.
        #[inline]
        pub fn module_path(&self) -> Option<&'static str> {
            self.ty().module_path()
        }

        /// Returns the crate name.
        #[inline]
        pub fn crate_name(&self) -> Option<&'static str> {
            self.ty().crate_name()
        }
    };
}

pub(crate) use impl_type_fn;
