use std::collections::HashSet;

use quote::ToTokens;
use syn::ExprPath;
use syn::parse::{Parse, ParseStream};
use syn::punctuated::Punctuated;
use web_reference::prelude::*;

use crate::{ApplyAttributes, SPECS};
use crate::apply::attributes::Property;
use crate::common::attributes_of_tag;

impl Parse for ApplyAttributes {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        Ok(Self {
            element: {
                let elm = input.parse()?;

                <Token![;]>::parse(input)?;

                elm
            },
            attributes: parse_attributes(input)?,
        })
    }
}

fn parse_attributes(input: ParseStream) -> syn::Result<Vec<Property>> {
    let mut include = Vec::new();
    let mut exclude = Vec::new();
    let mut optional = Vec::new();

    loop {
        let attribute = <ExprPath>::parse(input)?;

        let attr = attribute.to_token_stream().to_string();

        match attr.as_str() {
            "exclude" => parse_list_of_attributes(&mut exclude, input, Property::Required)?,
            "optional" => parse_list_of_attributes(&mut optional, input, Property::Optional)?,
            _ => {
                valid_attribute(attr.as_str(), input)?;

                include.push(Property::Required(attr));
            }
        }

        if !(input.peek(Token![,])) { break; }

        <Token![,]>::parse(input)?;
    }

    let include = expand_all_attributes(
        include.into_iter()
            .chain(optional.into_iter())
    );

    let exclude = expand_all_attributes(exclude);

    let mut attributes = include.into_iter()
        .filter(|inc| exclude.iter().all(|ex| ex.as_ref() != inc.as_ref()))
        .collect::<Vec<_>>();

    attributes.sort();

    Ok(attributes)
}

fn parse_list_of_attributes(
    list: &mut Vec<Property>, input: ParseStream, parse: fn(String) -> Property,
) -> syn::Result<()> {
    let content;

    parenthesized!(content in input);

    let excludes: Punctuated<ExprPath, Token![,]> = content.parse_terminated(ExprPath::parse)?;

    for attribute in excludes {
        let attr = attribute.to_token_stream().to_string();

        valid_attribute(attr.as_str(), input)?;

        list.push(parse(attr));
    }

    Ok(())
}

fn expand_all_attributes(attributes: impl IntoIterator<Item=Property>) -> HashSet<Property> {
    attributes.into_iter()
        .flat_map(|attribute| {
            if attribute.as_ref() == "global" {
                SPECS.get_attributes_of_category(AttributeCategory::GlobalAttributes).unwrap().into_iter()
                    .filter_map(filter_attribute)
                    .map(|new_attr| attribute.remap(new_attr))
                    .collect::<Vec<_>>()
            } else if SPECS.is_valid_attribute(attribute.as_ref()) {
                vec![attribute]
            } else if let Some(tag) = SPECS.get_tag(attribute.as_ref()) {
                attributes_of_tag(tag).into_iter()
                    .filter_map(filter_attribute)
                    .map(|new_attr| attribute.remap(new_attr))
                    .collect::<Vec<_>>()
            } else if let Ok(tags_category) = TagCategory::try_from(attribute.as_ref()) {
                SPECS.get_tags_of_category(tags_category).unwrap().into_iter()
                    .flat_map(attributes_of_tag)
                    .filter_map(filter_attribute)
                    .map(|new_attr| attribute.remap(new_attr))
                    .collect::<Vec<_>>()
            } else {
                unreachable!()
            }
        }).collect()
}

fn filter_attribute(attr: &Attribute) -> Option<String> {
    if attr.name.starts_with("data-") {
        None
    } else {
        Some(attr.name.clone())
    }
}

#[inline]
fn valid_attribute(attr: &str, input: ParseStream) -> syn::Result<()> {
    if attr == "global" ||
        SPECS.is_valid_attribute(attr) ||
        SPECS.is_valid_tag(attr) ||
        TagCategory::try_from(attr).is_ok()
    {
        Ok(())
    } else {
        Err(input.error(format!("{attr:?} is not a valid attribute, tag or tag category")))
    }
}
