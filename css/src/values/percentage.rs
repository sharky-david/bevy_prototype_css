use std::{
    cmp::Ordering,
    ops::Mul,
};
use cssparser::{Parser, Token};
use crate::{
    errors::{BevyCssParsingError, BevyCssParsingErrorKind},
    values::{
        generic::Numeric,
        parse::{AllowedValues, Parse},
    }
};

/// A `percentage` as specified in CSS with `<number>%`; and is some fraction of a reference
/// See also: https://drafts.csswg.org/css-values-4/#percentages
#[derive(Clone, Copy, Debug, Default)]
pub struct Percentage {
    /// `0%` to `100%` maps to `0.0` to `1.0` as a float
    pub value: f32,
    /// Used for `calc()` expressions to clamp the value within an allowed range
    pub clamping: Option<AllowedValues>
}

impl Percentage {
    pub(super) fn new_clamped(
        value: f32,
        clamping: Option<AllowedValues>,
    ) -> Self {
        Self { value, clamping, }
    }

    pub fn new(value: f32) -> Self {
        Self::new_clamped(value, None)
    }

    #[inline]
    pub fn hundred() -> Self {
        Self::new(1.0)
    }

    #[inline]
    pub fn is_hundred(&self) -> bool {
        self.value == 1.0
    }

    #[inline]
    pub fn is_calc(&self) -> bool {
        self.clamping.is_some()
    }

    /// Returns the fractional value of this `Percentage`, clamped as necessary
    #[inline]
    pub fn get(&self) -> f32 {
        self.clamping.map_or(
            self.value,
            |allowed| allowed.clamp(self.value)
        )
    }

    /// Mutates the `Percentage` in place. Returns the original (mutated) `Percentage`
    #[inline]
    pub fn reverse(mut self) -> Self {
        self.value = 1.0 - self.value;
        self
    }

    /// Mutates the `Percentage` in place.  Will limit the value to `100%` if it is greater.
    #[inline]
    pub fn limit_to_hundred(mut self) -> Self {
        self.value = self.value.min(1.0);
        self
    }

    /// Naively adds the value of both `Percentages`, losing any clamping bounds
    #[inline]
    pub fn try_sum(&self, that: &Self) -> Result<Self, ()> {
        Ok(Self::new(self.value + that.value))
    }

    /// It is the caller's responsibility to only pass `Token::Percentage` tokens
    pub(super) fn from_pc_token<'i>(
        token: &Token<'i>,
        allowed_values: AllowedValues,
    ) -> Result<Self, BevyCssParsingErrorKind<'i>> {
        assert!(matches!(token, Token::Percentage {..}));
        if let Token::Percentage { unit_value, .. } = *token {
            if allowed_values.is_ok(unit_value) { Ok(Percentage::new(unit_value)) }
            else {
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

    pub(super) fn parse_internal<'i, 't>(
        input: &mut Parser<'i, 't>,
        allowed_values: AllowedValues,
    ) -> Result<Self, BevyCssParsingError<'i>> {
        let start = input.current_source_location();
        let token = input.next()?;
        match *token {
            Token::Percentage { .. } =>
                Self::from_pc_token(token, allowed_values)
                    .map_err(|err| start.new_custom_error(err)),
            Token::Function(_) =>
                Self::from_func_token(token, allowed_values)
                    .map_err(|err| start.new_custom_error(err)),
            _ => Err(start.new_unexpected_token_error(token.clone())),
        }
    }
}

impl Numeric for Percentage {
    #[inline]
    fn zero() -> Self {
        Self::new(0.0)
    }

    #[inline]
    fn one() -> Self {
        Self::new(1.0)
    }

    #[inline]
    fn is_zero(&self) -> bool {
        self.value == 0.0
    }

    #[inline]
    fn is_negative(&self) -> bool {
        self.value < 0.0
    }

    fn is_infinite(&self) -> bool {
        self.value.is_infinite()
    }
}

impl PartialEq for Percentage {
    fn eq(&self, other: &Self) -> bool {
        self.value == other.value
    }
}

impl PartialOrd for Percentage {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        self.value.partial_cmp(&other.value)
    }
}

impl Mul<f32> for Percentage {
    type Output = Percentage;
    fn mul(self, rhs: f32) -> Self::Output {
        Percentage::new(self.value * rhs)
    }
}

impl Parse for Percentage {
    #[inline]
    fn parse<'i, 't>(input: &mut Parser<'i, 't>) -> Result<Self, BevyCssParsingError<'i>> {
        Self::parse_internal(input, AllowedValues::All)
    }
}