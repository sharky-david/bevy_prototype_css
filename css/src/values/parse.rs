use std::fmt;
use cssparser::{CowRcStr, Parser};
use crate::errors::BevyCssParsingError;

/// Interface for values to call the appropriate parsing code
pub trait Parse: Sized {
    fn parse<'i, 't>(
        input: &mut Parser<'i, 't>
    ) -> Result<Self, BevyCssParsingError<'i>>;
}

/// Parsing where `none` could be used
impl<P: Parse> Parse for Option<P> {
    #[inline]
    fn parse<'i, 't>(input: &mut Parser<'i, 't>) -> Result<Self, BevyCssParsingError<'i>> {
        if input.try_parse(|i| i.expect_ident_matching("none")).is_ok() {
            Ok(None)
        } else {
            Ok(
                Some(<P as Parse>::parse(input)?)
            )
        }
    }
}

/// Used to (possibly) restrict the range of values an internal parsing function will take.
#[derive(Clone, Copy, Debug)]
pub enum AllowedValues {
    All,
    NonNegative,
    AtLeastOne,
}

impl Default for AllowedValues {
    #[inline]
    fn default() -> Self {
        AllowedValues::All
    }
}

impl fmt::Display for AllowedValues {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{:?}", self)
    }
}

impl<'i> Into<CowRcStr<'i>> for AllowedValues {
    fn into(self) -> CowRcStr<'i> {
        self.to_string().into()
    }
}

impl AllowedValues {
    #[inline]
    pub fn is_ok(&self, value: f32) -> bool {
        match *self {
            Self::All => true,
            Self::NonNegative => value >= 0.0,
            Self::AtLeastOne => value >= 1.0,
        }
    }

    #[inline]
    pub fn clamp(&self, value: f32) -> f32 {
        match *self {
            Self::NonNegative if value < 0.0 => 0.0,
            Self::AtLeastOne if value < 1.0 => 1.0,
            _ => value
        }
    }
}

