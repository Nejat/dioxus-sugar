use proc_macro2::{Ident, Span, TokenStream};
use quote::ToTokens;

use crate::ApplyAttributes;

impl ToTokens for ApplyAttributes {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        let cx = &self.context;

        for attribute in &self.attributes {
            let attr = Ident::new(attribute.as_str(), Span::call_site());
            let value = quote! { {#cx.props.#attr} }.to_string();

            tokens.extend(quote! { #attr: #value, });
        }
    }
}
