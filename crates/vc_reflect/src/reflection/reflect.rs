use core::any::{Any, TypeId};

use alloc::boxed::Box;

use crate::{
    impls::NonGenericTypeInfoCell,
    info::{DynamicTypePath, DynamicTyped, OpaqueInfo, ReflectKind, TypeInfo, TypePath, Typed},
    ops::{ApplyError, ReflectCloneError, ReflectMut, ReflectOwned, ReflectRef},
};

/// The foundational trait of [`vc_reflect`], used for accessing and modifying data dynamically.
///
/// It's recommended to use [the derive macro for `Reflect`] rather than manually implementing this trait.
/// Doing so will automatically implement this trait as well as many other useful traits for reflection,
/// including one of the appropriate subtraits: [`Struct`], [`TupleStruct`] or [`Enum`].
///
/// # Methods
///
/// [`Reflect`] is a subtrait of [`DynamicTypePath`] and [`DynamicTyped`], so you can call their methods:
///
/// ```
/// # use vc_reflect::{Reflect, info::{DynamicTypePath, DynamicTyped}};
/// let value = 10i32.into_boxed_reflect();
/// let type_path = value.reflect_type_path();
/// let type_info = value.reflect_type_info();
/// ```
///
/// This also supports [`Any`], but [`Any::type_id`] has a problem,
/// For `Box<dyn Reflect>`, it may return the container type instead of inner value type.
///
/// Therefore, we provided a [`ty_id`] method:
///
/// ```
/// # use vc_reflect::Reflect;
/// # use core::any::{Any, TypeId};
/// let mut x: Box<dyn Reflect> = Box::new(32_i32).into_reflect();
///
/// assert!(x.type_id() != TypeId::of::<i32>()); // !!!
/// assert!((*x).type_id() == TypeId::of::<i32>());
/// assert!(x.ty_id() == TypeId::of::<i32>());   // good
/// ```
///
/// The type_info of the dynamic type is `OpaqueInfo`, so we provided a [`represented_type_info`]
/// method, it returns information represented by a dynamic type.
/// For other type, it's equal to [`reflect_type_info`].
///
/// ```
/// # use vc_reflect::{ops::{Array, DynamicArray}, Reflect};
/// let info = [1_i32, 2, 3, 4, 5].to_dynamic_array().represented_type_info().unwrap();
/// assert!(info.type_is::<[i32;5]>());
/// ```
///
/// Use [`reflect_ref`], [`reflect_mut`], [`reflect_owned`] to convert itself to a subtype
/// (e.g. [`Struct`], [`TupleStruct`]) .
///
/// ```
/// # use vc_reflect::{Reflect, ops::List};
/// let vec = vec![1, 2, 3].into_boxed_reflect();
/// let vec: Box<dyn List> = vec.reflect_owned().into_list().unwrap();
/// ```
///
/// Use `downcast_ref`, `downcast_mut`, `downcast` to convert itself to a specific type.
///
/// ```
/// # use vc_reflect::Reflect;
/// let x: Box<dyn Reflect> = 10.into_boxed_reflect();
/// let y = x.downcast_ref::<i32>().unwrap();
/// assert_eq!(*y, 10);
/// ```
///
/// See more examples in [`reflect_clone`], [`try_apply`],  [`to_dynamic`].
///
/// See the [crate-level documentation] to see how this trait and its subtraits can be used.
///
/// [`reflect_clone`]: Reflect::reflect_clone
/// [`try_apply`]: Reflect::try_apply
/// [`to_dynamic`]: Reflect::to_dynamic
/// [`reflect_ref`]: Reflect::reflect_ref
/// [`reflect_mut`]: Reflect::reflect_mut
/// [`reflect_owned`]: Reflect::reflect_owned
/// [`reflect_type_info`]: crate::info::DynamicTyped::reflect_type_info
/// [`represented_type_info`]: Reflect::represented_type_info
/// [`ty_id`]: Reflect::ty_id
/// [`vc_reflect`]: crate
/// [the derive macro for `Reflect`]: crate::derive::Reflect
/// [`Struct`]: crate::ops::Struct
/// [`TupleStruct`]: crate::ops::TupleStruct
/// [`Enum`]: crate::ops::Enum
/// [crate-level documentation]: crate
pub trait Reflect: DynamicTypePath + DynamicTyped + Send + Sync + Any {
    /// Casts this type to a fully-reflected value.
    ///
    /// # Example
    ///
    /// ```
    /// use vc_reflect::Reflect;
    ///
    /// let x = 32;
    /// let r: &dyn Reflect = x.as_reflect();
    /// // Equal to this:
    /// // let r: &dyn Reflect = &x;
    /// ```
    #[inline(always)]
    fn as_reflect(&self) -> &dyn Reflect
    where
        Self: Sized,
    {
        self
    }

