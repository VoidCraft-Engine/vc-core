mod reflect;
pub use reflect::Reflect;
pub(crate) use reflect::impl_reflect_cast_fn;

mod from_reflect;
pub use from_reflect::FromReflect;

/// A Fixed Hasher for [`Reflect::reflect_hash`] implementation.
///
/// See more infomation in [`FixedHashState`](vc_utils::hash::FixedHashState) .
#[inline(always)]
pub fn reflect_hasher() -> vc_utils::hash::FixedHasher {
    core::hash::BuildHasher::build_hasher(&vc_utils::hash::FixedHashState)
}
