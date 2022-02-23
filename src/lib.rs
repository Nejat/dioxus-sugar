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
//!
//! A collection of macros for simplifying the creation of custom [`Dioxus`] elements
//!
//! - `classes` attribute implements the [`Display`] trait for selected fields on a
//!    properties `struct`
//! - `attributes` extends properties `struct` with standard html dom attributes, by name,
//!    by tag, by tag category
//! -  `events` extends properties `struct` with standard html dom events by name, by event
//!    category
//!
//! [`Dioxus`] (https://dioxuslabs.com/)

#[macro_use]
extern crate lazy_static;
#[macro_use]
extern crate proc_macro_error;
#[macro_use]
extern crate quote;
#[macro_use]
extern crate syn;

use proc_macro::TokenStream;

use web_reference::prelude::*;

mod extend;
mod class;

#[cfg(test)]
mod publish_tests;

// todo: macro to apply props to component? analyze feasibility
// todo: detailed documentation
// todo: extend events by events of a tag
// todo: can't test html dom attribute, tag, event, etc. validity in failed compile tests
//       specifically invalid attributes, invalid events and missing lifetime in events

lazy_static! {
    pub(crate) static ref SPECS: WebReference = WebReference::load_specs().unwrap();
}

/// # `attributes` Attribute
#[proc_macro_attribute]
#[proc_macro_error]
pub fn attributes(attr: TokenStream, item: TokenStream) -> TokenStream {
    let args = parse_macro_input!(attr as syn::AttributeArgs);
    let mut source = parse_macro_input!(item as syn::ItemStruct);

    let extended = extend::attributes::input_struct(&mut source, &args);

    (quote! { #extended }).into()
}

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

/// # `events` Attribute
#[proc_macro_attribute]
#[proc_macro_error]
pub fn events(attr: TokenStream, item: TokenStream) -> TokenStream {
    let args = parse_macro_input!(attr as syn::AttributeArgs);
    let mut source = parse_macro_input!(item as syn::ItemStruct);

    let extended = extend::events::input_struct(&mut source, &args);

    (quote! { #extended }).into()
}
