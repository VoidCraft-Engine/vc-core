use core::any::{Any, TypeId};

use alloc::boxed::Box;

use crate::{
    impls::NonGenericTypeInfoCell,
    info::{DynamicTypePath, DynamicTyped, OpaqueInfo, ReflectKind, TypeInfo, TypePath, Typed},
    ops::{ApplyError, ReflectCloneError, ReflectMut, ReflectOwned, ReflectRef},
};

pub trait Reflect: DynamicTypePath + DynamicTyped + Send + Sync + Any {
    /// Casts this type to a fully-reflected value.
    #[inline(always)]
    fn as_reflect(&self) -> &dyn Reflect
    where
        Self: Sized,
    {
        self
    }

    /// Casts this type to a mutable, fully-reflected value.
    #[inline(always)]
    fn as_reflect_mut(&mut self) -> &mut dyn Reflect
    where
        Self: Sized,
    {
        self
    }

    /// Casts this type to a boxed, fully-reflected value.
    #[inline(always)]
    fn into_reflect(self: Box<Self>) -> Box<dyn Reflect>
    where
        Self: Sized,
    {
        self
    }

    /// Return the [`TypeId`] of underlying type.
    ///
    /// Used to replace [`Any::type_id`].
    ///
    /// When you use `Box<dyn Reflect>::type_id`, it will return
    /// the [`TypeId`] of the entire container, instead of `dyn Reflect`.
    ///
    /// This is prone to errors, so we provide this method.
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
    #[inline]
    fn represented_type_info(&self) -> Option<&'static TypeInfo> {
        Some(self.reflect_type_info())
    }

    /// Performs a type-checked assignment of a reflected value to this value.
    ///
    /// This is type strict but fast; to allow compatible-but-not-identical inputs,
    /// use [`Reflect::try_apply`].
    fn set(&mut self, value: Box<dyn Reflect>) -> Result<(), Box<dyn Reflect>>;

    /// Returns a zero-sized enumeration of "kinds" of type.
    fn reflect_kind(&self) -> ReflectKind;
    // Normal impl: fn (&self)-> ReflectKind{ ReflectKind::??? }

    /// Returns an immutable enumeration of "kinds" of type.
    fn reflect_ref(&self) -> ReflectRef<'_>;
    // Normal impl: fn (&self)-> ReflectRef{ ReflectRef::???::(self) }

    /// Returns a mutable enumeration of "kinds" of type.
    fn reflect_mut(&mut self) -> ReflectMut<'_>;
    // Normal impl: fn (&mut self)-> ReflectMut{ ReflectMut::???::(self) }

    /// Returns an owned enumeration of "kinds" of type.
    fn reflect_owned(self: Box<Self>) -> ReflectOwned;
    // Normal impl: fn (self: Box<Self>)-> ReflectOwned{ ReflectOwned::???::(self) }

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

    fn try_apply(&mut self, _value: &dyn Reflect) -> Result<(), ApplyError>;

    #[inline]
    fn apply(&mut self, value: &dyn Reflect) {
        Reflect::try_apply(self, value).unwrap();
    }

    /// Attempts to clone `Self` using reflection.
    ///
    /// Unlike [`Reflect::to_dynamic`], which generally returns a dynamic representation of `Self`,
    /// this method attempts to create a clone of `Self` directly, if possible.
    fn reflect_clone(&self) -> Result<Box<dyn Reflect>, ReflectCloneError>;

    /// Returns a "partial equality" comparison result.
    ///
    /// If the underlying type does not support equality testing, returns `None`.
    #[inline]
    fn reflect_partial_eq(&self, _other: &dyn Reflect) -> Option<bool> {
        // Only Inline for default implement
        None
    }

    /// Returns a hash of the value (which includes the type).
    ///
    /// If the underlying type does not support hashing, returns `None`.
    ///
    /// The return value of this implementation may differ from [`core::hash::Hash`].
    #[inline]
    fn reflect_hash(&self) -> Option<u64> {
        None
    }

    fn reflect_debug(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        use crate::impls;
        match self.reflect_ref() {
            ReflectRef::Struct(dyn_struct) => impls::struct_debug(dyn_struct, f),
            ReflectRef::TupleStruct(dyn_tuple_struct) => {
                impls::tuple_struct_debug(dyn_tuple_struct, f)
            }
            ReflectRef::Tuple(dyn_tuple) => impls::tuple_debug(dyn_tuple, f),
            ReflectRef::List(dyn_list) => impls::list_debug(dyn_list, f),
            ReflectRef::Array(dyn_array) => impls::array_debug(dyn_array, f),
            ReflectRef::Map(dyn_map) => impls::map_debug(dyn_map, f),
            ReflectRef::Set(dyn_set) => impls::set_debug(dyn_set, f),
            ReflectRef::Enum(dyn_enum) => impls::enum_debug(dyn_enum, f),
            ReflectRef::Opaque(_) => write!(f, "Opaque({})", self.reflect_type_path()),
        }
    }
}

impl dyn Reflect {
    /// Returns `true` if the underlying value is of type `T`.
    #[inline(always)]
    pub fn is<T: Any>(&self) -> bool {
        // Any::Type_id(self)
        self.ty_id() == TypeId::of::<T>()
    }

    /// Downcasts the value to type `T` by reference.
    ///
    /// If the underlying value is not of type `T`, returns `None`.
    #[inline]
    pub fn downcast_ref<T: Any>(&self) -> Option<&T> {
        <dyn Any>::downcast_ref(self)
    }

    /// Downcasts the value to type `T` by mutable reference.
    ///
    /// If the underlying value is not of type `T`, returns `None`.
    #[inline]
    pub fn downcast_mut<T: Any>(&mut self) -> Option<&mut T> {
        <dyn Any>::downcast_mut(self)
    }

    /// Downcasts the value to type `T`, consuming the trait object.
    ///
    /// If the underlying value is not of type `T`, returns `Err(self)`.
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
            // Manually inline
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