    /// Casts this type to a mutable, fully-reflected value.
    ///
    /// # Example
    ///
    /// ```
    /// use vc_reflect::Reflect;
    ///
    /// let mut x = 32;
    /// let r: &mut dyn Reflect = x.as_reflect_mut();
    /// // Equal to this:
    /// // let r: &mut dyn Reflect = &mut x;
    /// ```
    #[inline(always)]
    fn as_reflect_mut(&mut self) -> &mut dyn Reflect
    where
        Self: Sized,
    {
        self
    }

    /// Casts this type to a boxed, fully-reflected value.
    ///
    /// # Example
    ///
    /// ```
    /// use vc_reflect::Reflect;
    ///
    /// let mut x = Box::new(32);
    /// let r = x.into_reflect();
    /// // Equal to this:
    /// // let r = x as Box<dyn Reflect>;
    /// ```
    #[inline(always)]
    fn into_reflect(self: Box<Self>) -> Box<dyn Reflect>
    where
        Self: Sized,
    {
        self
    }

    /// Casts this type to a boxed, fully-reflected value.
    ///
    /// # Example
    ///
    /// ```
    /// use vc_reflect::Reflect;
    ///
    /// let r = 32.into_boxed_reflect();
    /// // Equal to this:
    /// // let r = Box::new(32) as Box<dyn Reflect>;
    /// ```
    #[inline(always)]
    fn into_boxed_reflect(self) -> Box<dyn Reflect>
    where
        Self: Sized,
    {
        Box::new(self)
    }

    /// Return the [`TypeId`] of underlying type.
    ///
    /// When you call `Box<dyn Reflect>::type_id`, it will return
    /// the [`TypeId`] of the entire container, instead of `dyn Reflect`.
    ///
    /// This is prone to errors, so we provide this method.
    ///
    /// # Example
    ///
    /// ```
    /// use vc_reflect::Reflect;
    /// use core::any::{Any, TypeId};
    ///
    /// let mut x: Box<dyn Reflect> = Box::new(32_i32).into_reflect();
    ///
    /// assert!(x.type_id() != TypeId::of::<i32>()); // !!!
    /// assert!((*x).type_id() == TypeId::of::<i32>());
    /// assert!(x.ty_id() == TypeId::of::<i32>());   // good
    /// ```
    #[inline]
    fn ty_id(&self) -> TypeId {
        TypeId::of::<Self>()
    }

    /// Indicates whether or not this type is a _dynamic_ data type.
    ///
    /// Normally, All other types should return false,
    /// meaning there is no need to implement it.
    #[inline]
    fn is_dynamic(&self) -> bool {
        false
    }

