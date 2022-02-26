use proc_macro2::{Ident, Span, TokenStream};
use quote::ToTokens;

use crate::apply::attributes::Property;
use crate::ApplyAttributes;

impl ToTokens for ApplyAttributes {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        const FORMAT_DEBUG_PARAMETER: &str = "{:?}";
        const FORMAT_DISPLAY_PARAMETER: &str = "{}";

        let element = &self.element;

        let mut attrs = TokenStream::new();

        for attribute in &self.attributes {
            let attr = Ident::new(attribute.as_ref(), Span::call_site());
            let opt = match attribute {
                Property::Optional(_) => FORMAT_DEBUG_PARAMETER,
                Property::Required(_) => FORMAT_DISPLAY_PARAMETER
            };

            attrs.extend(quote! {
                    dioxus_elements::#element.#attr(
                        __cx,
                        format_args_f!(#opt, cx.Props.#attr)
                    ),
                });
        }

        println!("{attrs}\n");

        tokens.extend(quote! { [ #attrs ] });
    }
}

/*

dioxus_elements::div.accesskey          (__cx, format_args_f! ("{}", cx.Props.accesskey:?)),
dioxus_elements::div.contenteditable    (__cx, format_args_f! ("{}", cx.Props.contenteditable:?)),
dioxus_elements::div.dir                (__cx, format_args_f! ("{}", cx.Props.dir:?)),
dioxus_elements::div.draggable          (__cx, format_args_f! ("{}", cx.Props.draggable:?)),
dioxus_elements::div.hidden             (__cx, format_args_f! ("{}", cx.Props.hidden:?)),
dioxus_elements::div.lang               (__cx, format_args_f! ("{}", cx.Props.lang:?)),
dioxus_elements::div.spellcheck         (__cx, format_args_f! ("{}", cx.Props.spellcheck:?)),
dioxus_elements::div.style              (__cx, format_args_f! ("{}", cx.Props.style:?)),
dioxus_elements::div.tabindex           (__cx, format_args_f! ("{}", cx.Props.tabindex:?)),
dioxus_elements::div.title              (__cx, format_args_f! ("{}", cx.Props.title:?)),
dioxus_elements::div.translate          (__cx, format_args_f! ("{}", cx.Props.translate:?)),

 */
