use crate::{Reflect, ops::ReflectRef};
use alloc::boxed::Box;

/// A trait that enables types to be dynamically constructed from reflected data.
///
/// It's recommended to use the [derive macro] rather than manually implementing this trait.
///
/// `FromReflect` allows dynamic proxy types, like [`DynamicStruct`], to be used to generate
/// their concrete counterparts.
/// It can also be used to partially or fully clone a type (depending on whether it has
/// ignored fields or not).
///
/// In some cases, this trait may even be required.
/// Deriving [`Reflect`] on an enum requires all its fields to implement `FromReflect`.
/// Additionally, some complex types like `Vec<T>` require that their element types
/// implement this trait.
/// The reason for such requirements is that some operations require new data to be constructed,
/// such as swapping to a new variant or pushing data to a homogeneous list.
///
/// See the [crate-level documentation] to see how this trait can be used.
///
/// [derive macro]: crate::FromReflect
/// [`DynamicStruct`]: crate::ops::DynamicStruct
/// [crate-level documentation]: crate
#[diagnostic::on_unimplemented(
    message = "`{Self}` does not implement `FromReflect` so cannot be created through reflection",
    note = "consider annotating `{Self}` with `#[derive(Reflect)]`"
)]
pub trait FromReflect: Reflect + Sized {
    /// Constructs a concrete instance of `Self` from a reflected value.
    fn from_reflect(reflect: &dyn Reflect) -> Option<Self>;

    /// Attempts to downcast the given value to `Self`; if that fails, try to construct
    /// the value using [`FromReflect::from_reflect`].
    fn take_from_reflect(reflect: Box<dyn Reflect>) -> Result<Self, Box<dyn Reflect>> {
        if reflect.is::<Self>() {
            // TODO: use `dowmcast_unchecked` when stablized
            #[expect(unsafe_code, reason = "already checked")]
            Ok(unsafe { *reflect.downcast::<Self>().unwrap_unchecked() })
        } else {
            match Self::from_reflect(reflect.as_ref()) {
                Some(success) => Ok(success),
                None => Err(reflect),
            }
        }
    }
}

impl FromReflect for crate::ops::DynamicStruct {
    fn from_reflect(reflect: &dyn Reflect) -> Option<Self> {
        if let ReflectRef::Struct(val) = reflect.reflect_ref() {
            Some(val.to_dynamic_struct())
        } else {
            None
        }
    }
}

impl FromReflect for crate::ops::DynamicTupleStruct {
    fn from_reflect(reflect: &dyn Reflect) -> Option<Self> {
        if let ReflectRef::TupleStruct(val) = reflect.reflect_ref() {
            Some(val.to_dynamic_tuple_struct())
        } else {
            None
        }
    }
}

impl FromReflect for crate::ops::DynamicTuple {
    fn from_reflect(reflect: &dyn Reflect) -> Option<Self> {
        if let ReflectRef::Tuple(val) = reflect.reflect_ref() {
            Some(val.to_dynamic_tuple())
        } else {
            None
        }
    }
}

impl FromReflect for crate::ops::DynamicArray {
    fn from_reflect(reflect: &dyn Reflect) -> Option<Self> {
        if let ReflectRef::Array(val) = reflect.reflect_ref() {
            Some(val.to_dynamic_array())
        } else {
            None
        }
    }
}

impl FromReflect for crate::ops::DynamicList {
    fn from_reflect(reflect: &dyn Reflect) -> Option<Self> {
        if let ReflectRef::List(val) = reflect.reflect_ref() {
            Some(val.to_dynamic_list())
        } else {
            None
        }
    }
}

impl FromReflect for crate::ops::DynamicMap {
    fn from_reflect(reflect: &dyn Reflect) -> Option<Self> {
        if let ReflectRef::Map(val) = reflect.reflect_ref() {
            Some(val.to_dynamic_map())
        } else {
            None
        }
    }
}

impl FromReflect for crate::ops::DynamicSet {
    fn from_reflect(reflect: &dyn Reflect) -> Option<Self> {
        if let ReflectRef::Set(val) = reflect.reflect_ref() {
            Some(val.to_dynamic_set())
        } else {
            None
        }
    }
}

impl FromReflect for crate::ops::DynamicEnum {
    fn from_reflect(reflect: &dyn Reflect) -> Option<Self> {
        if let ReflectRef::Enum(val) = reflect.reflect_ref() {
            Some(val.to_dynamic_enum())
        } else {
            None
        }
    }
}
