use std::collections::HashMap;

use proc_macro2::{Ident, Span, TokenStream};
use quote::ToTokens;
use syn::{AttributeArgs, ItemStruct, Path};
use web_reference::prelude::*;

use crate::extend::{extend_input_struct, Extension, net_extensions, parse_extensions_args};
use crate::SPECS;

///
pub fn input_struct(input: &mut ItemStruct, args: &AttributeArgs) -> TokenStream {
    let extensions = extract_event_extensions(args);

    extend_input_struct(input, &extensions, write_event_extension)
}

///
fn extract_event_extensions(args: &AttributeArgs) -> Vec<Extension> {
    let extensions = parse_extensions_args(
        args, "event", validate_event_extension,
    ).flat_map(|ext| {
        if let Ok(event_category) = EventCategory::try_from(ext.name.to_string().as_str()) {
            SPECS.get_events_of_category(event_category).unwrap()
                .into_iter()
                .map(|attr| (
                    attr.name.clone(),
                    Extension {
                        name: Ident::new(&attr.name, ext.name.span()),
                        exclude: ext.exclude,
                    }
                )).collect::<Vec<_>>()
        } else {
            vec![(ext.name.to_string(), ext)]
        }
    }).collect::<HashMap<String, Extension>>().into_iter()
        .map(|(_key, value)| value).collect::<Vec<_>>();

    if extensions.is_empty() {
        abort!(Span::call_site(), "no events were listed, remove 'events' macro or add an event")
    }

    net_extensions(extensions, "no net events to extend, check exclude list")
}

///
fn write_event(event: &Event) -> TokenStream {
    let attr_ident = Ident::new(&event.name, Span::call_site());
    let ty = Ident::new(event.event_objects.iter().next().unwrap().as_str(), Span::call_site());

    quote! { #attr_ident: dioxus::events::#ty }
}

///
fn write_event_extension(extension: &Extension) -> TokenStream {
    let name = extension.name.to_string();

    SPECS.get_event(&name)
        .map_or_else(
            || abort!(Span::call_site(), format!("could not find {name:?} event")),
            write_event,
        )
}

///
fn validate_event_extension(path: &Path) {
    let event = path.to_token_stream().to_string();

    if SPECS.is_valid_event(&event) || EventCategory::try_from(event.as_str()).is_ok() {
        return;
    }

    abort!(&path.to_token_stream(), format!("invalid event {event:?}"))
}
