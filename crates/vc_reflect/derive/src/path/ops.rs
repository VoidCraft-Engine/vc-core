use proc_macro2::TokenStream;
use quote::quote;

#[inline]
pub(crate) fn apply_error_(vc_reflect_path: &syn::Path) -> TokenStream {
    quote! {
        #vc_reflect_path::ops::ApplyError
    }
}

#[inline]
pub(crate) fn reflect_clone_error_(vc_reflect_path: &syn::Path) -> TokenStream {
    quote! {
        #vc_reflect_path::ops::ReflectCloneError
    }
}

#[inline]
pub(crate) fn reflect_mut_(vc_reflect_path: &syn::Path) -> TokenStream {
    quote! {
        #vc_reflect_path::ops::ReflectMut
    }
}

#[inline]
pub(crate) fn reflect_owned_(vc_reflect_path: &syn::Path) -> TokenStream {
    quote! {
        #vc_reflect_path::ops::ReflectOwned
    }
}

#[inline]
pub(crate) fn reflect_ref_(vc_reflect_path: &syn::Path) -> TokenStream {
    quote! {
        #vc_reflect_path::ops::ReflectRef
    }
}

#[inline]
pub(crate) fn dynamic_struct_(vc_reflect_path: &syn::Path) -> TokenStream {
    quote! {
        #vc_reflect_path::ops::DynamicStruct
    }
}

// #[inline]
// pub(crate) fn get_struct_field_(vc_reflect_path: &syn::Path) -> TokenStream {
//     quote! {
//         #vc_reflect_path::ops::GetStructField
//     }
// }

#[inline]
pub(crate) fn struct_(vc_reflect_path: &syn::Path) -> TokenStream {
    quote! {
        #vc_reflect_path::ops::Struct
    }
}

#[inline]
pub(crate) fn struct_field_iter_(vc_reflect_path: &syn::Path) -> TokenStream {
    quote! {
        #vc_reflect_path::ops::StructFieldIter
    }
}

#[inline]
pub(crate) fn struct_try_apply_(vc_reflect_path: &syn::Path) -> TokenStream {
    quote! {
        #vc_reflect_path::impls::struct_try_apply
    }
}

#[inline]
pub(crate) fn struct_hash_(vc_reflect_path: &syn::Path) -> TokenStream {
    quote! {
        #vc_reflect_path::impls::struct_hash
    }
}

#[inline]
pub(crate) fn struct_debug_(vc_reflect_path: &syn::Path) -> TokenStream {
    quote! {
        #vc_reflect_path::impls::struct_debug
    }
}

#[inline]
pub(crate) fn struct_partial_eq_(vc_reflect_path: &syn::Path) -> TokenStream {
    quote! {
        #vc_reflect_path::impls::struct_partial_eq
    }
}

#[inline]
pub(crate) fn dynamic_tuple_struct_(vc_reflect_path: &syn::Path) -> TokenStream {
    quote! {
        #vc_reflect_path::ops::DynamicTupleStruct
    }
}

// #[inline]
// pub(crate) fn get_tuple_struct_field_(vc_reflect_path: &syn::Path) -> TokenStream {
//     quote! {
//         #vc_reflect_path::ops::GetTupleStructField
//     }
// }

#[inline]
pub(crate) fn tuple_struct_(vc_reflect_path: &syn::Path) -> TokenStream {
    quote! {
        #vc_reflect_path::ops::TupleStruct
    }
}

#[inline]
pub(crate) fn tuple_struct_field_iter_(vc_reflect_path: &syn::Path) -> TokenStream {
    quote! {
        #vc_reflect_path::ops::TupleStructFieldIter
    }
}

#[inline]
pub(crate) fn tuple_struct_try_apply_(vc_reflect_path: &syn::Path) -> TokenStream {
    quote! {
        #vc_reflect_path::impls::tuple_struct_try_apply
    }
}

