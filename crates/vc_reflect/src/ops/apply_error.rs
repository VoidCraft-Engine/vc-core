use alloc::borrow::Cow;
use core::{error, fmt};

use crate::info::{ReflectKind, ReflectKindError};

/// A enumeration of all error outcomes
/// that might happen when running [`try_apply`](crate::Reflect::try_apply).
#[derive(Debug)]
pub enum ApplyError {
    /// Special reflection type, not allowed to apply.
    NotSupport { type_path: Cow<'static, str> },
    /// Tried to apply incompatible types.
    MismatchedTypes {
        from_type: Cow<'static, str>,
        to_type: Cow<'static, str>,
    },
    /// Attempted to apply the wrong [kind](ReflectKind) to a type, e.g. a struct to an enum.
    MismatchedKinds {
        from_kind: ReflectKind,
        to_kind: ReflectKind,
    },
    /// Attempted to apply an array or tuple like type to another of different size, e.g. a `[u8; 4]` to `[u8; 3]`.
    DifferentSize { from_size: usize, to_size: usize },
    /// The enum we tried to apply to didn't contain a variant with the give name.
    UnknownVariant {
        enum_name: Cow<'static, str>,
        variant_name: Cow<'static, str>,
    },
}

impl fmt::Display for ApplyError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::NotSupport { type_path } => {
                write!(f, "type `{type_path}` does not support `apply`")
            }
            Self::MismatchedTypes { from_type, to_type } => {
                write!(f, "attempted to apply `{from_type}` to `{to_type}`")
            }
            Self::MismatchedKinds { from_kind, to_kind } => {
                write!(f, "attempted to apply `{from_kind}` to `{to_kind}`")
            }
            Self::DifferentSize { from_size, to_size } => {
                write!(
                    f,
                    "attempted to apply type with {from_size} size to {to_size} size"
                )
            }
            Self::UnknownVariant {
                enum_name,
                variant_name,
            } => {
                write!(
                    f,
                    "variant `{variant_name}` does not exist on enum `{enum_name}`"
                )
            }
        }
    }
}

impl error::Error for ApplyError {}

impl From<ReflectKindError> for ApplyError {
    #[inline]
    fn from(value: ReflectKindError) -> Self {
        Self::MismatchedKinds {
            from_kind: value.received,
            to_kind: value.expected,
        }
    }
}
