/// A container for [`ConstParamInfo`](crate::info::ConstParamInfo), used to reduce heap allocation.
///
/// The only allowed types of const parameters are
/// u8, u16, u32, u64, u128, usize, i8, i16, i32, i64, i128, isize, char and bool.
///
/// ```
/// # use vc_reflect::info::ConstParamData;
///
/// let x: ConstParamData = 7i32.into();
/// # assert!(x.as_u32().is_none());
/// let x = x.as_i32().unwrap();
/// ```
///
/// See: <https://doc.rust-lang.org/reference/items/generics.html>
#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq)]
pub enum ConstParamData {
    U8(u8),
    U16(u16),
    U32(u32),
    U64(u64),
    U128(u128),
    Usize(usize),
    I8(i8),
    I16(i16),
    I32(i32),
    I64(i64),
    I128(i128),
    Isize(isize),
    Char(char),
    Bool(bool),
}

macro_rules! impl_from_fn {
    ($ty:ident, $kind:ident) => {
        impl From<$ty> for ConstParamData {
            #[inline(always)]
            fn from(value: $ty) -> Self {
                Self::$kind(value)
            }
        }
    };
}

macro_rules! impl_cast_fn {
    ($name:ident, $ty:ident, $kind:ident) => {
        #[inline]
        pub fn $name(&self) -> Option<$ty> {
            match self {
                Self::$kind(val) => Some(*val),
                _ => None,
            }
        }
    };
}

impl_from_fn!(u8, U8);
impl_from_fn!(u16, U16);
impl_from_fn!(u32, U32);
impl_from_fn!(u64, U64);
impl_from_fn!(u128, U128);
impl_from_fn!(usize, Usize);
impl_from_fn!(i8, I8);
impl_from_fn!(i16, I16);
impl_from_fn!(i32, I32);
impl_from_fn!(i64, I64);
impl_from_fn!(i128, I128);
impl_from_fn!(isize, Isize);
impl_from_fn!(char, Char);
impl_from_fn!(bool, Bool);

impl ConstParamData {
    impl_cast_fn!(as_u8, u8, U8);
    impl_cast_fn!(as_u16, u16, U16);
    impl_cast_fn!(as_u32, u32, U32);
    impl_cast_fn!(as_u64, u64, U64);
    impl_cast_fn!(as_u128, u128, U128);
    impl_cast_fn!(as_usize, usize, Usize);
    impl_cast_fn!(as_i8, i8, I8);
    impl_cast_fn!(as_i16, i16, I16);
    impl_cast_fn!(as_i32, i32, I32);
    impl_cast_fn!(as_i64, i64, I64);
    impl_cast_fn!(as_i128, i128, I128);
    impl_cast_fn!(as_isize, isize, Isize);
    impl_cast_fn!(as_char, char, Char);
    impl_cast_fn!(as_bool, bool, Bool);
}
