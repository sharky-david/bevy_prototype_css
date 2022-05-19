use std::fmt::Debug;
use cssparser::Parser;
use crate::{
    errors::BevyCssParsingError,
    values::Parse,
};

/// Common template for numeric value types
pub trait Numeric {
    fn zero() -> Self where Self: Sized;
    fn one() -> Self where Self: Sized;
    fn is_zero(&self) -> bool;
    fn is_negative(&self) -> bool;
    fn is_infinite(&self) -> bool;
}

/// Wrapper type where the value must not be negative
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct NonNegative<NumericType: Numeric>(pub NumericType);

impl<NumericType: Numeric> Numeric for NonNegative<NumericType> {
    #[inline]
    fn zero() -> Self {
        Self(NumericType::zero())
    }

    #[inline]
    fn one() -> Self {
        Self(NumericType::one())
    }

    #[inline]
    fn is_zero(&self) -> bool {
        self.0.is_zero()
    }

    #[inline]
    fn is_negative(&self) -> bool {
        self.0.is_negative()
    }

    #[inline]
    fn is_infinite(&self) -> bool { self.0.is_infinite() }
}

/// Wrapper type where the `auto` keyword can be used
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum MaybeAuto<T>  {
    Auto,
    NotAuto(T),
}

impl<T: Clone + Copy> MaybeAuto<T> {
    #[inline]
    pub fn auto() -> Self {
        Self::Auto
    }

    #[inline]
    pub fn is_auto(&self) -> bool {
        matches!(*self, Self::Auto)
    }

    #[inline]
    pub fn auto_eval(
        &self,
        func: impl FnOnce() -> T
    ) -> T {
        match self {
            Self::NotAuto(val) => val.clone(),
            Self::Auto => func(),
        }
    }

    #[inline]
    pub fn non_auto(&self) -> Option<T> {
        match self {
            Self::NotAuto(val) => Some(val.clone()),
            Self::Auto => None,
        }
    }

    pub fn parse_maybe_auto<'i, 't>(
        input: &mut Parser<'i, 't>,
        parser: impl FnOnce(&mut Parser<'i, 't>) -> Result<T, BevyCssParsingError<'i>>
    ) -> Result<Self, BevyCssParsingError<'i>> {
        if input.try_parse(|i| i.expect_ident_matching("auto")).is_ok() {
            Ok(Self::Auto)
        } else {
            Ok(Self::NotAuto( parser(input)? ))
        }
    }
}

impl<T: Numeric> Numeric for MaybeAuto<T> {

    #[inline]
    fn zero() -> Self {
        Self::NotAuto(<T as Numeric>::zero())
    }

    #[inline]
    fn one() -> Self {
        Self::NotAuto(<T as Numeric>::one())
    }

    #[inline]
    fn is_zero(&self) -> bool {
        match *self {
            Self::Auto => false,
            Self::NotAuto(ref num) => num.is_zero(),
        }
    }

    #[inline]
    fn is_negative(&self) -> bool {
        match *self {
            Self::Auto => false,
            Self::NotAuto(ref num) => num.is_negative(),
        }
    }

    #[inline]
    fn is_infinite(&self) -> bool {
        match *self {
            Self::Auto => false,
            Self::NotAuto(ref num) => num.is_infinite(),
        }
    }
}

impl<T: Parse + Clone + Copy> Parse for MaybeAuto<T> {
    #[inline]
    fn parse<'i, 't>(input: &mut Parser<'i, 't>) -> Result<Self, BevyCssParsingError<'i>> {
        Self::parse_maybe_auto(input, <T as Parse>::parse)
    }
}