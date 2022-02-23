use std::fmt;
use std::fmt::{Debug, Formatter};

use proc_macro2::{Ident, Span, TokenStream};
use quote::ToTokens;
use syn::{AttributeArgs, Fields, FieldsNamed, GenericParam, Generics, ItemStruct, Lifetime, LifetimeDef, Lit, Meta, NestedMeta, Path};
use syn::punctuated::Punctuated;

pub mod attributes;
pub mod events;

///
#[derive(Clone)]
pub struct Extension {
    ///
    name: Ident,

    ///
    exclude: bool,

    ///
    optional: bool,

    ///
    default: Option<Lit>,
}

impl Debug for Extension {
    fn fmt(&self, fmt: &mut Formatter<'_>) -> fmt::Result {
        let default = self.default.as_ref().map(|default| format!("{}", default.to_token_stream()));

        fmt.debug_struct("Extension")
            .field("name", &self.name)
            .field("exclude", &self.exclude)
            .field("optional", &self.optional)
            .field("default", &default)
            .finish()
    }
}

///
pub fn extend_input_struct(
    input: &mut ItemStruct,
    extensions: &[Extension],
    requires_life_time: bool,
    extend: fn(&Extension, &Option<Lifetime>) -> TokenStream,
) -> TokenStream {
    let ItemStruct { attrs, vis, struct_token, ident, generics, fields, .. } = input;
    let (generics, life_time) = parse_generics(generics, requires_life_time);
    let extensions = extensions.iter().map(|ext| extend(ext, &life_time));

    let fields = match fields {
        Fields::Named(FieldsNamed { named, .. }) => {
            let fields = named.iter();

            quote! { #(#fields ,)* }
        }
        Fields::Unnamed(_) => abort!(input, "structs with unnamed fields are not supported"),
        Fields::Unit => quote! {},
    };

    quote! {
        #(#attrs)* #vis #struct_token #ident #generics {
            #fields
            #(#extensions),*
        }
    }
}

fn parse_generics(generics: &mut Generics, requires_life_time: bool) -> (TokenStream, Option<Lifetime>) {
    let Generics { params, where_clause, .. } = &generics;

    let life_time = params.iter().find_map(
        |param| if let GenericParam::Lifetime(life_time) = param {
            Some(life_time.lifetime.clone())
        } else {
            None
        }
    );

    let (life_time, new_life_time) = match life_time {
        lt @ Some(_) => (lt, false),
        None if requires_life_time => (Some(Lifetime::new("'a", Span::call_site())), true),
        None => (None, false)
    };

    let lt = if new_life_time {
        let lt = GenericParam::Lifetime(LifetimeDef {
            attrs: vec![],
            lifetime: life_time.as_ref().unwrap().clone(),
            colon_token: None,
            bounds: Punctuated::default(),
        });

        if params.is_empty() {
            quote! { #lt }
        } else {
            quote! { #lt, }
        }
    } else {
        quote! { }
    };

    let params = params.iter();

    let generics = if new_life_time {
        quote! { < #lt #(#params),* > #where_clause }
    } else {
        quote! { #generics }
    };

    (generics, life_time)
}

///
pub fn net_extensions(extensions: Vec<Extension>, error: &str) -> Vec<Extension> {
    let include = extensions.iter()
        .filter_map(|ext| if ext.exclude { None } else { Some(ext.name.to_string()) })
        .collect::<Vec<_>>();

    let exclude = extensions.iter()
        .filter_map(|ext| if ext.exclude { Some(ext.name.to_string()) } else { None })
        .collect::<Vec<_>>();

    if include.iter().all(|inc| exclude.contains(inc)) {
        abort!(Span::call_site(), error);
    }

    extensions.into_iter()
        .filter(|ext| {
            let name = ext.name.to_string();

            include.contains(&name) && !exclude.contains(&name)
        }).collect::<Vec<_>>()
}

///
pub fn parse_extensions_args<'a>(
    args: &'a AttributeArgs,
    attr_type: &'a str,
    supports_default: bool,
    validate_extension: fn(&'a Path),
) -> impl Iterator<Item=Extension> + 'a {
    const DEFAULT: &str = "default";
    const EXCLUDE: &str = "exclude";
    const OPTIONAL: &str = "optional";

    args.iter()
        .flat_map(move |arg| match arg {
            NestedMeta::Meta(Meta::Path(path)) if path.get_ident().is_some() => {
                validate_extension(path);

                vec![Extension {
                    name: path.get_ident().unwrap().clone(),
                    exclude: false,
                    optional: false,
                    default: None,
                }]
            }

            NestedMeta::Meta(Meta::List(list)) => {
                let list_name = list.path.to_token_stream().to_string();

                let (exclude, optional) = match list_name.as_str() {
                    EXCLUDE => (true, false),
                    OPTIONAL if supports_default => (false, true),
                    DEFAULT if supports_default => (false, false),
                    _ => abort!(&arg.to_token_stream(), format!(r#"not a valid {attr_type} list"#))
                };

                list.nested.iter()
                    .flat_map(move |item| match item {
                        NestedMeta::Meta(Meta::Path(path)) if path.segments.len() == 1 => {
                            validate_extension(path);

                            path.segments.iter()
                                .map(move |seg| Extension {
                                    name: seg.ident.clone(),
                                    exclude,
                                    optional,
                                    default: None,
                                }).collect::<Vec<_>>()
                        }
                        NestedMeta::Meta(Meta::NameValue(named)) if supports_default => {
                            validate_extension(&named.path);

                            named.path.segments.iter()
                                .map(move |seg| Extension {
                                    name: seg.ident.clone(),
                                    exclude,
                                    optional,
                                    default: Some(named.lit.clone()),
                                }).collect::<Vec<_>>()
                        }

                        _ => abort!(&arg.to_token_stream(), format!(r#"not a valid {attr_type}"#))
                    })
                    .collect::<Vec<_>>()
            }

            _ => abort!(&arg.to_token_stream(), format!(r#"not a valid {attr_type}"#))
        })
}
