//! See following macros:
//!
//! - [`Reflect`]
//! - [`TypePath`]
//! - [`impl_reflect`]
//! - [`impl_reflect_opaque`]
//! - [`impl_type_path`]
//! - [`impl_auto_register`]
//! - [`reflect_trait`]

use proc_macro::TokenStream;
use quote::quote;
use syn::{DeriveInput, parse_macro_input};

use crate::derive_data::{
    ReflectMeta, ReflectOpaqueParser, ReflectTypePathParser, TypeAttributes, TypeParser,
};

pub(crate) static REFLECT_ATTRIBUTE_NAME: &str = "reflect";

mod path;

mod derive_data;
mod impls;
mod utils;

/// # Derive Full Reflection
///
/// `#[derive(Reflect)]` will implement the following traits:
///
/// - `TypePath`
/// - `Typed`
/// - `Reflect`
/// - `GetTypeMeta`
/// - `FromReflect`
/// - `Struct` for `struct T { ... }`
/// - `TupleStruct` for `struct T(...);`
/// - `Enum` for `enum T { ... }`
///
/// Note: `struct T;` is treated as `Opaque` rather than as `Struct` or other composite kinds.
///
/// ## Impl Control
///
/// ### Turn off default implementation
///
/// You can disable specific impls via attributes; then you must provide them manually.
///
/// ```rust, ignore
/// #[derive(Reflect)]
/// #[reflect(TypePath = false, Typed = false)]
/// struct Foo { /* ... */ }
/// ```
///
/// All of the above toggles can be turned off; turning them on explicitly is meaningless because it is the default.
///
/// ### Custom type path
///
/// Because `TypePath` often needs customization, an attribute is provided to override the path:
///
/// ```rust, ignore
/// #[derive(Reflect)]
/// #[reflect(type_path = "you::me::Foo")]
/// struct Foo { /* ... */ }
/// ```
///
/// This path does not need to contain generics (it will be automatically added).
///
/// ### Opaque Type
///
/// Unit structs like `struct A;` are treated as `Opaque`.
/// They have no internal data, so the macro can auto-generate `reflect_clone`, `reflect_partial_eq` etc.
///
/// ```rust, ignore
/// #[derive(Reflect)]
/// struct MyFlag;
/// ```
///
/// `Opaque` is a special attribute that forces the type to be treated as `Opaque` instead of `Struct`, `Enum` or `TupleStruct`.
///
/// When you mark a type `Opaque`, the macro will not inspect internal fields; as a result
/// methods such as `reflect_clone` or `reflect_hash` that depend on field content cannot be
/// generated automatically. Therefore `Opaque` types must implement `Clone` (and be marked
/// with the `clone` flag) when appropriate:
///
/// ```rust, ignore
/// #[derive(Reflect)]
/// #[reflect(Opaque, clone)]
/// struct Foo { /* ... */ }
///
/// impl Clone for Foo {  /* ... */ }
/// ```
///
/// ## Optimize with standard traits
///
/// If the type implements traits like `Hash` or `Clone`, the reflection impls can be simplified (often much faster).
/// The macro cannot know this, so it does not assume them by default.
/// Use attributes to declare availability so the macro can optimize.
///
/// As noted, `Opaque` types must support `Clone`, so implement it and mark with `clone`.
///
/// ```rust, ignore
/// #[derive(Reflect)]
/// #[reflect(Opaque, clone, hash)]
/// struct Foo { /* ... */ }
/// // impl Clone, Hash ...
/// ```
///
/// Available flags:
///
/// - `clone`: std Clone
/// - `default`: std Default
/// - `hash`: std Hash
/// - `partial_eq`: std PartialEq
/// - `serialize`: serde::Serialize
/// - `deserialize`: serde::Deserialize
///
/// Three convenience bundles enable multiple flags at once:
///
/// - `mini`: `clone` + `auto_register`
/// - `serde`: `serialize` + `deserialize` + `auto_register`
/// - `full`: above six types and `auto_register`
///
/// ## auto_register
///
/// Unlike Bevy, automatic registration is turned off by default(even if `auto_register` feature is enabled).
/// You need to explicitly enable it using the `auto_register` attribute.
///
/// ### Example
///
/// ```rust, ignore
/// #[derive(Reflect)]
/// #[reflect(auto_register)]
/// struct A { /* ... */ }
/// ```
///
/// Note that this macro has no effect on generic types,
/// because we cannot know which specific types it will instantiate.
///
/// This attribute is a no-op when the `auto_register` feature is disabled.
///
/// ## Custom GetTypeMeta
///
/// By default, a type's `get_type_meta` contains at least `TypeTraitFromPtr`.
/// And the following type traits may also be included:
///
/// - `TypeTraitFromReflect`: If the default `FromReflect` impl is enabled (not disabled with `#[reflect(FromReflect = false)]`).
/// - `TypeTraitDefault`: If `Default` is marked available with `#[reflect(default)]`.
/// - `TypeTraitSerialize`: If `serde::Serialize` is marked available with `#[reflect(serialize)]`.
/// - `TypeTraitDeserialize`: If `serde::Deserialize` is marked available with `#[reflect(deserialize)]`.
///
/// But you can also add the TypeTrait with `#[reflect(type_trait = (...))]`,
/// and they will be automatically inserted during `get_type_meta`.
///
/// ### Example
///
/// ```rust, ignore
/// #[derive(Reflect)]
/// #[reflect(type_trait = TypeTraitPrint)]
/// struct A;
///
/// #[derive(Reflect)]
/// #[reflect(type_trait = (ReflectDebug, TypeTraitClone, ReflectDisplay))]
/// struct A;
/// ```
///
/// ## Docs Reflection
///
/// You can enable the `reflect_docs` feature to include docs in type info.
///
/// By default the macro collects `#[doc = "..."]` (including `/// ...`).
///
/// If you need to disable doc collection for a specific type, use `#[reflect(doc = false)]`.
///
/// ```rust, ignore
/// /// example doc comments
/// #[derive(Reflect)]
/// #[reflect(doc = false)]
/// struct A;
/// ```
///
/// To provide custom docs instead of collecting `#[doc = "..."]`, use one or more
/// `#[reflect(doc = "...")]` attributes, for example:
///
/// ```rust, ignore
/// /// default comments
/// /// ...
/// #[derive(Reflect)]
/// #[reflect(doc = "custom comments, line 1.")]
/// #[reflect(doc = "custom comments, line 2.")]
/// struct A;
/// ```
///
/// When the macro detects `#[reflect(doc = "...")]`, it will stop collecting standard `#[doc = "..."]` documentation.
///
/// This attribute is a no-op when the `reflect_docs` feature is disabled.
#[proc_macro_derive(Reflect, attributes(reflect))]
pub fn derive_full_reflect(input: TokenStream) -> TokenStream {
    let ast = parse_macro_input!(input as DeriveInput);

    impls::match_reflect_impls(ast, ImplSourceKind::DeriveLocalType)
}

