use std::sync::Arc;
use bevy::prelude::warn;
use cssparser::{
    AtRuleParser, CowRcStr, DeclarationListParser, DeclarationParser, Delimiter, ParseErrorKind,
    Parser, ParserState, QualifiedRuleParser, RuleListParser
};
use crate::{
    errors::{
        BevyCssContextualError, BevyCssParsingError, BevyCssParsingErrorKind
    },
    properties::BevyPropertyDeclaration,
    rules::{
        BevyCssRule, BevyStyleRule
    },
    selectors::BevySelectorList,
};

/// Handle CSS 'sheet' style strings with selectors, @-rules (currently ignored), etc.
pub struct BevySheetParser;

impl BevySheetParser {

    pub fn parse_with(input: &mut Parser) -> Vec<BevyCssRule> {
        let list_parser =
            RuleListParser::new_for_stylesheet(input, BevyTopLevelParser);
        let mut rules = Vec::new();
        for result in list_parser {
            match result {
                Ok(rule) => rules.push(rule),
                Err((err, bad_css)) =>
                    BevySheetParser::handle_error(err, bad_css),
            }
        }
        rules
    }

    fn handle_error<'i>(err: BevyCssParsingError<'i>, bad_css: &'i str) {
        warn!("{}", BevyCssContextualError::UnsupportedProperty(bad_css, err))
    }
}

/// Top level parser that may delegates parsing to more specialised parsers based on what is
/// encountered
pub struct BevyTopLevelParser;

impl<'i> QualifiedRuleParser<'i> for BevyTopLevelParser {    // aka 'normal' style rule parser
    type Prelude = BevySelectorList;
    type QualifiedRule = BevyCssRule;
    type Error = BevyCssParsingErrorKind<'i>;

    fn parse_prelude<'t>(                                    // Prelude here means selector list
        &mut self, input: &mut Parser<'i, 't>
    ) -> Result<Self::Prelude, BevyCssParsingError<'i>> {
        BevySelectorList::parse(input)
    }

    fn parse_block<'t>(                                      // For the bit between the curly braces
        &mut self,
        prelude: Self::Prelude,
        _start: &ParserState,
        input: &mut Parser<'i, 't>
    ) -> Result<Self::QualifiedRule, BevyCssParsingError<'i>> {
        let declarations = BevyPropertyListParser::parse_with(input);
        let style = BevyStyleRule {
            selectors: prelude,
            declarations: Arc::new(declarations),
        };
        Ok(BevyCssRule::Style(style))
    }
}

// @todo support @font-face
impl<'i> AtRuleParser<'i> for BevyTopLevelParser {
    type PreludeNoBlock = ();
    type PreludeBlock = ();
    type AtRule = BevyCssRule;
    type Error = BevyCssParsingErrorKind<'i>;
}

/// Parses a whole block of property declarations (e.g. between curly braces `{ ... }`).
pub struct BevyPropertyListParser;

impl BevyPropertyListParser {

    pub fn parse_with(input: &mut Parser) -> Vec<BevyPropertyDeclaration> {
        let list_parser =
            DeclarationListParser::new(input, BevyPropertyDeclarationParser);
        let mut declarations = Vec::new();
        for result in list_parser {
            match result {
                Ok(dec) => declarations.push(dec),
                Err((err, bad_css)) =>
                    BevyPropertyListParser::handle_error(err, bad_css),
            }
        }
        declarations
    }

    fn handle_error<'i>(err: BevyCssParsingError<'i>, bad_css: &'i str) {
        let contextual_error = match err.kind {
            ParseErrorKind::Custom(BevyCssParsingErrorKind::UnknownProperty(_)) =>
                BevyCssContextualError::UnsupportedProperty(bad_css, err),
            _ => BevyCssContextualError::InvalidValue(bad_css, err),
        };
        warn!("{}", contextual_error)
    }
}

/// Parses one single property declaration
pub struct BevyPropertyDeclarationParser;

impl<'i> DeclarationParser<'i> for BevyPropertyDeclarationParser {
    type Declaration = BevyPropertyDeclaration;
    type Error = BevyCssParsingErrorKind<'i>;

    fn parse_value<'t>(
        &mut self,
        name: CowRcStr<'i>,
        input: &mut Parser<'i, 't>
    ) -> Result<Self::Declaration, BevyCssParsingError<'i>> {

        let property = input.parse_until_before(Delimiter::Bang, |input| {
            // `parse_input` checks that a) the name is valid, and b) if it can parse the input
            BevyPropertyDeclaration::parse_input(name, input)
        })?;

        // Consume any `!important` rules
        // @fixme currently !important rules aren't treated any differently
        let _important = match input.try_parse(cssparser::parse_important) {
            Ok(()) => true,
            Err(_) => false,
        };

        input.expect_exhausted()?;       // Roll back (i.e. return err) if there is still input left

        Ok(property)
    }
}

impl<'i> AtRuleParser<'i> for BevyPropertyDeclarationParser {             // Required by `cssparser`
    type PreludeNoBlock = ();
    type PreludeBlock = ();
    type AtRule = BevyPropertyDeclaration;
    type Error = BevyCssParsingErrorKind<'i>;
}