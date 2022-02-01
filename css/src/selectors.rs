use std::fmt;
use smallvec::SmallVec;

use cssparser::{
    Parser as CssParser,
    ToCss
};
use selectors::{
    attr::{AttrSelectorOperation, CaseSensitivity, NamespaceConstraint},
    context::{MatchingContext, MatchingMode, QuirksMode},
    matching::{matches_selector, ElementSelectorFlags},
    parser::{NonTSPseudoClass, PseudoElement, Parser as SelectorParser, Selector, SelectorImpl},
    SelectorList, Element, OpaqueElement
};

use crate::{
    css_strings::CssString,
    errors::{BevyCssParsingError, BevyCssParsingErrorKind},
};

/// A list of selectors that apply to a particular `BevyStyleRule`, as defined in a .css sheet
#[derive(Clone)]
pub struct BevySelectorList(pub SmallVec<[BevyCssSelector; 1]>);

// SelectorList<BevyCssSelectorKinds>;

impl BevySelectorList {
    #[inline]
    pub fn parse<'i, 't>(input: &mut CssParser<'i, 't>) -> Result<Self, BevyCssParsingError<'i>> {
        let selector_list = SelectorList::parse(
            &BevySelectorParser,
            input
        )?;
        let selectors = selector_list.0.into_iter().map(BevyCssSelector).collect();
        Ok(Self(selectors))
    }

    pub fn matches(&self, id: &Option<String>, classes: &SmallVec<[String; 1]>) -> bool {
        self.0.iter().any(|s| s.matches(id, classes))
    }
}

impl fmt::Display for BevySelectorList {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut first = true;
        for sel in self.0.iter() {
            if !first {
                f.write_str(", ")?;
            }
            first = false;
            sel.0.to_css(f)?;
        }
        Ok(())
    }
}

impl fmt::Debug for BevySelectorList {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        fmt::Display::fmt(self, f)
    }
}

/// A particular selector (as defined in a .css sheet) that could match an entity with the right
/// `id` and `classes`
#[derive(Clone)]
pub struct BevyCssSelector(Selector<BevyCssSelectorKinds>);

impl BevyCssSelector {
    #[inline]
    pub fn matches(&self, id: &Option<String>, classes: &SmallVec<[String; 1]>) -> bool {
        let mut context = MatchingContext::new(
            MatchingMode::Normal,
            None,
            None,
            QuirksMode::NoQuirks
        );
        let element = BevyElement { id, classes };
        matches_selector(
            &self.0,
            0,
            None,
            &element,
            &mut context,
            &mut |_, _| {}
        )
    }

    pub fn specificity(&self) -> u32 {
        self.0.specificity()
    }
}

impl fmt::Display for BevyCssSelector {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        self.0.to_css(f)
    }
}

impl fmt::Debug for BevyCssSelector {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        fmt::Display::fmt(self, f)
    }
}

#[derive(Debug, Copy, Clone, Ord, PartialOrd, Eq, PartialEq)]
pub struct BevyCssSelectorKinds;

impl SelectorImpl for BevyCssSelectorKinds {
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
        dest.write_str("")
    }
}

impl NonTSPseudoClass for BevyPseudoClass {
    type Impl = BevyCssSelectorKinds;
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
        dest.write_str("")
    }
}

impl PseudoElement for BevyPseudoElement {
    type Impl = BevyCssSelectorKinds;
}

pub struct BevySelectorParser;

impl<'i> SelectorParser<'i> for BevySelectorParser {
    type Impl = BevyCssSelectorKinds;
    type Error = BevyCssParsingErrorKind<'i>;
}

#[derive(Copy, Clone, Debug)]
struct BevyElement<'a> {
    id: &'a Option<String>,
    classes: &'a SmallVec<[String; 1]>
}

impl<'a> Element for BevyElement<'a> {
    type Impl = BevyCssSelectorKinds;

    #[inline]
    fn opaque(&self) -> OpaqueElement {
        OpaqueElement::new(self)
    }

    #[inline]
    fn parent_element(&self) -> Option<Self> {
        None
    }

    #[inline]
    fn parent_node_is_shadow_root(&self) -> bool {
        false
    }

    #[inline]
    fn containing_shadow_host(&self) -> Option<Self> {
        None
    }

    #[inline]
    fn is_pseudo_element(&self) -> bool {
        false
    }

    #[inline]
    fn prev_sibling_element(&self) -> Option<Self> {
        None
    }

    #[inline]
    fn next_sibling_element(&self) -> Option<Self> {
        None
    }

    #[inline]
    fn is_html_element_in_html_document(&self) -> bool {
        false
    }

    #[inline]
    fn has_local_name(&self, _local_name: &str) -> bool {
        false
    }

    #[inline]
    fn has_namespace(&self, _ns: &str) -> bool {
        false
    }

    #[inline]
    fn is_same_type(&self, _other: &Self) -> bool {
        false
    }

    #[inline]
    fn attr_matches(
        &self,
        _ns: &NamespaceConstraint<&CssString>,
        _local_name: &CssString,
        _operation: &AttrSelectorOperation<&CssString>
    ) -> bool {
        false
    }

    #[inline]
    fn match_non_ts_pseudo_class<F>(
        &self,
        _pc: &BevyPseudoClass,
        _context: &mut MatchingContext<Self::Impl>,
        _flags_setter: &mut F
    ) -> bool
        where F: FnMut(&Self, ElementSelectorFlags) {
        false
    }

    #[inline]
    fn match_pseudo_element(
        &self,
        _pe: &BevyPseudoElement,
        _context: &mut MatchingContext<Self::Impl>
    ) -> bool {
        false
    }

    #[inline]
    fn is_link(&self) -> bool {
        false
    }

    #[inline]
    fn is_html_slot_element(&self) -> bool {
        false
    }

    #[inline]
    fn has_id(&self, id: &CssString, case_sensitivity: CaseSensitivity) -> bool {
        match self.id {
            Some(id_str) => case_sensitivity.eq(id_str.as_bytes(), id.as_bytes()),
            None => false,
        }
    }

    #[inline]
    fn has_class(&self, name: &CssString, case_sensitivity: CaseSensitivity) -> bool {
        self.classes.iter().any(|class|
            case_sensitivity.eq(class.as_bytes(), name.as_bytes())
        )
    }

    #[inline]
    fn imported_part(&self, _name: &CssString) -> Option<CssString> {
        None
    }

    #[inline]
    fn is_part(&self, _name: &CssString) -> bool {
        false
    }

    #[inline]
    fn is_empty(&self) -> bool {
        false
    }

    #[inline]
    fn is_root(&self) -> bool {
        false
    }
}