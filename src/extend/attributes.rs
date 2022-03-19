use std::collections::HashMap;

use proc_macro2::{Ident, Span, TokenStream};
use quote::ToTokens;
use syn::{AttributeArgs, ItemStruct, Lifetime, Path};
use web_reference::prelude::*;

use crate::common::attributes_of_tag;
use crate::extend::{extend_input_struct, Extension, ExtType, net_extensions, parse_extensions_args};
use crate::SPECS;

///
pub fn input_struct(input: &mut ItemStruct, args: &AttributeArgs) -> TokenStream {
    let extensions = extract_attribute_extensions(args);

    let requires_life_time = !extensions.iter()
        .all(|ext| {
            let name = ext.name.to_string();

            SPECS.get_attributes(&name)
                .map_or_else(
                    || abort!(Span::call_site(), format!("could not find {name:?} attribute")),
                    |attrs| matches!(
                        attrs.values().next().unwrap().values,
                        AttributeValue::Boolean { .. } | AttributeValue::YesNo { .. }
                    ),
                )
        });

    extend_input_struct(input, &extensions, requires_life_time, write_attribute_extension)
}

///
fn extract_attribute_extensions(args: &AttributeArgs) -> Vec<Extension> {
    let extensions = parse_extensions_args(
        args, "attribute or tag", true, validate_attribute_extension,
    ).flat_map(|ext| {
        let name = ext.name.to_string();

        if name == "global" {
            SPECS.get_attributes_of_category(AttributeCategory::GlobalAttributes).unwrap().into_iter()
                .filter_map(filter_attribute_key_value(&ext))
                .collect::<Vec<_>>()
        } else if SPECS.is_valid_attribute(&name) {
            vec![(format!("{}{}", ext.name, ext.ext_type == ExtType::Exclude), ext)]
        } else if let Some(tag) = SPECS.get_tag(&name) {
            attributes_of_tag(tag).into_iter()
                .filter_map(filter_attribute_key_value(&ext))
                .collect::<Vec<_>>()
        } else if let Ok(tags_category) = TagCategory::try_from(name.as_str()) {
            SPECS.get_tags_of_category(tags_category).unwrap().into_iter()
                .flat_map(attributes_of_tag)
                .filter_map(filter_attribute_key_value(&ext))
                .collect::<Vec<_>>()
        } else {
            unreachable!()
        }
    }).collect::<HashMap<_, _>>().into_iter()
        .map(|(_key, value)| value)
        .collect::<Vec<_>>();

    if extensions.is_empty() {
        abort!(Span::call_site(), "no attributes or tags were listed, remove 'attributes' macro or add an attribute or tag");
    }

    net_extensions(extensions, "no net attributes or tags to extend, check exclude list")
}

///
fn filter_attribute_key_value<'a>(ext: &'a Extension) -> impl Fn(&'a Attribute) -> Option<(String, Extension)> {
    |attr: &'a Attribute|
        if attr.name.starts_with("data-") {
            None
        } else {
            Some((
                format!("{}{}", attr.name, ext.ext_type == ExtType::Exclude),
                Extension {
                    name: Ident::new(attr.name.as_str(), ext.name.span()),
                    ext_type: ext.ext_type,
                    default: ext.default.clone(),
                }))
        }
}

///
fn write_attribute(attribute: &Attribute, life_time: &Option<Lifetime>, optional: bool) -> TokenStream {
    let attr_ident = Ident::new(&attribute.name, Span::call_site());

    let life_time = match life_time {
        Some(life_time) => quote! { #life_time },
        None => quote! {}
    };

    let ty = match &attribute.values {
        AttributeValue::OnOff { .. } |
        AttributeValue::Boolean { .. } => quote! { bool },
        AttributeValue::URLList { .. } => quote! { Vec<&#life_time str> },
        _ => quote! { &#life_time str },
    };

    if optional {
        quote! { #attr_ident: Option<#ty> }
    } else {
        quote! { #attr_ident: #ty }
    }
}

///
fn write_attribute_extension(extension: &Extension, life_time: &Option<Lifetime>) -> TokenStream {
    let name = extension.name.to_string();
    let props_attr = extension.default.as_ref()
        .map_or_else(|| if extension.ext_type == ExtType::Optional {
            quote! { #[props(optional)]}
        } else {
            quote! {}
        }, |default| {
            let lit = default.to_token_stream();

            if lit.is_empty() {
                quote! { #[props(default)] }
            } else {
                quote! { #[props(default = #lit)] }
            }
        });

    SPECS.get_attributes(&name)
        .map_or_else(
            || abort!(Span::call_site(), format!("could not find {name:?} attribute")),
            |attrs| {
                let html_attr = write_attribute(
                    attrs.values().next().unwrap(),
                    life_time,
                    extension.ext_type == ExtType::Optional,
                );

                quote! {
                    #props_attr
                    #html_attr
                }
            },
        )
}

///
fn validate_attribute_extension(path: &Path) {
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
}
