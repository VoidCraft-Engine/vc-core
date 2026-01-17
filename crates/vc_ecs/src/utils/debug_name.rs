use alloc::borrow::Cow;
use alloc::string::String;
use core::fmt;
use core::ops::Deref;

use vc_utils::extra::ShortName;

use crate::cfg;

/// Wrapper to help debugging ECS issues.
///
/// - If the `debug` feature is enabled or in `Debug` mode, the name will be used.
/// - If it is disabled, a string mentioning the disabled feature will be us.
#[derive(Clone, PartialEq, Eq)]
pub struct DebugName {
    #[cfg(any(debug_assertions, feature = "debug"))]
    name: Cow<'static, str>,
}

cfg::debug! {
    else { const FEATURE_DISABLED: &str = "Enable the debug feature to see the name"; }
}

impl Deref for DebugName {
    type Target = str;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        cfg::debug! {
            if { &self.name } else { FEATURE_DISABLED }
        }
    }
}

impl fmt::Display for DebugName {
    #[inline]
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        // Deref to `str`, which will use `FEATURE_DISABLED` if necessary
        write!(f, "{}", &**self)
    }
}

impl fmt::Debug for DebugName {
    #[inline]
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        // Deref to `str`, which will use `FEATURE_DISABLED` if necessary
        write!(f, "{:?}", &**self)
    }
}

impl From<Cow<'static, str>> for DebugName {
    #[inline(always)]
    fn from(_value: Cow<'static, str>) -> Self {
        cfg::debug! {
            if { Self{ name: _value } }
            else { Self{} }
        }
    }
}

impl From<DebugName> for Cow<'static, str> {
    #[inline(always)]
    fn from(_value: DebugName) -> Self {
        cfg::debug! {
            if { _value.name }
            else { Cow::Borrowed(FEATURE_DISABLED) }
        }
    }
}

impl From<String> for DebugName {
    #[inline(always)]
    fn from(value: String) -> Self {
        Self::owned(value)
    }
}

impl From<&'static str> for DebugName {
    #[inline(always)]
    fn from(value: &'static str) -> Self {
        Self::borrowed(value)
    }
}

impl DebugName {
    /// Create a new `DebugName` from a `&str`
    ///
    /// The value will be ignored if the `debug` feature is not enabled
    #[inline(always)]
    pub const fn borrowed(_value: &'static str) -> Self {
        cfg::debug! {
            if { Self { name: Cow::Borrowed(_value) } }
            else { Self {} }
        }
    }

    /// Create a new `DebugName` from a `String`
    ///
    /// The value will be ignored if the `debug` feature is not enabled
    #[inline(always)]
    pub fn owned(_value: String) -> Self {
        cfg::debug! {
            if { Self { name: Cow::Owned(_value) } }
            else { Self {} }
        }
    }

    /// Create a new `DebugName` from a type by using its [`core::any::type_name`]
    ///
    /// The value will be ignored if the `debug` feature is not enabled
    #[inline(always)]
    pub fn type_name<T>() -> Self {
        cfg::debug! {
            if {
                Self {
                    name: Cow::Borrowed(::core::any::type_name::<T>())
                }
            }
            else {
                Self {}
            }
        }
    }

    /// Get the [`ShortName`] corresponding to this debug name
    ///
    /// The value will be a static string if the `debug` feature is not enabled
    #[inline(always)]
    pub fn shortname(&self) -> ShortName<'_> {
        cfg::debug! {
            if { ShortName(self.name.as_ref()) }
            else { ShortName(FEATURE_DISABLED) }
        }
    }
}
