use crate::ReflectMeta;
use quote::quote_spanned;

#[cfg(feature = "auto_register")]
pub(crate) fn get_auto_register_impl(meta: &ReflectMeta) -> proc_macro2::TokenStream {
    if let Some(span) = meta.attrs().auto_register {
        if meta.impl_with_generic() {
            return crate::utils::empty();
        }

        let vc_reflect_path = meta.vc_reflect_path();
        let auto_register_ = crate::path::auto_register_(vc_reflect_path);
        let real_ident = meta.real_ident();

        quote_spanned! { span =>
            #auto_register_::inventory::submit!{
                #auto_register_::__AutoRegisterFunc(
                    <#real_ident as #auto_register_::__RegisterType>::__register
                )
            }
        }
    } else {
        crate::utils::empty()
    }
}

#[cfg(not(feature = "auto_register"))]
pub(crate) fn get_auto_register_impl(_: &ReflectMeta) -> proc_macro2::TokenStream {
    crate::utils::empty()
}
