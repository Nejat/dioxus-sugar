use proc_macro2::{Ident, Span, TokenStream};
use quote::ToTokens;

use crate::apply::attributes::Property;
use crate::ApplyAttributes;

impl ToTokens for ApplyAttributes {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        let cx = &self.context;

        for attribute in &self.attributes {
            let attr = Ident::new(attribute.as_ref(), Span::call_site());

            let opt = match attribute {
                Property::Optional(_) => ":?",
                Property::Required(_) => "",
            };

            let value = format!("{{{}.props.{}{}}}", cx.to_token_stream(), attribute.as_ref(), opt);

            tokens.extend(quote! { #attr: #value, });
        }
    }
}