#[inline]
pub(crate) fn tuple_struct_hash_(vc_reflect_path: &syn::Path) -> TokenStream {
    quote! {
        #vc_reflect_path::impls::tuple_struct_hash
    }
}

#[inline]
pub(crate) fn tuple_struct_debug_(vc_reflect_path: &syn::Path) -> TokenStream {
    quote! {
        #vc_reflect_path::impls::tuple_struct_debug
    }
}

#[inline]
pub(crate) fn tuple_struct_partial_eq_(vc_reflect_path: &syn::Path) -> TokenStream {
    quote! {
        #vc_reflect_path::impls::tuple_struct_partial_eq
    }
}

// #[inline]
// pub(crate) fn dynamic_tuple_(vc_reflect_path: &syn::Path) -> TokenStream {
//     quote! {
//         #vc_reflect_path::ops::DynamicTuple
//     }
// }

// #[inline]
// pub(crate) fn get_tuple_field_(vc_reflect_path: &syn::Path) -> TokenStream {
//     quote! {
//         #vc_reflect_path::ops::GetTupleField
//     }
// }

// #[inline]
// pub(crate) fn tuple_(vc_reflect_path: &syn::Path) -> TokenStream {
//     quote! {
//         #vc_reflect_path::ops::Tuple
//     }
// }

// #[inline]
// pub(crate) fn tuple_field_iter_(vc_reflect_path: &syn::Path) -> TokenStream {
//     quote! {
//         #vc_reflect_path::ops::TupleFieldIter
//     }
// }

// #[inline]
// pub(crate) fn tuple_debug_(vc_reflect_path: &syn::Path) -> TokenStream {
//     quote! {
//         #vc_reflect_path::ops::tuple_debug
//     }
// }

// #[inline]
// pub(crate) fn tuple_partial_eq_(vc_reflect_path: &syn::Path) -> TokenStream {
//     quote! {
//         #vc_reflect_path::ops::tuple_partial_eq
//     }
// }

// #[inline]
// pub(crate) fn tuple_try_apply_(vc_reflect_path: &syn::Path) -> TokenStream {
//     quote! {
//         #vc_reflect_path::ops::tuple_try_apply
//     }
// }

// #[inline]
// pub(crate) fn dynamic_list_(vc_reflect_path: &syn::Path) -> TokenStream {
//     quote! {
//         #vc_reflect_path::ops::DynamicList
//     }
// }

// #[inline]
// pub(crate) fn list_(vc_reflect_path: &syn::Path) -> TokenStream {
//     quote! {
//         #vc_reflect_path::ops::List
//     }
// }

// #[inline]
// pub(crate) fn list_item_iter_(vc_reflect_path: &syn::Path) -> TokenStream {
//     quote! {
//         #vc_reflect_path::ops::ListItemIter
//     }
// }

// #[inline]
// pub(crate) fn list_debug_(vc_reflect_path: &syn::Path) -> TokenStream {
//     quote! {
//         #vc_reflect_path::ops::list_debug
//     }
// }

// #[inline]
// pub(crate) fn list_partial_eq_(vc_reflect_path: &syn::Path) -> TokenStream {
//     quote! {
//         #vc_reflect_path::ops::list_partial_eq
//     }
// }

// #[inline]
// pub(crate) fn array_(vc_reflect_path: &syn::Path) -> TokenStream {
//     quote! {
//         #vc_reflect_path::ops::Array
//     }
// }

// #[inline]
// pub(crate) fn array_item_iter_(vc_reflect_path: &syn::Path) -> TokenStream {
//     quote! {
//         #vc_reflect_path::ops::ArrayItemIter
//     }
// }

// #[inline]
// pub(crate) fn dynamic_array_(vc_reflect_path: &syn::Path) -> TokenStream {
//     quote! {
//         #vc_reflect_path::ops::DynamicArray
//     }
// }

