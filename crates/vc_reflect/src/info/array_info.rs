use crate::{
    Reflect,
    info::{
        Generics, Type, TypeInfo, TypePath, Typed, impl_docs_fn, impl_generic_fn, impl_type_fn,
    },
    ops::Array,
};

/// A container for compile-time array info, size = 128 (exclude `docs`).
///
/// At present, `ArrayInfo` does not have `CustomAttributes`, which can save memory.
///
/// # Examples
///
/// ```
/// use vc_reflect::info::{Typed, ArrayInfo};
///
/// // Get the `ArrayInfo` for `[i32; 5]` and inspect its properties.
/// let info = <[i32; 5] as Typed>::type_info().as_array().unwrap();
///
/// assert_eq!(info.capacity(), 5);
/// assert_eq!(info.type_path(), "[i32; 5]");
///
/// let item_info = info.item_info();
/// assert!(item_info.type_is::<i32>());
/// ```
#[derive(Clone, Debug)]
pub struct ArrayInfo {
    ty: Type,
    item_ty: Type,
    // `TypeInfo` is created on the first visit, use function pointers to delay it.
    item_info: fn() -> &'static TypeInfo,
    generics: Generics,
    capacity: usize,
    #[cfg(feature = "reflect_docs")]
    docs: Option<&'static str>,
}

impl ArrayInfo {
    impl_type_fn!(ty);
    impl_docs_fn!(docs);
    impl_generic_fn!(generics);

    /// Create a new [`ArrayInfo`].
    ///
    /// # Arguments
    ///
    /// - `capacity`: The maximum capacity of the underlying array.
    ///
    /// # Examples
    ///
    /// ```
    /// use vc_reflect::info::ArrayInfo;
    /// let info = ArrayInfo::new::<[i32; 7], i32>(7);
    /// ```
    #[inline]
    pub const fn new<TArray: Array + TypePath, TItem: Reflect + Typed>(capacity: usize) -> Self {
        Self {
            ty: Type::of::<TArray>(),
            generics: Generics::new(),
            item_ty: Type::of::<TItem>(),
            item_info: TItem::type_info,
            capacity,
            #[cfg(feature = "reflect_docs")]
            docs: None,
        }
    }

    /// The compile-time capacity of the array.
    #[inline]
    pub const fn capacity(&self) -> usize {
        self.capacity
    }

    /// Returns the [`Type`] of an array item.
    #[inline]
    pub const fn item_ty(&self) -> Type {
        self.item_ty
    }

    /// Returns the [`TypeInfo`] of array items.
    #[inline]
    pub fn item_info(&self) -> &'static TypeInfo {
        (self.item_info)()
    }
}
