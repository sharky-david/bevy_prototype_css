use std::ops::Mul;
use cssparser::{Parser, Token};
use crate::{
    errors::{BevyCssParsingError, BevyCssParsingErrorKind},
    values::{
        generic::{NonNegative, Numeric, MaybeAuto},
        parse::{AllowedValues, Parse},
    },
};

/// A bare number, without units or `%`.  Can be teh result of a `calc()` css function.
/// See also: https://drafts.csswg.org/css-values-3/#number-value
#[derive(Clone, Copy, Debug, PartialEq, PartialOrd)]
pub struct Number(pub f32);

impl Number {
    /// It is the caller's responsibility to only pass `Token::Number` tokens
    pub(crate) fn from_num_token<'i>(
        token: &Token<'i>,
        allowed_values: AllowedValues,
    ) -> Result<Self, BevyCssParsingErrorKind<'i>> {
        assert!(matches!(token, Token::Number {..}));
        if let Token::Number { value, .. } = *token {
            if allowed_values.is_ok(value) {
                Ok(Self(value))
            } else {
                Err(BevyCssParsingErrorKind::InvalidValue(
                    allowed_values.into(),
                    Some(token.clone())
                ))
            }
        } else { unreachable!() }
    }

    /// It is the caller's responsibility to only pass `Token::Function` tokens
    pub(super) fn from_func_token<'i>(
        token: &Token<'i>,
        allowed_values: AllowedValues,
    ) -> Result<Self, BevyCssParsingErrorKind<'i>> {
        assert!(matches!(token, Token::Function(_)));
        if let Token::Function(ref name) = *token {
            // @todo Add support for css `calc()` functions
            Err(BevyCssParsingErrorKind::FunctionNotSupported(name.to_owned()))
        } else { unreachable!() }
    }

    pub fn parse_internal<'i, 't>(
        input: &mut Parser<'i, 't>,
        allowed_values: AllowedValues,
    ) -> Result<Self, BevyCssParsingError<'i>> {
        let start = input.current_source_location();
        let token = input.next()?;
        match *token {
            Token::Number { .. } =>
                Self::from_num_token(token, allowed_values)
                    .map_err(|err| start.new_custom_error(err)),
            Token::Function(ref name) =>
                Self::from_func_token(token, allowed_values)
                    .map_err(|err| start.new_custom_error(err)),
            _ => Err(start.new_unexpected_token_error(token.clone()))
        }
    }
}

impl From<Number> for f32 {
    #[inline]
    fn from(num: Number) -> Self {
        num.0
    }
}

impl Numeric for Number {
    #[inline]
    fn zero() -> Self {
        Self(0.0)
    }

    #[inline]
    fn one() -> Self {
        Self(1.0)
    }

    #[inline]
    fn is_zero(&self) -> bool {
        self.0 == 0.0
    }

    #[inline]
    fn is_negative(&self) -> bool {
        self.0 < 0.0
    }

    #[inline]
    fn is_infinite(&self) -> bool {
        self.0.is_infinite()
    }
}

impl Mul<f32> for Number {
    type Output = Self;
    #[inline]
    fn mul(self, rhs: f32) -> Self::Output {
        Self(self.0 * rhs)
    }
}

impl Parse for Number {
    fn parse<'i, 't>(input: &mut Parser<'i, 't>) -> Result<Self, BevyCssParsingError<'i>> {
        Self::parse_internal(input, AllowedValues::All)
    }
}

/// A wrapper around `Number` that disallows negative values (i.e. < 0.0)
pub type NonNegativeNumber = NonNegative<Number>;

impl From<NonNegativeNumber> for f32 {
    fn from(num: NonNegativeNumber) -> Self {
        num.0.into()
    }
}

impl Parse for NonNegativeNumber {
    #[inline]
    fn parse<'i, 't>(input: &mut Parser<'i, 't>) -> Result<Self, BevyCssParsingError<'i>> {
        Ok(Self(
            Number::parse_internal(input, AllowedValues::NonNegative)?
        ))
    }
}

/// A wrapper around `Number` that allows the use of `auto`
pub type NumberOrAuto = MaybeAuto<Number>;

/// A wrapper around `NonNegativeLength` that allows the use of `auto`
pub type NonNegativeNumberOrAuto = MaybeAuto<NonNegativeNumber>;