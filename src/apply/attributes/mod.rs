#![allow(dead_code)]

use proc_macro2::Ident;
use std::cmp::Ordering;

mod parse;
mod tokens;

#[derive(Debug, Clone, Eq, PartialEq, Hash)]
pub enum Property {
    Optional(String),
    Required(String),
}

impl PartialOrd<Self> for Property {
    #[inline]
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for Property {
    fn cmp(&self, other: &Self) -> Ordering {
        let self_value = match self { Self::Optional(value) | Self::Required(value) => value };
        let other_value = match other { Self::Optional(value) | Self::Required(value) => value };

        self_value.cmp(other_value)
    }
}

impl Property {
    pub const fn remap(&self, new_value: String) -> Self {
        match self {
            Self::Optional(_) => Self::Optional(new_value),
            Self::Required(_) => Self::Required(new_value),
        }
    }
}

impl AsRef<str> for Property {
    fn as_ref(&self) -> &str {
        match self {
            Self::Optional(value) |
            Self::Required(value) => value.as_str(),
        }
    }
}

///
pub struct ApplyAttributes {
    pub(crate) element: Ident,

    ///
    pub(crate) attributes: Vec<Property>,
}