/// # Derive TypePath Trait
///
/// This macro only implements `TypePath` trait,
///
/// The usage is similar to [`derive Reflect`](derive_full_reflect).
///
/// ## Example
///
/// ```rust, ignore
/// // default implementation
/// #[derive(TypePath)]
/// struct A;
///
/// // custom implementation
/// #[derive(TypePath)]
/// #[reflect(type_path = "crate_name::foo::B")]
/// struct B;
///
/// // support generics
/// #[derive(TypePath)]
/// #[reflect(type_path = "crate_name::foo::C")]
/// struct C<T>(T);
/// ```
#[proc_macro_derive(TypePath, attributes(reflect))]
pub fn derive_type_path(input: TokenStream) -> TokenStream {
    let ast: DeriveInput = parse_macro_input!(input as DeriveInput);

    let type_attributes = match TypeAttributes::parse_attrs(&ast.attrs) {
        Ok(v) => v,
        Err(err) => return err.into_compile_error().into(),
    };

    let type_parser =
        TypeParser::new_local(&ast.ident, type_attributes.type_path.clone(), &ast.generics);

    let meta = ReflectMeta::new(type_attributes, type_parser);
    impls::impl_trait_type_path(&meta).into()
}

/// Implements reflection for foreign types.
///
/// It requires full type information and access to fields.
/// Because of the orphan rule, this is typically used inside the reflection crate itself.
///
/// The usage is similar to [`derive Reflect`](derive_full_reflect).
///
/// ## Example
///
/// ```rust, ignore
/// impl_reflect! {
///     #[reflect(type_path = "core::option:Option")]
///     enum Option<T> {
///         Some(T),
///         None,
///     }
/// }
/// ```
///
/// See more infomation in [`derive Reflect`](derive_full_reflect) .
#[proc_macro]
pub fn impl_reflect(input: TokenStream) -> TokenStream {
    let ast = parse_macro_input!(input as DeriveInput);

    impls::match_reflect_impls(ast, ImplSourceKind::ImplForeignType)
}

/// How the macro was invoked.
#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub(crate) enum ImplSourceKind {
    /// Using `impl_full_reflect!`.
    ImplForeignType,
    /// Using `#[derive(...)]`.
    DeriveLocalType,
}

/// Implements reflection for `Opaque` types.
///
/// Syntax: `(in module_path as alias_name) ident (..attrs..)`.
///
/// ## Example
///
/// ```rust, ignore
/// impl_reflect_opaque!(u64 (full));
/// impl_reflect_opaque!(::utils::One<T: Clone> (clone));
/// impl_reflect_opaque!(::alloc::string::String (clone, debug, docs = "hello"));
/// impl_reflect_opaque!((in core::time) Instant (clone));
/// impl_reflect_opaque!((in core::time as Ins) Instant (clone));
/// ```
///
/// This macro always implies `Opaque`, so `clone` is required.
///
/// See available attributes in [`derive Reflect`](derive_full_reflect) .
#[proc_macro]
pub fn impl_reflect_opaque(input: TokenStream) -> TokenStream {
    let ReflectOpaqueParser {
        attrs,
        custom_path,
        type_ident,
        type_path,
        generics,
    } = parse_macro_input!(input with ReflectOpaqueParser::parse);

    let parser = TypeParser::new_foreign(&type_ident, &type_path, custom_path, &generics);

    let meta = ReflectMeta::new(attrs, parser);

    let assert_tokens = meta.assert_ident_tokens();
    let reflect_impls = impls::impl_opaque(&meta);

    quote! {
        const _: () = {
            #assert_tokens
            #reflect_impls
        };
    }
    .into()
}

