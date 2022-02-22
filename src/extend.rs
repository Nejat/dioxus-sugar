use proc_macro::Span;

use proc_macro2::{Ident, TokenStream};
use quote::ToTokens;
use syn::{AttributeArgs, ItemStruct, Meta, NestedMeta, Path};
use web_reference::prelude::*;

use crate::SPECS;

#[derive(Debug, Clone, Eq, PartialEq)]
pub struct Extension {
    name: Ident,
    exclude: bool,
}

pub fn extend_struct_attributes(
    input: &mut ItemStruct, args: &AttributeArgs,
) -> TokenStream {
    let extensions = extensions(
        args,
        "attribute or tag",
        |path: &Path| {
            const GLOBAL: &str = "global";

            let attribute = path.to_token_stream().to_string();

            if SPECS.is_valid_attribute(&attribute) ||
                SPECS.is_valid_tag(&attribute) ||
                attribute == GLOBAL ||
                TagCategory::try_from(attribute.as_str()).is_ok()
            {
                return;
            }

            abort!(&path.to_token_stream(), format!("invalid attribute or tag {attribute:?}"));
        },
    ).flat_map(|ext| {
        if ext.name == "global" {
            SPECS.get_attributes_of_category(AttributeCategory::GlobalAttributes).unwrap()
                .into_iter()
                .map(|attr| Extension {
                    name: Ident::new(attr.name.trim_end_matches("-*"), ext.name.span()),
                    exclude: ext.exclude
                })
                .collect::<Vec<_>>()
        } else if let Ok(tags_category) = TagCategory::try_from(ext.name.to_string().as_str()) {
            SPECS.get_tags_of_category(tags_category).unwrap()
                .into_iter()
                .map(|attr| Extension {
                    name: Ident::new(attr.name.trim_end_matches("-*"), ext.name.span()),
                    exclude: ext.exclude
                })
                .collect::<Vec<_>>()
        } else {
            vec![ext]
        }
    }).collect::<Vec<_>>();

    if extensions.is_empty() {
        abort!(Span::call_site(), "no attributes or tags were listed, remove 'attributes' macro or add an attribute or tag");
    }

    check_net_extensions(&extensions, "no net attributes or tags to extend, check exclude list");

    quote! { #input }
}

pub fn extend_struct_events(
    input: &mut ItemStruct, args: &AttributeArgs,
) -> TokenStream {
    let extensions = extensions(
        args,
        "event",
        |path: &Path| {
            let event = path.to_token_stream().to_string();

            if SPECS.is_valid_event(&event) || EventCategory::try_from(event.as_str()).is_ok() {
                return;
            }

            abort!(&path.to_token_stream(), format!("invalid event {event:?}"))
        },
    ).flat_map(|ext| {
        if let Ok(event_category) = EventCategory::try_from(ext.name.to_string().as_str()) {
            SPECS.get_events_of_category(event_category).unwrap()
                .into_iter()
                .map(|attr| Extension {
                    name: Ident::new(&attr.name, ext.name.span()),
                    exclude: ext.exclude
                })
                .collect::<Vec<_>>()
        } else {
            vec![ext]
        }
    }).collect::<Vec<_>>();

    if extensions.is_empty() {
        abort!(Span::call_site(), "no events were listed, remove 'events' macro or add an event")
    }

    check_net_extensions(&extensions, "no net events to extend, check exclude list");

    quote! { #input }
}

fn check_net_extensions(extensions: &[Extension], error: &str) {
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

fn extensions<'a>(
    args: &'a AttributeArgs, attr_type: &'a str, validate_extension: fn(&'a Path),
) -> impl Iterator<Item=Extension> +'a {
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
