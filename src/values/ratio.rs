use std::cmp::Ordering;
use cssparser::Parser;
use crate::errors::BevyCssParsingError;
use crate::values::{
    generic::{MaybeAuto, Numeric},
    NonNegativeNumber, Parse,
};

/// Type accepting a ratio as either a fraction (`0.5`) or between two (non-negative) numbers (`1/2`)
/// See also: https://drafts.csswg.org/css-values-4/#ratio-value
/// See also: https://developer.mozilla.org/en-US/docs/Web/CSS/ratio
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Ratio(pub NonNegativeNumber, pub NonNegativeNumber);

impl Ratio {
    #[inline]
    pub fn is_degenerate(&self) -> bool {
        self.0.is_zero() || self.0.is_infinite() ||
            self.1.is_zero() || self.1.is_infinite()
    }

    #[inline]
    pub fn as_fraction(&self) -> f32 {
        f32::from(self.0) / f32::from(self.1)
    }
}

impl PartialOrd for Ratio {
    #[inline]
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        self.as_fraction().partial_cmp(&other.as_fraction())
    }
}

impl Parse for Ratio {
    fn parse<'i, 't>(input: &mut Parser<'i, 't>) -> Result<Self, BevyCssParsingError<'i>> {
        let a = NonNegativeNumber::parse(input)?;
        let delim = input.try_parse(|i| i.expect_delim('/'));
        let b = match delim {
            Ok(()) => NonNegativeNumber::parse(input)?,
            _ => NonNegativeNumber::one(),
        };
        Ok(Ratio(a, b))
    }
}

/// A ratio, where the `auto` keyword could be used as well
pub type RatioOrAuto = MaybeAuto<Ratio>;

#[cfg(test)]
mod tests {
    use crate::values::generic::NonNegative;
    use crate::values::Number;
    use super::*;

    fn ratio_from(a: f32, b: f32) -> Ratio {
        Ratio(
            NonNegative(Number(a)),
            NonNegative(Number(b))
        )
    }

    #[test]
    fn test_degenerate() {
        assert!(ratio_from(1.0, 0.0).is_degenerate());
        assert!(ratio_from(0.0, 1.0).is_degenerate());
        assert!(ratio_from(0.0, 0.0).is_degenerate());
        assert!(!ratio_from(1.0, 1.0).is_degenerate());
        assert!(!ratio_from(1.0, 2.0).is_degenerate());
    }

    #[test]
    fn test_fraction() {
        assert_eq!(ratio_from(1.0, 1.0).as_fraction(), 1.0);
        assert_eq!(ratio_from(2.0, 1.0).as_fraction(), 2.0);
        assert_eq!(ratio_from(1.0, 2.0).as_fraction(), 0.5);
    }

    #[test]
    fn test_parse_degenerate() {
        assert!(Ratio::parse_str("1 / 0").unwrap().is_degenerate());
        assert!(Ratio::parse_str("1.0 / 0").unwrap().is_degenerate());
        assert!(Ratio::parse_str("1 / 0.0").unwrap().is_degenerate());
        assert!(Ratio::parse_str("1.0 / 0.0").unwrap().is_degenerate());
        assert!(Ratio::parse_str("0 / 1").unwrap().is_degenerate());
        assert!(Ratio::parse_str("0.0 / 1").unwrap().is_degenerate());
        assert!(Ratio::parse_str("0 / 1.0").unwrap().is_degenerate());
        assert!(Ratio::parse_str("0.0 / 1.0").unwrap().is_degenerate());
        assert!(Ratio::parse_str("0 / 0").unwrap().is_degenerate());
        assert!(Ratio::parse_str("0.0 / 0").unwrap().is_degenerate());
        assert!(Ratio::parse_str("0 / 0.0").unwrap().is_degenerate());
        assert!(Ratio::parse_str("0.0 / 0.0").unwrap().is_degenerate());
        assert!(!Ratio::parse_str("1 / 1").unwrap().is_degenerate());
        assert!(!Ratio::parse_str("1.0 / 1").unwrap().is_degenerate());
        assert!(!Ratio::parse_str("1 / 1.0").unwrap().is_degenerate());
        assert!(!Ratio::parse_str("1.0 / 1.0").unwrap().is_degenerate());
        assert!(!Ratio::parse_str("1 / 2").unwrap().is_degenerate());
        assert!(!Ratio::parse_str("1.0 / 2").unwrap().is_degenerate());
        assert!(!Ratio::parse_str("1 / 2.0").unwrap().is_degenerate());
        assert!(!Ratio::parse_str("1.0 / 2.0").unwrap().is_degenerate());
    }

    #[test]
    fn test_parse_fraction() {
        assert_eq!(Ratio::parse_str("1 / 1").unwrap().as_fraction(), 1.0);
        assert_eq!(Ratio::parse_str("1.0 / 1").unwrap().as_fraction(), 1.0);
        assert_eq!(Ratio::parse_str("1 / 1.0").unwrap().as_fraction(), 1.0);
        assert_eq!(Ratio::parse_str("1.0 / 1.0").unwrap().as_fraction(), 1.0);
        assert_eq!(Ratio::parse_str("1.0").unwrap().as_fraction(), 1.0);
        assert_eq!(Ratio::parse_str("2 / 1").unwrap().as_fraction(), 2.0);
        assert_eq!(Ratio::parse_str("2.0 / 1").unwrap().as_fraction(), 2.0);
        assert_eq!(Ratio::parse_str("2 / 1.0").unwrap().as_fraction(), 2.0);
        assert_eq!(Ratio::parse_str("2.0 / 1.0").unwrap().as_fraction(), 2.0);
        assert_eq!(Ratio::parse_str("2.0").unwrap().as_fraction(), 2.0);
        assert_eq!(Ratio::parse_str("1 / 2").unwrap().as_fraction(), 0.5);
        assert_eq!(Ratio::parse_str("1.0 / 2").unwrap().as_fraction(), 0.5);
        assert_eq!(Ratio::parse_str("1 / 2.0").unwrap().as_fraction(), 0.5);
        assert_eq!(Ratio::parse_str("1.0 / 2.0").unwrap().as_fraction(), 0.5);
        assert_eq!(Ratio::parse_str("0.5").unwrap().as_fraction(), 0.5);
    }

    #[test]
    #[should_panic]
    fn test_bad_ratio() {
        Ratio::parse_str("-1 / 1").unwrap();
        Ratio::parse_str("1 / -1").unwrap();
        Ratio::parse_str("-1 / -1").unwrap();
        Ratio::parse_str("").unwrap();
        Ratio::parse_str("-1").unwrap();
    }

}