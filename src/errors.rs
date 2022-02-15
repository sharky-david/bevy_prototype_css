use std::fmt;
use cssparser::{BasicParseErrorKind, CowRcStr, ParseError, ParseErrorKind, Token};
use selectors::parser::SelectorParseErrorKind;

pub type BevyCssParsingError<'i> = ParseError<'i, BevyCssParsingErrorKind<'i>>;

#[derive(Debug, Clone)]
/// Higher level errors that describe why/how parsing has failed/is not possible/was rejected for a
/// CSS sheet or part thereof
/// Used for error reporting/printing, mainly by Parser Rules
pub enum BevyCssContextualError<'i> {
    /// An @-rule was encountered that is not recognised/supported
    UnsupportedAtRule(&'i str, BevyCssParsingError<'i>),
    /// An @-rule was encountered that is invalid
    InvalidAtRule(&'i str, BevyCssParsingError<'i>),
    /// A property name was declared that is not recognised/supported
    UnsupportedProperty(&'i str, BevyCssParsingError<'i>),
    /// A value was encountered that is invalid
    InvalidValue(&'i str, BevyCssParsingError<'i>),
}

impl<'i> BevyCssContextualError<'i> {
    #[inline]
    pub fn parsing_error(&self) -> &BevyCssParsingError {
        match *self {
            Self::UnsupportedAtRule(_, ref err) |
            Self::InvalidAtRule(_, ref err) |
            Self::UnsupportedProperty(_, ref err) |
            Self::InvalidValue(_, ref err) => err
        }
    }

    #[inline]
    pub fn error_string_with_location(&self) -> String {
        let location = self.parsing_error().location;
        format!(
            "Failed to parse css at (line: {}, col: {}): {}",
            location.line, location.column, self
        )
    }
}

impl<'i> fmt::Display for BevyCssContextualError<'i> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match *self {
            Self::UnsupportedAtRule(at_rule, ref err) =>
                write!(f,
                    "Unsupported/unrecognised @-rule ({}), {}",
                    at_rule, parse_error_2_str(err)
                ),
            Self::InvalidAtRule(at_rule, ref err) =>
                write!(f,
                       "Invalid @-rule ({}), {}",
                       at_rule, parse_error_2_str(err)
                ),
            Self::UnsupportedProperty(property, ref err) =>
                write!(f,
                       "Unsupported/unrecognised property name ({}), {}",
                       property, parse_error_2_str(err)
                ),
            Self::InvalidValue(_, ref err) =>
                write!(f,
                       "The value of a property is invalid: {}", parse_error_2_str(err)
                ),
        }
    }
}

