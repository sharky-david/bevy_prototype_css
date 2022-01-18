use std::fmt;
use cssparser::ToCss;
use selectors::{
    parser::{NonTSPseudoClass, PseudoElement},
    SelectorList,
};
use crate::{
    css_strings::CssString,
    errors::BevyCssParsingErrorKind,
};

pub type BevySelectorList = SelectorList<BevyCssSelector>;

#[derive(Debug, Copy, Clone, Ord, PartialOrd, Eq, PartialEq, Hash)]
pub struct BevyCssSelector;

impl selectors::parser::SelectorImpl for BevyCssSelector {
    type ExtraMatchingData = ();
    type AttrValue = CssString;
    type Identifier = CssString;
    type LocalName = CssString;
    type NamespaceUrl = CssString;
    type NamespacePrefix = CssString;
    type BorrowedNamespaceUrl = str;
    type BorrowedLocalName = str;
    type NonTSPseudoClass = BevyPseudoClass;
    type PseudoElement = BevyPseudoElement;
}

#[derive(Clone, PartialEq, Eq)]
pub struct BevyPseudoClass;

impl ToCss for BevyPseudoClass {
    fn to_css<W>(&self, dest: &mut W) -> fmt::Result where W: fmt::Write {
        match *self {
            _ => dest.write_str("")
        }
    }
}

impl NonTSPseudoClass for BevyPseudoClass {
    type Impl = BevyCssSelector;

    fn is_active_or_hover(&self) -> bool {
        false
    }

    fn is_user_action_state(&self) -> bool {
        false
    }
}

#[derive(Clone, PartialEq, Eq)]
pub struct BevyPseudoElement;

impl ToCss for BevyPseudoElement {
    fn to_css<W>(&self, dest: &mut W) -> fmt::Result where W: fmt::Write {
        match *self {
            _ => dest.write_str("")
        }
    }
}

impl PseudoElement for BevyPseudoElement {
    type Impl = BevyCssSelector;
}

pub struct BevySelectorParser;

impl<'i> selectors::parser::Parser<'i> for BevySelectorParser {
    type Impl = BevyCssSelector;
    type Error = BevyCssParsingErrorKind<'i>;
}