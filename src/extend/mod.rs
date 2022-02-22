use proc_macro::Span;

use proc_macro2::Ident;
use quote::ToTokens;
use syn::{AttributeArgs, Meta, NestedMeta, Path};

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
pub fn check_net_extensions(extensions: &[Extension], error: &str) {
    let include = extensions.iter()
        .filter_map(|ext| if ext.exclude { None } else { Some(ext.name.clone()) })
        .collect::<Vec<_>>();

    let exclude = extensions.iter()
        .filter_map(|ext| if ext.exclude { Some(ext.name.clone()) } else { None })
        .collect::<Vec<_>>();

    if include.iter().all(|inc| exclude.contains(inc)) {
        abort!(Span::call_site(), error);
    }
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
