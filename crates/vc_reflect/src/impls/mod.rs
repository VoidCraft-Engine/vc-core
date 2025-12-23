mod cell;
pub use cell::{GenericTypeInfoCell, GenericTypePathCell, NonGenericTypeInfoCell};

mod utils;
pub use utils::*;

/// An efficient string concatenation function.
///
/// This is usually used for the implementation of `TypePath`.
///
/// # Example
///
/// ```
/// use vc_reflect::impls;
///
/// let s = impls::concat(&["module", "::", "name", "<", "T" , ">"]);
///
/// assert_eq!(s.capacity(), 15);
/// ```
///
/// Inline is prohibited here to reduce compilation time.
#[inline(never)]
pub fn concat(arr: &[&str]) -> ::alloc::string::String {
    let mut len = 0usize;
    for &item in arr {
        len += item.len();
    }
    let mut res = ::alloc::string::String::with_capacity(len);
    for &item in arr {
        res.push_str(item);
    }
    res
}

pub(crate) use utils::impl_simple_type_reflect;

mod alloc;
mod core;
mod native;
