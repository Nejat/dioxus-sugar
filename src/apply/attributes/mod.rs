#![allow(dead_code)]

use syn::ExprPath;

mod parse;
mod tokens;

///
pub struct ApplyAttributes {
    ///
    pub(crate) context: ExprPath,

    ///
    pub(crate) splitter: Token![;],

    ///
    pub(crate) attributes: Vec<String>,
}