#[derive(Debug, Clone)]
/// Detailed errors that may be declared while parsing parts of a CSS sheet
pub enum BevyCssParsingErrorKind<'i> {
    /// An invalid URL was encountered
    BadURLValue(CowRcStr<'i>),
    /// An invalid String was encountered
    BadStringValue(CowRcStr<'i>),
    /// An close parenthesis - `)` - was encountered
    UnbalancedCloseParenthesis,
    /// An close square bracket - `]` - was encountered
    UnbalancedCloseSquareBracket,
    /// An close curly bracket - `}` - was encountered
    UnbalancedCloseCurlyBracket,
    /// Declaration value still has input remaining after successful parsing
    DeclarationValueNotExhausted(CowRcStr<'i>),
    /// A dimension was given that does not match one of the expected types
    UnexpectedDimension(CowRcStr<'i>),
    /// A function was given that does not match one of the expected types
    UnexpectedFunction(CowRcStr<'i>),
    /// An @-rule was encountered that is not supported by this parser
    UnsupportedAtRule(CowRcStr<'i>),
    /// An error occurred while parsing a selector(s)
    SelectorError(SelectorParseErrorKind<'i>),
    /// A property was declared with an unknown name
    UnknownProperty(CowRcStr<'i>),
    /// A number (other than `0`) was given without a dimension (e.g. `px`) where a dimension is expected
    MissingDimension(Token<'i>),
    /// The keyword supplied is not supported by this parsing framework
    InvalidKeyword(CowRcStr<'i>),
    /// A value was given that is invalid in its context (but is still syntactically correct)
    InvalidValue(CowRcStr<'i>, Option<Token<'i>>),
    /// A function was used where it is not supported by this parsing framework
    FunctionNotSupported(CowRcStr<'i>),
    /// An unspecified or undefined error occurred.  Usually signifies low level parsing errors.
    UnspecifiedError,
}

impl<'i> From<SelectorParseErrorKind<'i>> for BevyCssParsingErrorKind<'i> {
    fn from(e: SelectorParseErrorKind<'i>) -> Self {
        Self::SelectorError(e)
    }
}

fn parse_error_2_str(err: &BevyCssParsingError) -> String {
    use ParseErrorKind::*;
    use BasicParseErrorKind::*;
    match err.kind {
        Basic(AtRuleBodyInvalid) => format!("The @-rule body is invalid"),
        Basic(AtRuleInvalid(ref at_rule)) => format!("The @-rule is invalid: {}", at_rule),
        Basic(EndOfInput) => format!("The end of input was reached unexpectedly"),
        Basic(QualifiedRuleInvalid) => format!("The qualified rule is invalid"),
        Basic(UnexpectedToken(ref token)) =>
            format!("An unexpected {} was found", error_token_2_str(token)),
        Custom(ref bevy_css_err) => format!("{:?}", bevy_css_err)
    }
}

fn error_token_2_str(token: &Token) -> String {
    match *token {
        Token::Ident(ref ident) => format!("identifier {}", ident),
        Token::AtKeyword(ref at_keyword) => format!("keyword @{}", at_keyword),
        Token::Hash(ref hash) => format!("hash #{}", hash),
        Token::IDHash(ref id_hash) => format!("id selector #{}", id_hash),
        Token::QuotedString(ref quoted_string) => format!("quoted string \"{}\"", quoted_string),
        Token::UnquotedUrl(ref unquoted_url) => format!("unquoted url {}", unquoted_url),
        Token::Delim(ref delim) => format!("delimiter {}", delim),
        Token::Number { int_value: Some(number), .. } => format!("number {}", number),
        Token::Number { value, .. } => format!("number {}", value),
        Token::Percentage { int_value: Some(percentage), .. } => format!("percentage {}", percentage),
        Token::Percentage { unit_value, .. } => format!("percentage {}", unit_value * 100.0),
        Token::Dimension { value, ref unit, .. } => format!("dimension {}{}", value, unit),
        Token::WhiteSpace(_) => format!("whitespace"),
        Token::Comment(_) => format!("comment"),
        Token::Colon => format!("colon (:)"),
        Token::Semicolon => format!("semicolon (;)"),
        Token::Comma => format!("comma (,)"),
        Token::IncludeMatch => format!("include match (~=)"),
        Token::DashMatch => format!("dash match (|=)"),
        Token::PrefixMatch => format!("prefix match (^=)"),
        Token::SuffixMatch => format!("suffix match ($=)"),
        Token::SubstringMatch => format!("substring match (*=)"),
        Token::CDO => format!("CDO (<!--)"),
        Token::CDC => format!("CDC (-->)"),
        Token::Function(ref func_name) => format!("function {}", func_name),
        Token::ParenthesisBlock => format!("parenthesis ("),
        Token::SquareBracketBlock => format!("square bracket ["),
        Token::CurlyBracketBlock => format!("curly bracket {{"),
        Token::BadUrl(_) => format!("bad url parse error"),
        Token::BadString(_) => format!("bad string parse error"),
        Token::CloseParenthesis => format!("unmatched close parenthesis"),
        Token::CloseSquareBracket => format!("unmatched close square bracket"),
        Token::CloseCurlyBracket => format!("unmatched close curly bracket"),
    }
}