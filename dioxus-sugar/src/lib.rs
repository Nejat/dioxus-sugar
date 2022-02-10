#![deny(clippy::all)]
#![deny(clippy::pedantic)]
#![deny(clippy::nursery)]
#![deny(clippy::cargo)]
#![deny(missing_docs)]
// ==============================================================
#![allow(clippy::module_name_repetitions)]
#![allow(clippy::items_after_statements)]
// ==============================================================
#![doc(html_root_url = "https://docs.rs/dioxus-sugar/0.1.0")]

//! # Dioxus Sugar

#[macro_use]
extern crate proc_macro_error;
#[macro_use]
extern crate quote;
#[macro_use]
extern crate syn;

use proc_macro::TokenStream;

mod class;

#[cfg(test)]
mod publish_tests;

/// # `classes` Attribute
#[proc_macro_attribute]
#[proc_macro_error]
pub fn classes(attr: TokenStream, item: TokenStream) -> TokenStream {
    let args = parse_macro_input!(attr as syn::AttributeArgs);
    let mut source = parse_macro_input!(item as syn::ItemStruct);

    let impl_display = class::impl_display_for_struct(&mut source, &args);

    (quote! {
        #source

        #impl_display
    }).into()
}