    /// Returns the [`TypeInfo`] of the type **represented** by this value.
    ///
    /// For most types, this will simply return their own [`TypeInfo`], as same as [`reflect_type_info`].
    ///
    /// However, for dynamic types, such as [`DynamicStruct`] or [`DynamicList`],
    /// this will return the type they represent
    /// (or `None` if they don't represent any particular type).
    ///
    /// [`reflect_type_info`]: crate::info::DynamicTyped
    /// [`DynamicStruct`]: crate::ops::DynamicStruct
    /// [`DynamicList`]: crate::ops::DynamicList
    /// [`TypeRegistry::get_type_info`]: crate::registry::TypeRegistry::get_type_info
    #[inline]
    fn represented_type_info(&self) -> Option<&'static TypeInfo> {
        Some(self.reflect_type_info())
    }

    /// Performs a type-checked assignment of a reflected value to this value.
    ///
    /// This is type strict but fast; to allow compatible-but-not-identical inputs,
    /// use [`Reflect::try_apply`].
    ///
    /// # Examples
    ///
    /// ```
    /// # use vc_reflect::Reflect;
    /// let data = vec![1_i32, 2_i32, 3_i32].into_boxed_reflect();
    /// let mut vec = Vec::<i32>::new();
    ///
    /// vec.set(data);
    /// assert_eq!(vec, [1, 2, 3]);
    /// ```
    fn set(&mut self, value: Box<dyn Reflect>) -> Result<(), Box<dyn Reflect>>;

    /// Returns a pure enumeration of ["kinds"](ReflectKind) of type.
    ///
    /// # Examples
    ///
    /// ```
    /// # use vc_reflect::{Reflect, info::ReflectKind};
    /// let vec = vec![1, 2, 3].into_boxed_reflect();
    ///
    /// assert_eq!(vec.reflect_kind(), ReflectKind::List);
    /// ```
    fn reflect_kind(&self) -> ReflectKind;

    /// Returns an immutable enumeration of ["kinds"](ReflectRef) of type.
    ///
    /// # Examples
    ///
    /// ```
    /// # use vc_reflect::{Reflect, ops::List};
    /// let vec = vec![1, 2, 3].into_boxed_reflect();
    ///
    /// let vec_mut: &dyn List = vec.reflect_ref().as_list().unwrap();
    /// ```
    fn reflect_ref(&self) -> ReflectRef<'_>;

    /// Returns a mutable enumeration of ["kinds"](ReflectMut) of type.
    ///
    /// # Examples
    ///
    /// ```
    /// # use vc_reflect::{Reflect, ops::List};
    /// let mut vec = vec![1, 2, 3].into_boxed_reflect();
    ///
    /// let vec_mut: &mut dyn List = vec.reflect_mut().as_list().unwrap();
    /// ```
    fn reflect_mut(&mut self) -> ReflectMut<'_>;

    /// Returns an owned enumeration of ["kinds"](ReflectOwned) of type.
    ///
    /// # Examples
    ///
    /// ```
    /// # use vc_reflect::{Reflect, ops::List};
    /// let vec = vec![1, 2, 3].into_boxed_reflect();
    ///
    /// let vec: Box<dyn List> = vec.reflect_owned().into_list().unwrap();
    /// ```
    fn reflect_owned(self: Box<Self>) -> ReflectOwned;

    /// Converts this reflected value into its dynamic representation based on its [kind].
    ///
    /// For example, a [`List`] type will internally invoke [`List::to_dynamic_list`], returning [`DynamicList`].
    /// A [`Struct`] type will invoke [`Struct::to_dynamic_struct`], returning [`DynamicStruct`].
    /// And so on.
    ///
    /// If the [kind] is [opaque], then the value will attempt to be cloned directly via [`reflect_clone`],
    /// since opaque types do not have any standard dynamic representation.
    ///
    /// To attempt to clone the value directly such that it returns a concrete instance of this type,
    /// use [`reflect_clone`].
    ///
    /// # Panics
    ///
    /// This method will panic if the [kind] is [opaque] and the call to [`reflect_clone`] fails.
    ///
    /// # Example
    ///
    /// ```
    /// # use vc_reflect::Reflect;
    /// let value = (1, true, 3.14);
    /// let dynamic_value = value.to_dynamic();
    /// assert!(dynamic_value.is_dynamic())
    /// ```
    ///
    /// [kind]: Reflect::reflect_kind
    /// [`List`]: crate::ops::List
    /// [`List::to_dynamic_list`]: crate::ops::List::to_dynamic_list
    /// [`DynamicList`]: crate::ops::DynamicList
    /// [`Struct`]: crate::ops::Struct
    /// [`Struct::to_dynamic_struct`]: crate::ops::Struct::to_dynamic_struct
    /// [`DynamicStruct`]: crate::ops::DynamicStruct
    /// [opaque]: crate::info::ReflectKind::Opaque
    /// [`reflect_clone`]: Reflect::reflect_clone
    fn to_dynamic(&self) -> Box<dyn Reflect> {
        match self.reflect_ref() {
            ReflectRef::Struct(val) => Box::new(val.to_dynamic_struct()),
            ReflectRef::TupleStruct(val) => Box::new(val.to_dynamic_tuple_struct()),
            ReflectRef::Tuple(val) => Box::new(val.to_dynamic_tuple()),
            ReflectRef::List(val) => Box::new(val.to_dynamic_list()),
            ReflectRef::Array(val) => Box::new(val.to_dynamic_array()),
            ReflectRef::Map(val) => Box::new(val.to_dynamic_map()),
            ReflectRef::Set(val) => Box::new(val.to_dynamic_set()),
            ReflectRef::Enum(val) => Box::new(val.to_dynamic_enum()),
            ReflectRef::Opaque(val) => val.reflect_clone().unwrap_or_else(|_| {
                panic!(
                    "`Reflect::to_dynamic` failed because opaque type `{}` does not support `reflect_clone`.",
                    val.reflect_type_path()
                );
            }),
        }
    }

    /// Try applies a reflected value to this value.
    ///
    /// # Apply Rules
    ///
    /// If `self.ty_id` == `value.ty_id`:
    ///
    /// - If the type support `Clone`, try `downcast_ref` + [`Clone::clone`] + assignment.
    /// - Otherwise, try [`Reflect::reflect_clone`] + `Reflect::take` + assignment.
    ///
    /// Otherwise, call following method, depend on [`ReflectKind`]:
    ///
    /// - [`crate::impls::array_try_apply`]
    /// - [`crate::impls::list_try_apply`]
    /// - [`crate::impls::struct_try_apply`]
    /// - [`crate::impls::tuple_struct_try_apply`]
    /// - [`crate::impls::tuple_try_apply`]
    /// - [`crate::impls::enum_try_apply`]
    /// - [`crate::impls::set_try_apply`]
    /// - [`crate::impls::map_try_apply`]
    ///
    /// The only special kind is `Enum`, the same type but different variant
    /// cannot `try_apply` through `enum_try_apply` directly,
    /// The default implementation may depend on [`FromReflect`](crate::FromReflect).
    ///
    /// # Fail Reason
    /// - Defferent [`ReflectKind`].
    /// - Defferent Item/Field size in `Array`, `Tuple`, `TupleStruct` and `Enum`'s tuple variant.
    /// - Incompatible type in any try_apply.
    /// - Opaque type but do not support `Clone` or reflect clone.
    ///
    /// # Handling Errors
    ///
    /// This function may leave `self` in a partially mutated state if a error was encountered on the way.
    /// consider maintaining a cloned instance of this data you can switch to if a error is encountered.
    fn try_apply(&mut self, value: &dyn Reflect) -> Result<(), ApplyError>;

    /// Applies a reflected value to this value.
    ///
    /// This function is similar to `try_apply(..).unwrap()`.
    ///
    /// See more infomation in [`Reflect::try_apply`] .
    ///
    /// # Panics
    /// - Defferent [`ReflectKind`].
    /// - Defferent Item/Field size in `Array`, `Tuple`, `TupleStruct` and `Enum`'s tuple variant.
    /// - Incompatible type in any `try_apply`.
    /// - Opaque type but do not support `Clone` or reflect clone.
    #[inline]
    fn apply(&mut self, value: &dyn Reflect) {
        Reflect::try_apply(self, value).unwrap();
    }

    /// Attempts to clone `Self` using reflection.
    ///
    /// Unlike [`to_dynamic`], which generally returns a dynamic representation of `Self`,
    /// this method attempts create a clone of `Self` directly, if possible.
    ///
    /// This function normally succeeds, except for certain types that explicitly prohibit cloning.
    /// But if the clone cannot be performed, an appropriate [`ReflectCloneError`] is returned.
    ///
    /// # Example
    ///
    /// ```
    /// # use vc_reflect::Reflect;
    /// let value = (1, true, 3.14);
    /// let cloned = value.reflect_clone().unwrap();
    /// assert!(cloned.is::<(i32, bool, f64)>())
    /// ```
    ///
    /// When generating implementations via macros, opaque types are required to
    /// implement the [`Clone`] trait, making this operation infallible for them.
    ///
    /// For non-opaque types, this function performs a field-by-field `reflect_clone`
    /// by default. Therefore, it's generally recommended to implement [`Clone`]
    /// for your type and mark it with the `#[reflect(clone)]` attribute.
    /// When this is done, the function directly uses the trait implementation,
    /// guaranteeing success.
    ///
    /// ```
    /// use vc_reflect::derive::Reflect;
    ///
    /// #[derive(Reflect, Clone)]
    /// #[reflect(clone)]
    /// struct A { /* ... */ }
    /// ```
    ///
    /// [`to_dynamic`]: Reflect::to_dynamic
    fn reflect_clone(&self) -> Result<Box<dyn Reflect>, ReflectCloneError>;

    /// Returns a "partial equality" comparison result.
    ///
    /// If the underlying type does not support equality testing, returns `None`.
    ///
    /// In the default implementation, this always returns `None` for opaque types,
    /// while unit structs are compared by checking their type IDs directly.
    ///
    /// However, for composite types, this performs a field-by-field comparison
    /// using `reflect_partial_eq`, which may not be efficient.
    ///
    /// See:
    /// - [`crate::impls::array_partial_eq`]
    /// - [`crate::impls::list_partial_eq`]
    /// - [`crate::impls::struct_partial_eq`]
    /// - [`crate::impls::tuple_struct_partial_eq`]
    /// - [`crate::impls::tuple_partial_eq`]
    /// - [`crate::impls::enum_partial_eq`]
    /// - [`crate::impls::set_partial_eq`]
    /// - [`crate::impls::map_partial_eq`]
    ///
    /// If the type implements [`PartialEq`], consider marking it with the
    /// `#[reflect(partial_eq)]` attribute. When this attribute is present,
    /// the function uses the type's own implementation instead, and types that
    /// differ immediately return `Some(false)`.
    ///
    /// ```
    /// use vc_reflect::derive::Reflect;
    ///
    /// #[derive(Reflect, PartialEq)]
    /// #[reflect(partial_eq)]
    /// struct A { /* ... */ }
    /// ```
    #[inline]
    fn reflect_partial_eq(&self, _other: &dyn Reflect) -> Option<bool> {
        // Only Inline for default implement
        None
    }

    /// Returns a hash of the value, may differ from [`core::hash::Hash`].
    ///
    /// We use [`reflect_hasher`](crate::reflect_hasher) to ensure that the hash
    /// result of the program running multiple times is the same for the same data.
    ///
    /// In the default implementation, this always returns `None` for opaque types,
    /// while unit structs compute their hash by directly hashing the [`TypeId`].
    ///
    /// For composite types, however, this performs a field-by-field hash using
    /// `reflect_hash`, which may be inefficient.
    ///
    /// See:
    /// - [`crate::impls::array_hash`]
    /// - [`crate::impls::list_hash`]
    /// - [`crate::impls::struct_hash`]
    /// - [`crate::impls::tuple_struct_hash`]
    /// - [`crate::impls::tuple_hash`]
    /// - [`crate::impls::enum_hash`]
    /// - [`crate::impls::set_hash`]
    /// - [`crate::impls::map_hash`]
    ///
    /// If the type implements [`Hash`](core::hash::Hash), consider annotating it with the
    /// `#[reflect(hash)]` attribute to make this function use the [`Hash`](core::hash::Hash)'s
    /// implementation instead.
    ///
    /// ```
    /// use vc_reflect::derive::Reflect;
    ///
    /// #[derive(Reflect, Hash)]
    /// #[reflect(hash)]
    /// struct A { /* ... */ }
    /// ```
    #[inline]
    fn reflect_hash(&self) -> Option<u64> {
        None
    }

    /// Debug formatter for the value.
    ///
    /// For opaque type, this function will write `"Opaque(type_path)"` by default.
    ///
    /// For other type, see:
    /// - [`crate::impls::array_debug`]
    /// - [`crate::impls::list_debug`]
    /// - [`crate::impls::struct_debug`]
    /// - [`crate::impls::tuple_struct_debug`]
    /// - [`crate::impls::tuple_debug`]
    /// - [`crate::impls::enum_debug`]
    /// - [`crate::impls::set_debug`]
    /// - [`crate::impls::map_debug`]
    ///
    /// If the type implements [`Debug`](core::fmt::Debug), consider annotating it with the
    /// `#[reflect(debug)]` attribute to make this function use the [`Debug`](core::fmt::Debug)'s
    /// implementation instead.
    ///
    /// ```
    /// use vc_reflect::derive::Reflect;
    ///
    /// #[derive(Reflect, Debug)]
    /// #[reflect(debug)]
    /// struct A { /* ... */ }
    /// ```
    ///
    /// [`List`]: crate::ops::List
    /// [`Map`]: crate::ops::Map
    /// [type path]: TypePath::type_path
    fn reflect_debug(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        use crate::impls;
        match self.reflect_ref() {
            ReflectRef::Struct(data) => impls::struct_debug(data, f),
            ReflectRef::TupleStruct(data) => impls::tuple_struct_debug(data, f),
            ReflectRef::Tuple(data) => impls::tuple_debug(data, f),
            ReflectRef::List(data) => impls::list_debug(data, f),
            ReflectRef::Array(data) => impls::array_debug(data, f),
            ReflectRef::Map(data) => impls::map_debug(data, f),
            ReflectRef::Set(data) => impls::set_debug(data, f),
            ReflectRef::Enum(data) => impls::enum_debug(data, f),
            ReflectRef::Opaque(_) => write!(f, "Opaque({})", self.reflect_type_path()),
        }
    }
}

