use crate::{impls::GenericTypePathCell, info::TypePath};
use alloc::format;

impl<T: TypePath + ?Sized> TypePath for &'static T {
    fn type_path() -> &'static str {
        static CELL: GenericTypePathCell = GenericTypePathCell::new();
        CELL.get_or_insert::<Self>(|| format!("&{}", T::type_path()))
    }

    fn type_name() -> &'static str {
        static CELL: GenericTypePathCell = GenericTypePathCell::new();
        CELL.get_or_insert::<Self>(|| format!("&{}", T::type_name()))
    }

    fn type_ident() -> &'static str {
        static CELL: GenericTypePathCell = GenericTypePathCell::new();
        CELL.get_or_insert::<Self>(|| format!("&{}", T::type_ident()))
    }
}

impl<T: TypePath + ?Sized> TypePath for &'static mut T {
    fn type_path() -> &'static str {
        static CELL: GenericTypePathCell = GenericTypePathCell::new();
        CELL.get_or_insert::<Self>(|| format!("&mut {}", T::type_path()))
    }

    fn type_name() -> &'static str {
        static CELL: GenericTypePathCell = GenericTypePathCell::new();
        CELL.get_or_insert::<Self>(|| format!("&mut {}", T::type_name()))
    }

    fn type_ident() -> &'static str {
        static CELL: GenericTypePathCell = GenericTypePathCell::new();
        CELL.get_or_insert::<Self>(|| format!("&mut {}", T::type_ident()))
    }
}
