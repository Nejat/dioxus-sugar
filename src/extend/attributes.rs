use proc_macro::Span;

use proc_macro2::{Ident, TokenStream};
use quote::ToTokens;
use syn::{AttributeArgs, ItemStruct, Path};
use web_reference::prelude::{AttributeCategory, TagCategory};

use crate::extend::{check_net_extensions, Extension, parse_extensions_args};
use crate::SPECS;

///
pub fn input_struct(
    input: &mut ItemStruct, args: &AttributeArgs,
) -> TokenStream {
    let _extensions = extract_attribute_extensions(args);

    quote! { #input }
}

///
fn extract_attribute_extensions(args: &AttributeArgs) -> Vec<Extension> {
    let extensions = parse_extensions_args(
        args, "attribute or tag", validate_attribute_extension,
    ).flat_map(|ext| {
        if ext.name == "global" {
            SPECS.get_attributes_of_category(AttributeCategory::GlobalAttributes).unwrap()
                .into_iter()
                .map(|attr| Extension {
                    name: Ident::new(attr.name.trim_end_matches("-*"), ext.name.span()),
                    exclude: ext.exclude,
                })
                .collect::<Vec<_>>()
        } else if let Ok(tags_category) = TagCategory::try_from(ext.name.to_string().as_str()) {
            SPECS.get_tags_of_category(tags_category).unwrap()
                .into_iter()
                .map(|attr| Extension {
                    name: Ident::new(attr.name.trim_end_matches("-*"), ext.name.span()),
                    exclude: ext.exclude,
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

    extensions
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