impl dyn Reflect {
    /// Returns `true` if the underlying value is of type `T`.
    ///
    /// # Examples
    ///
    /// ```
    /// # use vc_reflect::Reflect;
    /// let x: Box<dyn Reflect> = 10.into_boxed_reflect();
    ///
    /// assert!(x.is::<i32>());
    /// ```
    #[inline(always)]
    pub fn is<T: Any>(&self) -> bool {
        // Any::Type_id(self)
        self.ty_id() == TypeId::of::<T>()
    }

    /// Downcasts the value to type `T` by reference.
    ///
    /// If the underlying value is not of type `T`, returns `None`.
    ///
    /// # Examples
    ///
    /// ```
    /// # use vc_reflect::Reflect;
    /// let x: Box<dyn Reflect> = 10.into_boxed_reflect();
    ///
    /// let y = x.downcast_ref::<i32>().unwrap();
    /// assert_eq!(*y, 10);
    /// ```
    #[inline]
    pub fn downcast_ref<T: Any>(&self) -> Option<&T> {
        <dyn Any>::downcast_ref(self)
    }

    /// Downcasts the value to type `T` by mutable reference.
    ///
    /// If the underlying value is not of type `T`, returns `None`.
    ///
    /// # Examples
    ///
    /// ```
    /// # use vc_reflect::Reflect;
    /// let mut x: Box<dyn Reflect> = 10.into_boxed_reflect();
    ///
    /// let y = x.downcast_mut::<i32>().unwrap();
    /// *y += 2;
    ///
    /// assert_eq!(*y, 12);
    /// ```
    #[inline]
    pub fn downcast_mut<T: Any>(&mut self) -> Option<&mut T> {
        <dyn Any>::downcast_mut(self)
    }

