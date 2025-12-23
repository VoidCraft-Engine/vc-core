use crate::Reflect;
use alloc::boxed::Box;

/// A trait that enables types to be dynamically constructed from reflected data.
///
/// Types that support `Reflect` should also implement this trait.
///
/// ## Warning
///
/// The implementation of `FromReflect` cannot rely on its own [`Reflect::try_apply`] or [`Reflect::apply`]。
/// The reason is that the Enum `try_apply` relies on its own `FromReflect` and needs to avoid self looping.
///
/// However, it is possible to rely on the `try_apply` of the subfield,
/// and in this case, type transformation usually does not have a dead loop.
///
/// There is no loop problem with the implementation of types other than enumeration,
/// but it is recommended to treat them equally.
///
/// It is usually recommended to use independent implementations for both.
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

// impl FromReflect for crate::ops::DynamicStruct {
//     fn from_reflect(reflect: &dyn Reflect) -> Option<Self> {
//         if let ReflectRef::Struct(val) = reflect.reflect_ref() {
//             Some(val.to_dynamic_struct())
//         } else {
//             None
//         }
//     }
// }

// impl FromReflect for crate::ops::DynamicTupleStruct {
//     fn from_reflect(reflect: &dyn Reflect) -> Option<Self> {
//         if let ReflectRef::TupleStruct(val) = reflect.reflect_ref() {
//             Some(val.to_dynamic_tuple_struct())
//         } else {
//             None
//         }
//     }
// }

// impl FromReflect for crate::ops::DynamicTuple {
//     fn from_reflect(reflect: &dyn Reflect) -> Option<Self> {
//         if let ReflectRef::Tuple(val) = reflect.reflect_ref() {
//             Some(val.to_dynamic_tuple())
//         } else {
//             None
//         }
//     }
// }

// impl FromReflect for crate::ops::DynamicArray {
//     fn from_reflect(reflect: &dyn Reflect) -> Option<Self> {
//         if let ReflectRef::Array(val) = reflect.reflect_ref() {
//             Some(val.to_dynamic_array())
//         } else {
//             None
//         }
//     }
// }

// impl FromReflect for crate::ops::DynamicList {
//     fn from_reflect(reflect: &dyn Reflect) -> Option<Self> {
//         if let ReflectRef::List(val) = reflect.reflect_ref() {
//             Some(val.to_dynamic_list())
//         } else {
//             None
//         }
//     }
// }

// impl FromReflect for crate::ops::DynamicMap {
//     fn from_reflect(reflect: &dyn Reflect) -> Option<Self> {
//         if let ReflectRef::Map(val) = reflect.reflect_ref() {
//             Some(val.to_dynamic_map())
//         } else {
//             None
//         }
//     }
// }

// impl FromReflect for crate::ops::DynamicSet {
//     fn from_reflect(reflect: &dyn Reflect) -> Option<Self> {
//         if let ReflectRef::Set(val) = reflect.reflect_ref() {
//             Some(val.to_dynamic_set())
//         } else {
//             None
//         }
//     }
// }

// impl FromReflect for crate::ops::DynamicEnum {
//     fn from_reflect(reflect: &dyn Reflect) -> Option<Self> {
//         if let ReflectRef::Enum(val) = reflect.reflect_ref() {
//             Some(val.to_dynamic_enum())
//         } else {
//             None
//         }
//     }
// }
