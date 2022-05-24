use std::cmp::Ordering;
use std::ops::Mul;
use crate::CssContext;
use crate::values::generic::Numeric;

// Servo uses `60` app units per pixel (why???).  Servo also has a whole `Au` type that isn't used here.
// 60 `au` is used here on the basis that 'if it's good enough for Mozilla, it's fine for me'.
pub const AU_PER_PX: f32 = 60.0;
pub const AU_PER_IN: f32 = AU_PER_PX * 96.0;   // Servo assumes 96 dpi, and bases everything else off this
pub const AU_PER_PC: f32 = AU_PER_IN / 6.0;
pub const AU_PER_PT: f32 = AU_PER_IN / 72.0;
pub const AU_PER_CM: f32 = AU_PER_PX / 2.54;
pub const AU_PER_MM: f32 = AU_PER_CM / 10.0;
pub const AU_PER_Q : f32 = AU_PER_MM / 4.0;

/// An absolute length in the given units. The conversion of in/mm/etc to px assumes 96 dpi
/// See also: https://drafts.csswg.org/css-values/#absolute-length
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum AbsoluteLength {
    Px(f32),                // pixels
    Mm(f32),                // millimeters
    Cm(f32),                // centimeters
    Q (f32),                // quarter-millimeters (1/4 mm)
    In(f32),                // inches
    Pc(f32),                // pica (1/6 in)
    Pt(f32),                // points (1/72 in)
}

impl AbsoluteLength {
    #[inline]
    fn raw_value(&self) -> f32 {
        use AbsoluteLength::*;
        match *self {
            Px(v) | Mm(v) | Cm(v) | Q(v) | In(v) | Pc(v) | Pt(v)
            => v
        }
    }

    #[inline]
    pub fn to_computed_value(&self) -> f32 {
        self.to_px()
    }

    #[inline]
    pub fn to_px(&self) -> f32 {
        let pixels = match *self {
            Self::Px(v) => v,
            Self::Mm(v) => v * (AU_PER_MM / AU_PER_PX),
            Self::Cm(v) => v * (AU_PER_CM / AU_PER_PX),
            Self::Q(v)  => v * (AU_PER_Q  / AU_PER_PX),
            Self::In(v) => v * (AU_PER_IN / AU_PER_PX),
            Self::Pc(v) => v * (AU_PER_PC / AU_PER_PX),
            Self::Pt(v) => v * (AU_PER_PT / AU_PER_PX),
        };
        pixels.min(f32::MAX).max(f32::MIN)
    }

    #[inline]
    pub fn to_computed_px(&self, _context: &CssContext) -> f32 {
        self.to_px()
    }
}

impl Numeric for AbsoluteLength {
    #[inline]
    fn zero() -> Self {
        Self::Px(0.0)
    }

    #[inline]
    fn one() -> Self {
        Self::Px(1.0)
    }

    #[inline]
    fn is_zero(&self) -> bool {
        self.raw_value() == 0.0
    }

    #[inline]
    fn is_negative(&self) -> bool {
        self.raw_value() < 0.0
    }

    #[inline]
    fn is_infinite(&self) -> bool {
        self.raw_value().is_infinite()
    }
}

impl PartialOrd for AbsoluteLength {
    #[inline]
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        self.to_px().partial_cmp(&other.to_px())
    }
}

impl Mul<f32> for AbsoluteLength {
    type Output = Self;
    #[inline]
    fn mul(self, rhs: f32) -> Self::Output {
        match self {
            Self::Px(v) => Self::Px(v * rhs),
            Self::Mm(v) => Self::Mm(v * rhs),
            Self::Cm(v) => Self::Cm(v * rhs),
            Self::Q(v)  => Self::Q (v * rhs),
            Self::In(v) => Self::In(v * rhs),
            Self::Pc(v) => Self::Pc(v * rhs),
            Self::Pt(v) => Self::Pt(v * rhs),
        }
    }
}

impl From<f32> for AbsoluteLength {
    fn from(px: f32) -> Self {
        Self::Px(px)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_raw_value() {
        assert_eq!(AbsoluteLength::Px(1.0).raw_value(), 1.0);
        assert_eq!(AbsoluteLength::Mm(2.0).raw_value(), 2.0);
        assert_eq!(AbsoluteLength::Cm(3.0).raw_value(), 3.0);
        assert_eq!(AbsoluteLength::Q(4.0).raw_value(), 4.0);
        assert_eq!(AbsoluteLength::In(5.0).raw_value(), 5.0);
        assert_eq!(AbsoluteLength::Pc(6.0).raw_value(), 6.0);
        assert_eq!(AbsoluteLength::Pt(7.0).raw_value(), 7.0);
    }

    #[test]
    fn test_to_px() {
        assert_eq!(AbsoluteLength::Px(1.0).to_px(), 1.0);
        assert_eq!(AbsoluteLength::Mm(254.0).to_px(), 10.0);
        assert_eq!(AbsoluteLength::Cm(127.0).to_px(), 50.0);
        assert_eq!(AbsoluteLength::Q(508.0).to_px(), 5.0);
        assert_eq!(AbsoluteLength::In(5.0).to_px(), 480.0);
        assert_eq!(AbsoluteLength::Pc(6.0).to_px(), 96.0);
        assert_eq!(AbsoluteLength::Pt(9.0).to_px(), 12.0);
    }

    #[test]
    fn test_numeric() {
        assert_eq!(AbsoluteLength::zero(), AbsoluteLength::Px(0.0));
        assert_eq!(AbsoluteLength::one(), AbsoluteLength::Px(1.0));
        assert!(AbsoluteLength::zero().is_zero());
        assert!(!AbsoluteLength::one().is_zero());
        assert!(!AbsoluteLength::Px(-1.0).is_zero());
        assert!(!AbsoluteLength::zero().is_negative());
        assert!(!AbsoluteLength::one().is_negative());
        assert!(AbsoluteLength::Px(-1.0).is_negative());
        //assert!(AbsoluteLength::Px(f32::infinity()).is_infinite());
        assert!(!AbsoluteLength::zero().is_infinite());
        assert!(!AbsoluteLength::one().is_infinite());
    }

    #[test]
    fn test_partial_ord() {
        let zero = AbsoluteLength::zero();
        let one = AbsoluteLength::one();
        assert!(zero.partial_cmp(&zero).unwrap().is_eq());
        assert!(zero.partial_cmp(&one).unwrap().is_lt());
        assert!(one.partial_cmp(&zero).unwrap().is_gt());
    }

}