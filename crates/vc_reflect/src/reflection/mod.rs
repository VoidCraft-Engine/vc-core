mod reflect;
pub use reflect::Reflect;
pub(crate) use reflect::impl_reflect_cast_fn;

mod from_reflect;
pub use from_reflect::FromReflect;

/// Get Fixed Hasher
#[inline(always)]
pub fn reflect_hasher() -> vc_utils::hash::FixedHasher {
    core::hash::BuildHasher::build_hasher(&vc_utils::hash::FixedHashState)
}
