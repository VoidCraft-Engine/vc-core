use crate::ReflectMeta;
use proc_macro2::TokenStream;
use quote::quote;

/// Try `clone` or `reflect_clone`
pub(crate) fn get_common_try_apply_tokens(meta: &ReflectMeta, input: &syn::Ident) -> TokenStream {
    use crate::path::fp::{CloneFP, ResultFP};

    let vc_reflect_path = meta.vc_reflect_path();
    let reflect_ = crate::path::reflect_(vc_reflect_path);

    if meta.attrs().avail_traits.clone.is_some() {
        quote! {
            if let Some(__val) = <dyn #reflect_>::downcast_ref::<Self>(#input) {
                *self = #CloneFP::clone(__val);
                return #ResultFP::Ok(());
            }
        }
    } else {
        quote! {
            if <dyn #reflect_>::is::<Self>(#input) {
                if let Ok(__cloned) = #reflect_::reflect_clone(#input)
                    && let Ok(__val) = <dyn #reflect_>::take::<Self>(__cloned)
                {
                    *self = __val;
                    return #ResultFP::Ok(());
                }
            }
        }
    }
}

/// Try `clone` or `reflect_clone`
pub(crate) fn get_common_from_reflect_tokens(
    meta: &ReflectMeta,
    input: &syn::Ident,
) -> TokenStream {
    use crate::path::fp::{CloneFP, OptionFP};

    let vc_reflect_path = meta.vc_reflect_path();
    let reflect_ = crate::path::reflect_(vc_reflect_path);

    if meta.attrs().avail_traits.clone.is_some() {
        quote! {
            if let Some(__val) = <dyn #reflect_>::downcast_ref::<Self>(#input) {
                return #OptionFP::Some(#CloneFP::clone(__val));
            }
        }
    } else {
        quote! {
            if <dyn #reflect_>::is::<Self>(#input) {
                if let Ok(__cloned) = #reflect_::reflect_clone(#input)
                    && let Ok(__val) = <dyn #reflect_>::take::<Self>(__cloned)
                {
                    return #OptionFP::Some(__val);
                }
            }
        }
    }
}
