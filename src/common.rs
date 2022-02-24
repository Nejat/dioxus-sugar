use web_reference::prelude::{Attribute, AttributeCategory, Tag};

use crate::SPECS;

///
pub fn attributes_of_tag(tag: &Tag) -> Vec<&Attribute> {
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
