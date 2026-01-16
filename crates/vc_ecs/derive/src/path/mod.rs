#![allow(unused)]

use proc_macro2::TokenStream;
use quote::quote;

// -----------------------------------------------------------------------------
// Crate Path

/// Get the correct access path to the `vc_ecs` crate.
///
/// Not all modules can access the ECS crate through `vc_ecs` directly,
/// we have to scan the builder's `cargo.toml`.
pub(crate) fn vc_ecs_path() -> syn::Path {
    vc_macro_utils::Manifest::shared(|manifest| manifest.get_crate_path("vc_ecs"))
}

pub(crate) use vc_macro_utils::full_path as fp;

// -----------------------------------------------------------------------------
// Internal API

#[inline(always)]
pub(crate) fn macro_utils_(vc_ecs_path: &syn::Path) -> TokenStream {
    quote! {
        #vc_ecs_path::__macro_utils
    }
}

#[inline(always)]
pub(crate) fn resource_(vc_ecs_path: &syn::Path) -> TokenStream {
    quote! {
        #vc_ecs_path::resource::Resource
    }
}

#[inline(always)]
pub(crate) fn entity_mapper_(vc_ecs_path: &syn::Path) -> TokenStream {
    quote! {
        #vc_ecs_path::entity::EntityMapper
    }
}

#[inline(always)]
pub(crate) fn map_entities_(vc_ecs_path: &syn::Path) -> TokenStream {
    quote! {
        #vc_ecs_path::entity::MapEntities
    }
}

#[inline(always)]
pub(crate) fn storage_type_(vc_ecs_path: &syn::Path) -> TokenStream {
    quote! {
        #vc_ecs_path::component::StorageType
    }
}

#[inline(always)]
pub(crate) fn relation_ship_(vc_ecs_path: &syn::Path) -> TokenStream {
    quote! {
        #vc_ecs_path::relationship::Relationship
    }
}

#[inline(always)]
pub(crate) fn relation_ship_target_(vc_ecs_path: &syn::Path) -> TokenStream {
    quote! {
        #vc_ecs_path::relationship::RelationshipTarget
    }
}
