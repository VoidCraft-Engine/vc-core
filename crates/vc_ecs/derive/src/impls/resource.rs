use proc_macro::TokenStream;
use quote::quote;
use syn::spanned::Spanned;
use syn::{DeriveInput, parse_macro_input, parse_quote};

pub(crate) fn derive_resource(input: TokenStream) -> TokenStream {
    use crate::path::fp::{SendFP, SyncFP};

    let mut ast = parse_macro_input!(input as DeriveInput);

    // ------------------------------------------------------------------------
    // Check lifetime

    let non_static_lifetime_error = ast
        .generics
        .lifetimes()
        .filter(|lifetime| !lifetime.bounds.iter().any(|bound| bound.ident == "static"))
        .map(|param| syn::Error::new(param.span(), "Lifetimes must be 'static"))
        .reduce(|mut err_acc, err| {
            err_acc.combine(err);
            err_acc
        });

    if let Some(err) = non_static_lifetime_error {
        return err.into_compile_error().into();
    }

    // ------------------------------------------------------------------------
    // Tokens

    ast.generics
        .make_where_clause()
        .predicates
        .push(parse_quote! { Self: #SendFP + #SyncFP + 'static });

    let (impl_generics, type_generics, where_clause) = &ast.generics.split_for_impl();

    let struct_name = &ast.ident;

    let vc_ecs_path = crate::path::vc_ecs_path();
    let resource_ = crate::path::resource_(&vc_ecs_path);

    TokenStream::from(quote! {
        impl #impl_generics #resource_ for #struct_name #type_generics #where_clause {}
    })
}