    /// Downcasts the value to type `T`, consuming the trait object.
    ///
    /// If the underlying value is not of type `T`, returns `Err(self)`.
    ///
    /// # Examples
    ///
    /// ```
    /// # use vc_reflect::Reflect;
    /// let x: Box<dyn Reflect> = 10.into_boxed_reflect();
    ///
    /// let x: Box<i32> = x.downcast::<i32>().unwrap();
    /// assert_eq!(*x, 10);
    /// ```
    #[inline]
    pub fn downcast<T: Any>(self: Box<dyn Reflect>) -> Result<Box<T>, Box<dyn Reflect>> {
        if self.is::<T>() {
            // TODO: replace to `downcast_uncheck` when it's stable,
            #[expect(unsafe_code, reason = "type is already checked")]
            Ok(unsafe { <Box<dyn Any>>::downcast::<T>(self).unwrap_unchecked() })
        } else {
            Err(self)
        }
    }

    /// Downcasts the value to type `T`, unboxing and consuming the trait object.
    ///
    /// If the underlying value is not of type `T`, returns `Err(self)`.
    ///
    /// # Examples
    ///
    /// ```
    /// # use vc_reflect::Reflect;
    /// let x: Box<dyn Reflect> = 10.into_boxed_reflect();
    ///
    /// let x = x.take::<i32>().unwrap();
    /// assert_eq!(x, 10);
    /// ```
    #[inline]
    pub fn take<T: Any>(self: Box<dyn Reflect>) -> Result<T, Box<dyn Reflect>> {
        if self.is::<T>() {
            // TODO: replace to `downcast_uncheck` if it's unstable,
            #[expect(unsafe_code, reason = "type is already checked")]
            Ok(unsafe { *<Box<dyn Any>>::downcast::<T>(self).unwrap_unchecked() })
        } else {
            Err(self)
        }
    }
}

