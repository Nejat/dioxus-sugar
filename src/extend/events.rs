use proc_macro::Span;

use proc_macro2::{Ident, TokenStream};
use quote::ToTokens;
use syn::{AttributeArgs, ItemStruct, Path};
use web_reference::prelude::*;

use crate::extend::{check_net_extensions, Extension, parse_extensions_args};
use crate::SPECS;

///
pub fn input_struct(
    input: &mut ItemStruct, args: &AttributeArgs,
) -> TokenStream {
    let _extensions = extract_event_extensions(args);

    quote! { #input }
}

///
fn extract_event_extensions(args: &AttributeArgs) -> Vec<Extension> {
    let extensions = parse_extensions_args(
        args, "event", validate_event_extension,
    ).flat_map(|ext| {
        if let Ok(event_category) = EventCategory::try_from(ext.name.to_string().as_str()) {
            SPECS.get_events_of_category(event_category).unwrap()
                .into_iter()
                .map(|attr| Extension {
                    name: Ident::new(&attr.name, ext.name.span()),
                    exclude: ext.exclude,
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

    extensions
}

///
fn validate_event_extension(path: &Path) {
    let event = path.to_token_stream().to_string();

    if SPECS.is_valid_event(&event) || EventCategory::try_from(event.as_str()).is_ok() {
        return;
    }

    abort!(&path.to_token_stream(), format!("invalid event {event:?}"))
}