/// A macro that implements `TypePath` for foreign type.
///
/// Syntax: `(in module_path as alias_name) ident`.
///
/// Paths starting with `::` cannot be used for primitive types.
/// The specified path must resolve to the target type and be accessible from the crate where the macro is invoked.
///
/// ## Example
///
/// ```ignore
/// // impl for primitive type.
/// impl_type_path!(u64);
///
/// // Implement for specified type.
/// impl_type_path!(::alloc::string::String);
/// // The prefix `::` will be removed by the macro, but it's required.
/// // This indicates that this is a complete path.
///
/// // Generics are also supported.
/// impl_type_path!(::utils::One<T>);
///
/// // Custom module path for specified type.
/// // then, it's type_path is `core::time::Instant`
/// impl_type_path!((in core::time) Instant);
///
/// // Custom module and ident for specified type.
/// // then, it's type_path is `core::time::Ins`
/// impl_type_path!((in core::time as Ins) Instant);
/// ```
///
/// See: [`derive Reflect`](derive_full_reflect)
#[proc_macro]
pub fn impl_type_path(input: TokenStream) -> TokenStream {
    let ReflectTypePathParser {
        custom_path,
        type_ident,
        type_path,
        generics,
    } = parse_macro_input!(input with ReflectTypePathParser::parse);

    let parser = TypeParser::new_foreign(&type_ident, &type_path, custom_path, &generics);

    let meta = ReflectMeta::new(TypeAttributes::default(), parser);
    let assert_tokens = meta.assert_ident_tokens();

    let type_path_impls = impls::impl_trait_type_path(&meta);

    quote! {
        const _: () = {
            #assert_tokens
            #type_path_impls
        };
    }
    .into()
}

/// Add the type to the automatic registry.
///
/// If the feature is not enabled, this macro will not do anything.
///
/// The type must be concrete (no uncertain generic parameters).
///
/// ## Example
///
/// ```ignore
/// impl_auto_register!(foo::Foo);
/// impl_auto_register!(Vec<u32>); // Ok
/// impl_auto_register!(Vec<T: Clone>); // Error
/// ```
///
/// This will not conflict with `reflect(auto_register)` attribute.
///
/// See: [`derive Reflect`](derive_full_reflect)
#[proc_macro]
pub fn impl_auto_register(_input: TokenStream) -> TokenStream {
    #[cfg(not(feature = "auto_register"))]
    return crate::utils::empty().into();

    #[cfg(feature = "auto_register")]
    {
        let type_path = syn::parse_macro_input!(_input as syn::Type);

        let vc_reflect_path = path::vc_reflect();
        let auto_register_ = path::auto_register_(&vc_reflect_path);

        return TokenStream::from(quote! {
            const _: () = {
                #auto_register_::inventory::submit!{
                    #auto_register_::__AutoRegisterFunc(
                        <#type_path as #auto_register_::__RegisterType>::__register
                    )
                }
            };
        });
    }
}

/// Impl `TypeTrait` for specific trait with a new struct.
///
/// This macro will generate a `Reflect{trait_name}` struct, which implements `TypeTrait` and `TypePath`.
///
/// For example, for `Display`, this will generate `ReflectDisplay`.
///
/// It only contains three methods internally:
/// - `get`: cast `&dyn Reflect` to `&dyn {trait_name}`
/// - `get_mut`: cast `&mut dyn Reflect` to `&mut dyn {trait_name}`
/// - `get_boxed`: cast `Box<dyn Reflect>` to `Box<dyn {trait_name}>`
///
/// The generated `Reflect{Trait}` helper only provides casting helpers (not the trait methods
/// themselves), so the struct is named `Reflect{Trait}` rather than using a `TypeTrait` prefix.
///
/// ## Example
///
/// ```ignore
/// #[reflect_trait]
/// pub trait MyDebug {
///     fn debug(&self);
/// }
///
/// impl MyDebug for String { /* ... */ }
///
/// let registry = TypeRegistry::new()
///     .register::<String>()
///     .register_type_trait::<String, ReflectMyDebug>();
///
/// let x: Box<dyn Reflect> = Box::new(String::from("123"));
///
/// let reflect_my_debug = register.get_type_trait::<ReflectMyDebug>::(x.ty_id()).unwrap();
/// let x: Box<dyn MyDebug> = reflect_my_debug.get_boxed(x);
/// x.debug();
/// ```
#[proc_macro_attribute]
pub fn reflect_trait(_args: TokenStream, input: TokenStream) -> TokenStream {
    impls::impl_reflect_trait(input)
}