// #[inline]
// pub(crate) fn array_debug_(vc_reflect_path: &syn::Path) -> TokenStream {
//     quote! {
//         #vc_reflect_path::ops::array_debug
//     }
// }

// #[inline]
// pub(crate) fn array_partial_eq_(vc_reflect_path: &syn::Path) -> TokenStream {
//     quote! {
//         #vc_reflect_path::ops::array_partial_eq
//     }
// }

// #[inline]
// pub(crate) fn dynamic_map_(vc_reflect_path: &syn::Path) -> TokenStream {
//     quote! {
//         #vc_reflect_path::ops::DynamicMap
//     }
// }

// #[inline]
// pub(crate) fn map_(vc_reflect_path: &syn::Path) -> TokenStream {
//     quote! {
//         #vc_reflect_path::ops::Map
//     }
// }

// #[inline]
// pub(crate) fn map_debug_(vc_reflect_path: &syn::Path) -> TokenStream {
//     quote! {
//         #vc_reflect_path::ops::map_debug
//     }
// }

// #[inline]
// pub(crate) fn map_partial_eq_(vc_reflect_path: &syn::Path) -> TokenStream {
//     quote! {
//         #vc_reflect_path::ops::map_partial_eq
//     }
// }

// #[inline]
// pub(crate) fn dynamic_set_(vc_reflect_path: &syn::Path) -> TokenStream {
//     quote! {
//         #vc_reflect_path::ops::DynamicSet
//     }
// }

// #[inline]
// pub(crate) fn set_(vc_reflect_path: &syn::Path) -> TokenStream {
//     quote! {
//         #vc_reflect_path::ops::Set
//     }
// }

// #[inline]
// pub(crate) fn set_debug_(vc_reflect_path: &syn::Path) -> TokenStream {
//     quote! {
//         #vc_reflect_path::ops::set_debug
//     }
// }

// #[inline]
// pub(crate) fn set_partial_eq_(vc_reflect_path: &syn::Path) -> TokenStream {
//     quote! {
//         #vc_reflect_path::ops::set_partial_eq
//     }
// }

// #[inline]
// pub(crate) fn dynamic_variant_(vc_reflect_path: &syn::Path) -> TokenStream {
//     quote! {
//         #vc_reflect_path::ops::DynamicVariant
//     }
// }

// #[inline]
// pub(crate) fn variant_field_(vc_reflect_path: &syn::Path) -> TokenStream {
//     quote! {
//         #vc_reflect_path::ops::VariantField
//     }
// }

#[inline]
pub(crate) fn variant_field_iter_(vc_reflect_path: &syn::Path) -> TokenStream {
    quote! {
        #vc_reflect_path::ops::VariantFieldIter
    }
}

// #[inline]
// pub(crate) fn dynamic_enum_(vc_reflect_path: &syn::Path) -> TokenStream {
//     quote! {
//         #vc_reflect_path::ops::DynamicEnum
//     }
// }

#[inline]
pub(crate) fn enum_(vc_reflect_path: &syn::Path) -> TokenStream {
    quote! {
        #vc_reflect_path::ops::Enum
    }
}

#[inline]
pub(crate) fn enum_try_apply_(vc_reflect_path: &syn::Path) -> TokenStream {
    quote! {
        #vc_reflect_path::impls::enum_try_apply
    }
}

#[inline]
pub(crate) fn enum_debug_(vc_reflect_path: &syn::Path) -> TokenStream {
    quote! {
        #vc_reflect_path::impls::enum_debug
    }
}

#[inline]
pub(crate) fn enum_partial_eq_(vc_reflect_path: &syn::Path) -> TokenStream {
    quote! {
        #vc_reflect_path::impls::enum_partial_eq
    }
}

#[inline]
pub(crate) fn enum_hash_(vc_reflect_path: &syn::Path) -> TokenStream {
    quote! {
        #vc_reflect_path::impls::enum_hash
    }
}