impl core::fmt::Debug for dyn Reflect {
    #[inline]
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        self.reflect_debug(f)
    }
}

impl TypePath for dyn Reflect {
    #[inline]
    fn type_path() -> &'static str {
        "dyn vc_reflect::Reflect"
    }
    #[inline]
    fn type_name() -> &'static str {
        "dyn Reflect"
    }
    #[inline]
    fn type_ident() -> &'static str {
        "dyn Reflect"
    }
}

impl Typed for dyn Reflect {
    /// This is the [`TypeInfo`] of [`dyn Reflect`],
    /// not the [`TypeInfo`] of the underlying data!!!!
    ///
    /// Use [`DynamicTyped::reflect_type_info`] to get underlying [`TypeInfo`].
    ///
    /// [`dyn Reflect`]: crate::Reflect
    fn type_info() -> &'static TypeInfo {
        static CELL: NonGenericTypeInfoCell = NonGenericTypeInfoCell::new();
        CELL.get_or_init(|| TypeInfo::Opaque(OpaqueInfo::new::<Self>()))
    }
}

macro_rules! impl_reflect_cast_fn {
    ($kind:ident) => {
        fn set(
            &mut self,
            value: ::alloc::boxed::Box<dyn $crate::Reflect>,
        ) -> Result<(), ::alloc::boxed::Box<dyn $crate::Reflect>> {
            *self = value.take::<Self>()?;
            Ok(())
        }

        #[inline]
        fn reflect_kind(&self) -> $crate::info::ReflectKind {
            $crate::info::ReflectKind::$kind
        }

        #[inline]
        fn reflect_ref(&self) -> $crate::ops::ReflectRef<'_> {
            $crate::ops::ReflectRef::$kind(self)
        }

        #[inline]
        fn reflect_mut(&mut self) -> $crate::ops::ReflectMut<'_> {
            $crate::ops::ReflectMut::$kind(self)
        }

        #[inline]
        fn reflect_owned(self: ::alloc::boxed::Box<Self>) -> $crate::ops::ReflectOwned {
            $crate::ops::ReflectOwned::$kind(self)
        }
    };
}

pub(crate) use impl_reflect_cast_fn;
