use std::collections::HashSet;
use std::iter::FilterMap;

use proc_macro2::{Ident, TokenStream, TokenTree};
use proc_macro2::token_stream::IntoIter;
use quote::ToTokens;
use syn::{AttributeArgs, ItemStruct, Meta, NestedMeta};

const CLASS_ATTRIBUTE: &str = "class";

/// generates the [`Display`] implementation [`TokenStream`] for the
/// selected fields, designated with `class` attributes, of an input
/// [`ItemStruct`]
///
/// _* will modify the input [`ItemStruct`] if any of it's fields are
///  decorated with a `class` attribute_
pub fn impl_display_for_struct(
    input: &mut ItemStruct, args: &AttributeArgs,
) -> TokenStream {
    // find struct fields decorated with a class attribute
    let class_fields =
        input.fields.iter_mut()
            .filter_map(|field| {
                let class_attribute = field.attrs.iter()
                    .zip(0..field.attrs.len())
                    .find_map(
                        |(attr, idx)|
                            match attr.path.segments.first() {
                                Some(seg) if seg.ident == CLASS_ATTRIBUTE => Some(idx),
                                _ => None
                            }
                    );

                if let Some(index) = class_attribute {
                    let attributes = &mut field.attrs;

                    attributes.remove(index);

                    field.ident.as_ref()
                } else {
                    None
                }
            });

    // class fields listed in the 'classes' attribute
    let included = include_fields(args);

    // build token streams of included and listed class fields and their matching formatters
    let (formatter, fields) = class_fields.fold(
        included.fold(
            (String::new(), quote! {}),
            |(mut fmt, flds), nxt| {
                fmt.push_str("{} ");

                (fmt, quote! { #flds, self.#nxt })
            },
        ),
        |(mut fmt, flds), nxt| {
            fmt.push_str("{} ");

            (fmt, quote! { #flds, self.#nxt })
        },
    );

    // find any duplicate class definitions
    let duplicates = idents_only(&fields)
        .filter_map(|f1| {
            let occurrences: usize = idents_only(&fields)
                .filter_map(|f2| { if f1 == f2 { Some(1) } else { None } })
                .sum();

            if occurrences > 1 { Some(format!("{}", f1)) } else { None }
        })
        .collect::<HashSet<_>>();

    // abort if any duplicates found
    if !duplicates.is_empty() {
        let mut duplicates = duplicates.iter().collect::<Vec<_>>();

        duplicates.sort();

        abort!(
            input.to_token_stream(),
            "{} field{} tagged as 'class' more than once: {:?}",
            duplicates.len(), if duplicates.len() == 1 { " was" } else { "s were" },
            duplicates
        );
    }

    // if there isn't anything to list fail, implementing a blank Display can be confusing
    let writer_implementation = if formatter.is_empty() {
        abort_call_site!(
            "no fields were marked as `class`, remove 'classes' attribute from '{}' or tag one or more fields",
            input.ident
        );
    } else {
        let formatter = formatter.trim();

        quote! {
            write! {
                fmt, #formatter #fields
            }
        }
    };

    let input = &input.ident;

    // implement Display trait for input
    quote! {
        impl std::fmt::Display for #input {
            fn fmt(&self, fmt: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                #writer_implementation
            }
        }
    }
}

// return only the TokenTree::Ident instances, filter everything else out
fn idents_only(source: &TokenStream) -> FilterMap<IntoIter, fn(TokenTree) -> Option<Ident>> {
    const SELF_IDENT: &str = "self";

    source.to_token_stream()
        .into_iter()
        .filter_map(
            |tree|
                if let TokenTree::Ident(ident) = tree {
                    if ident == SELF_IDENT { None } else { Some(ident) }
                } else {
                    None
                }
        )
}

// parse attribute arguments list
fn include_fields(args: &AttributeArgs) -> impl Iterator<Item=&Ident> {
    const META_PATH: &str = "paths";
    const META_LIST: &str = "lists";
    const META_NAME_VALUE: &str = "named values";
    const META_LITERAL: &str = "literals";

    // only accept single segment ident paths
    return args.iter().filter_map(
        |arg| {
            match arg {
                NestedMeta::Meta(meta) =>
                    match meta {
                        Meta::Path(path) if path.segments.len() == 1 =>
                            path.segments.iter().map(|s| &s.ident).next(),

                        Meta::Path(unexpected) =>
                            fail(&unexpected.to_token_stream(), META_PATH),

                        Meta::List(unexpected) =>
                            fail(&unexpected.to_token_stream(), META_LIST),

                        Meta::NameValue(unexpected) =>
                            fail(&unexpected.to_token_stream(), META_NAME_VALUE),
                    },

                NestedMeta::Lit(unexpected) =>
                    fail(&unexpected.to_token_stream(), META_LITERAL),
            }
        }
    );

    #[inline]
    fn fail(tokens: &TokenStream, ty: &str) -> ! {
        abort!(tokens, "{} are not supported", ty);
    }
}
