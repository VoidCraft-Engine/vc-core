use proc_macro::{TokenStream, TokenTree};
use quote::{quote, quote_spanned};
use std::collections::HashSet;
use syn::{Data, DataEnum, DataUnion, Error, Fields};
use syn::{Ident, spanned::Spanned};

// -----------------------------------------------------------------------------
// ensure_no_collision

/// Finds an identifier that will not conflict with the specified set of tokens.
///
/// If the identifier is present in `haystack`, extra characters will be added
/// to it until it no longer conflicts with anything.
///
/// Note that the returned identifier can still conflict in niche cases,
/// such as if an identifier in `haystack` is hidden behind an un-expanded macro.
pub(crate) fn ensure_no_collision(value: Ident, haystack: TokenStream) -> Ident {
    // Collect all the identifiers in `haystack` into a set.
    let idents = {
        // List of token streams that will be visited in future loop iterations.
        let mut unvisited = vec![haystack];
        // Identifiers we have found while searching tokens.
        let mut found = HashSet::new();
        while let Some(tokens) = unvisited.pop() {
            for t in tokens {
                match t {
                    // Collect any identifiers we encounter.
                    TokenTree::Ident(ident) => {
                        found.insert(ident.to_string());
                    }
                    // Queue up nested token streams to be visited in a future loop iteration.
                    TokenTree::Group(g) => unvisited.push(g.stream()),
                    TokenTree::Punct(_) | TokenTree::Literal(_) => {}
                }
            }
        }

        found
    };

    let span = value.span();

    // If there's a collision, add more characters to the identifier
    // until it doesn't collide with anything anymore.
    let mut value = value.to_string();

    while idents.contains(&value) {
        value.push('_');
    }

    Ident::new(&value, span)
}

// -----------------------------------------------------------------------------
// derive_label

/// Derive a label trait
///
/// # Args
///
/// - `input`: The [`syn::DeriveInput`] for struct that is deriving the label trait
/// - `trait_name`: Name of the label trait
/// - `trait_path`: The [path](`syn::Path`) to the label trait
/// - `dyn_eq_path`: The [path](`syn::Path`) to the `DynEq` trait
pub(crate) fn derive_label(
    input: syn::DeriveInput,
    vc_ecs_path: &syn::Path,
    trait_path: &syn::Path,
    trait_name: &str,
) -> TokenStream {
    if let Data::Union(_) = &input.data {
        let message = format!("Cannot derive {trait_name} for unions.");
        return quote_spanned! {
            input.span() => compile_error!(#message);
        }
        .into();
    }

    let ident = input.ident.clone();

    let (impl_generics, ty_generics, where_clause) = input.generics.split_for_impl();
    let mut where_clause = where_clause.cloned().unwrap_or_else(|| syn::WhereClause {
        where_token: Default::default(),
        predicates: Default::default(),
    });

    use crate::path::fp::{CloneFP, DebugFP, EqFP, HashFP, SendFP, SyncFP};

    where_clause.predicates.push(
        syn::parse2(quote! {
            Self: 'static + #SendFP + #SyncFP + #CloneFP + #EqFP + #DebugFP + #HashFP
        })
        .unwrap(),
    );

    let macro_utils_ = crate::path::macro_utils_(vc_ecs_path);

    quote! {
        const _: () = {
            impl #impl_generics #trait_path for #ident #ty_generics #where_clause {
                fn dyn_clone(&self) -> #macro_utils_::Box<dyn #trait_path> {
                    #macro_utils_::Box::new(#CloneFP::clone(self))
                }
            }
        };
    }
    .into()
}

// -----------------------------------------------------------------------------
// get_struct_fields

/// Get the fields of a data structure if that structure is a struct;
/// otherwise, return a compile error that points to the site of the macro invocation.
///
/// `meta` should be the name of the macro calling this function.
pub fn get_struct_fields<'a>(data: &'a Data, meta: &str) -> Result<&'a Fields, Error> {
    match data {
        Data::Struct(data_struct) => Ok(&data_struct.fields),
        Data::Enum(DataEnum { enum_token, .. }) => Err(Error::new(
            enum_token.span(),
            format!("#[{meta}] only supports structs, not enums"),
        )),
        Data::Union(DataUnion { union_token, .. }) => Err(Error::new(
            union_token.span(),
            format!("#[{meta}] only supports structs, not unions"),
        )),
    }
}
