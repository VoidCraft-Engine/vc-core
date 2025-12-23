use crate::{
    Reflect,
    info::{
        Generics, Type, TypeInfo, TypePath, Typed, impl_docs_fn, impl_generic_fn, impl_type_fn,
    },
    ops::List,
};

///  A container for compile-time list-like info, size = 120 (exclude `docs`).
///
/// At present, `ListInfo` does not have `CustomAttributes`, which can save memory.
/// If necessary, it may be added in the future.
///
/// # Examples
///
/// ```rust
/// # use vc_reflect::info::{Typed, Type};
/// let info = <Vec<i32> as Typed>::type_info().as_list().unwrap();
///
/// assert_eq!(info.item_ty(), Type::of::<i32>());
/// ```
#[derive(Clone, Debug)]
pub struct ListInfo {
    ty: Type,
    generics: Generics,
    item_ty: Type,
    // `TypeInfo` is created on the first visit, use function pointers to delay it.
    item_info: fn() -> &'static TypeInfo,
    #[cfg(feature = "reflect_docs")]
    docs: Option<&'static str>,
}

impl ListInfo {
    impl_docs_fn!(docs);
    impl_type_fn!(ty);
    impl_generic_fn!(generics);

    /// Creates a new [`ListInfo`].
    #[inline]
    pub const fn new<TList: List + TypePath, TItem: Reflect + Typed>() -> Self {
        Self {
            ty: Type::of::<TList>(),
            generics: Generics::new(),
            item_ty: Type::of::<TItem>(),
            item_info: TItem::type_info,
            #[cfg(feature = "reflect_docs")]
            docs: None,
        }
    }

    /// Returns the [`Type`] of list items.
    #[inline]
    pub const fn item_ty(&self) -> Type {
        self.item_ty
    }

    /// Returns the [`TypeInfo`] of list items.
    #[inline]
    pub fn item_info(&self) -> &'static TypeInfo {
        (self.item_info)()
    }
}
