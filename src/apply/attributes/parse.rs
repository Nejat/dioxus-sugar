use std::collections::HashSet;

use quote::ToTokens;
use syn::ExprPath;
use syn::parse::{Parse, ParseStream};
use syn::punctuated::Punctuated;
use web_reference::prelude::*;

use crate::{ApplyAttributes, SPECS};
use crate::common::attributes_of_tag;

impl Parse for ApplyAttributes {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        Ok(Self {
            context: input.parse()?,
            splitter: input.parse()?,
            attributes: parse_attributes(input)?,
        })
    }
}

fn parse_attributes(input: ParseStream) -> syn::Result<Vec<String>> {
    let mut include = Vec::new();
    let mut exclude = Vec::new();


    loop {
        let attribute = <ExprPath>::parse(input)?;

        let attr = attribute.to_token_stream().to_string();

        if attr == "exclude" {
            let content;

            parenthesized!(content in input);

            let excludes: Punctuated<ExprPath, Token![,]> = content.parse_terminated(ExprPath::parse)?;

            for attribute in excludes {
                let attr = attribute.to_token_stream().to_string();

                valid_attribute(attr.as_str(), input)?;

                exclude.push(attr);
            }

            break;
        }

        valid_attribute(attr.as_str(), input)?;

        include.push(attr);

        if !(input.peek(Token![,])) { break; }

        <Token![,]>::parse(input)?;
    }

    let include = expand_all_attributes(&include);
    let exclude = expand_all_attributes(&exclude);

    let mut attributes = include.into_iter()
        .filter(|inc| !exclude.contains(inc))
        .collect::<Vec<_>>();

    attributes.sort();

    Ok(attributes)
}

fn expand_all_attributes(attributes: &[String]) -> HashSet<String> {
    attributes.iter()
        .flat_map(|name| {
            if name == "global" {
                SPECS.get_attributes_of_category(AttributeCategory::GlobalAttributes).unwrap().into_iter()
                    .filter_map(filter_attribute)
                    .collect::<Vec<_>>()
            } else if SPECS.is_valid_attribute(name.as_str()) {
                vec![name.clone()]
            } else if let Some(tag) = SPECS.get_tag(name.as_str()) {
                attributes_of_tag(tag).into_iter()
                    .filter_map(filter_attribute)
                    .collect::<Vec<_>>()
            } else if let Ok(tags_category) = TagCategory::try_from(name.as_str()) {
                SPECS.get_tags_of_category(tags_category).unwrap().into_iter()
                    .flat_map(attributes_of_tag)
                    .filter_map(filter_attribute)
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
