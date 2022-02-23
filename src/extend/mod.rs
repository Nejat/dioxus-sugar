use proc_macro::Span;

use proc_macro2::{Ident, TokenStream};
use quote::ToTokens;
use syn::{AttributeArgs, Fields, FieldsNamed, ItemStruct, Meta, NestedMeta, Path};

pub mod attributes;
pub mod events;

///
#[derive(Debug, Clone, Eq, PartialEq)]
pub struct Extension {
    ///
    name: Ident,

    ///
    exclude: bool,
}

///
pub fn extend_input_struct(
    input: &mut ItemStruct, extensions: &[Extension], extend: fn(&Extension) -> TokenStream,
) -> TokenStream {
    let ItemStruct { attrs, vis, struct_token, ident, generics, fields, .. } = input;
    let extensions = extensions.iter().map(extend);

    let fields = match fields {
        Fields::Named(FieldsNamed { named, .. }) => {
            let fields = named.iter();

            quote! { #(#fields ,)* }
        }
        Fields::Unnamed(_) => abort!(input, "Structs with unnamed fields are not supported"),
        Fields::Unit => quote! {},
    };

    quote! {
        #(#attrs)* #vis #struct_token #generics #ident {
            #fields
            #(#extensions),*
        }
    }
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
    args: &'a AttributeArgs, attr_type: &'a str, validate_extension: fn(&'a Path),
) -> impl Iterator<Item=Extension> + 'a {
    const EXCLUDE: &str = "exclude";

    args.iter()
        .flat_map(move |arg| match arg {
            NestedMeta::Meta(Meta::Path(path)) if path.get_ident().is_some() => {
                validate_extension(path);

                vec![Extension { name: path.get_ident().unwrap().clone(), exclude: false }]
            }

            NestedMeta::Meta(Meta::List(list)) if list.path.to_token_stream().to_string() == EXCLUDE =>
                list.nested.iter()
                    .flat_map(move |item| match item {
                        NestedMeta::Meta(Meta::Path(path)) if path.segments.len() == 1 => {
                            validate_extension(path);

                            path.segments.iter().map(|seg| Extension { name: seg.ident.clone(), exclude: true })
                        }

                        _ => abort!(&arg.to_token_stream(), format!(r#"not a valid {attr_type}"#))
                    })
                    .collect::<Vec<_>>(),

            _ => abort!(&arg.to_token_stream(), format!(r#"not a valid {attr_type}"#))
        })
}
