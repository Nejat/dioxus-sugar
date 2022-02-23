use std::collections::HashMap;

use proc_macro2::{Ident, Span, TokenStream};
use quote::ToTokens;
use syn::{AttributeArgs, ItemStruct, Lifetime, Path};
use web_reference::prelude::*;

use crate::extend::{extend_input_struct, Extension, net_extensions, parse_extensions_args};
use crate::SPECS;

// todo: optional attribute properties

///
pub fn input_struct(input: &mut ItemStruct, args: &AttributeArgs) -> TokenStream {
    let extensions = extract_attribute_extensions(args);

    extend_input_struct(input, &extensions, write_attribute_extension)
}

///
fn attributes_of_tag(tag: &Tag) -> Vec<&Attribute> {
    let attributes = tag.attributes.iter()
        .chain(&tag.optional_attributes)
        .filter_map(|attr| SPECS.get_attribute(attr.as_str()));

    if tag.global_attributes {
        attributes.chain(
            SPECS.get_attributes_of_category(AttributeCategory::GlobalAttributes)
                .unwrap().iter().copied()
        ).collect::<Vec<_>>()
    } else {
        attributes.collect::<Vec<_>>()
    }
}

///
fn extract_attribute_extensions(args: &AttributeArgs) -> Vec<Extension> {
    let extensions = parse_extensions_args(
        args, "attribute or tag", validate_attribute_extension,
    ).flat_map(|ext| {
        let name = ext.name.to_string();


        if name == "global" {
            SPECS.get_attributes_of_category(AttributeCategory::GlobalAttributes).unwrap().into_iter()
                .filter_map(filter_attribute_key_value(&ext))
                .collect::<Vec<_>>()
        } else if SPECS.is_valid_attribute(&name) {
            vec![(format!("{}{}", ext.name, ext.exclude), ext)]
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
                format!("{}{}", attr.name, ext.exclude),
                Extension {
                    name: Ident::new(attr.name.as_str(), ext.name.span()),
                    exclude: ext.exclude,
                }))
        }
}

///
fn write_attribute(attribute: &Attribute) -> TokenStream {
    let attr_ident = Ident::new(&attribute.name, Span::call_site());
    let ty = Ident::new(match &attribute.values {
        AttributeValue::OnOff { .. } |
        AttributeValue::Boolean { .. } => "bool",
        AttributeValue::URLList { .. } => "Vec<String>",
        _ => "String",
    }, Span::call_site());

    quote! { #attr_ident: #ty }
}

///
fn write_attribute_extension(extension: &Extension, _life_time: &Option<Lifetime>) -> TokenStream {
    let name = extension.name.to_string();

    SPECS.get_attributes(&name)
        .map_or_else(
            || abort!(Span::call_site(), format!("could not find {name:?} attribute")),
            |attrs| write_attribute(attrs.values().next().unwrap()),
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
