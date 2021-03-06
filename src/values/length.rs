// References: https://drafts.csswg.org/css-values/#lengths

use std::{
    cmp::Ordering,
    ops::Mul,
};
use bevy::math::Vec2;
use cssparser::{
    Parser, Token, match_ignore_ascii_case, _cssparser_internal_to_lowercase, CowRcStr
};
use crate::{
    context::CssContext,
    errors::{BevyCssParsingError, BevyCssParsingErrorKind},
    values::{
        AbsoluteLength,
        generic::{MaybeAuto, NonNegative, Numeric},
        number::Number,
        parse::{AllowedValues, Parse},
        percentage::Percentage,
    }
};





/// A length relative to the font base font size of the associated element/node.
/// See also: https://drafts.csswg.org/css-values/#font-relative-lengths
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum FontRelativeLength {
    Em(f32),      // relative to the selected element `font-size`
    Rem(f32),     // relative to the root element `font-size`
    Ex(f32),      // relative to the height of an `x` character for the `font` (& `font-size) in use
                  // https://drafts.csswg.org/css-values/#ex
    Ch(f32),      // relative to the font advance width/height of a 0/zero glyph
                  // https://drafts.csswg.org/css-values/#ch
    // @todo `cap`, `ic`, `lh`, `rlh`
}

impl FontRelativeLength {
    #[inline]
    fn raw_value(&self) -> f32 {
        use FontRelativeLength::*;
        match *self {
            Em(v)  | Rem(v) | Ex(v)  | Ch(v)  => v,
        }
    }

    pub fn to_px(
        &self,
        base_length: f32,
        is_vertical: bool,
        root_base_length: f32,
    ) -> f32 {
        match *self {
            Self::Em(relative_length) => base_length * relative_length,
            Self::Rem(relative_length) => root_base_length * relative_length,
            // @fixme Purely assumed x-height of 0.5
            Self::Ex(relative_length) => base_length * relative_length * 0.5,
            // @fixme Purely assumed character advance of 0.5 for horizontal and 1.0 for vertical text
            Self::Ch(relative_length) => base_length * relative_length * if is_vertical {1.0} else {0.5},
        }
    }

    #[inline]
    pub fn to_computed_px(&self, context: &CssContext) -> f32 {
        self.to_px(
            context.font_size,
            context.vertical_text,
            context.root_font_size
        )
    }
}

impl Numeric for FontRelativeLength {
    #[inline]
    fn zero() -> Self {
        Self::Em(0.0)
    }

    #[inline]
    fn one() -> Self {
        Self::Em(1.0)
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

impl PartialOrd for FontRelativeLength {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        // No ordering between different enum types
        if std::mem::discriminant(self) != std::mem::discriminant(other) {
            return None
        }
        // Because of the discriminant check, we know `self` and `right` are the same enum variant
        match self {
            &Self::Em (left) => left.partial_cmp(&other.raw_value()),
            &Self::Rem(left) => left.partial_cmp(&other.raw_value()),
            &Self::Ex (left) => left.partial_cmp(&other.raw_value()),
            &Self::Ch (left) => left.partial_cmp(&other.raw_value()),
        }
    }
}

impl Mul<f32> for FontRelativeLength {
    type Output = Self;
    #[inline]
    fn mul(self, rhs: f32) -> Self::Output {
        match self {
            Self::Em(v)  => Self::Em(v * rhs),
            Self::Rem(v) => Self::Rem(v * rhs),
            Self::Ex(v)  => Self::Ex(v * rhs),
            Self::Ch(v)  => Self::Ch(v * rhs),
        }
    }
}

/// A length specified as an fixed proportion of the containing viewport height/width.
/// See also: https://drafts.csswg.org/css-values/#viewport-relative-lengths
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ViewportRelativeLength {
    Vw(f32),        // relative to the viewport width
    Vh(f32),        // relative to the viewport height
    Vmin(f32),      // relative to the greater of viewport width/height
    Vmax(f32),      // relative to the lesser of viewport width/height
    // @todo `vi`, `vb`
}

impl ViewportRelativeLength {
    #[inline]
    fn raw_value(&self) -> f32 {
        use ViewportRelativeLength::*;
        match *self {
            Vw(v) | Vh(v) | Vmin(v) | Vmax(v) => v
        }
    }

    pub fn to_px(
        &self,
        viewport_size: &Vec2
    ) -> f32 {
        let (fraction, viewport_length) = match *self {
            Self::Vw  (fraction) => (fraction, viewport_size.x.clone()),
            Self::Vh  (fraction) => (fraction, viewport_size.y.clone()),
            Self::Vmin(fraction) => (fraction, f32::min(viewport_size.x.clone(), viewport_size.y.clone())),
            Self::Vmax(fraction) => (fraction, f32::max(viewport_size.x.clone(), viewport_size.y.clone())),
        };
        // Trunc is to avoid rounding errors for very small view ports
        ((viewport_length as f64) * fraction as f64 / 100.0).trunc() as f32
    }

    #[inline]
    pub fn to_computed_px(&self, context: &CssContext) -> f32 {
        self.to_px(&context.viewport_size)
    }
}

impl Numeric for ViewportRelativeLength {
    #[inline]
    fn zero() -> Self {
        Self::Vw(0.0)
    }

    #[inline]
    fn one() -> Self {
        Self::Vw(1.0)
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

impl PartialOrd for ViewportRelativeLength {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        // No ordering between different enum types
        if std::mem::discriminant(self) != std::mem::discriminant(other) {
            return None
        }
        // Because of the discriminant check, we know `self` and `right` are the same enum variant
        match self {
            Self::Vw  (left) => left.partial_cmp(&other.raw_value()),
            Self::Vh  (left) => left.partial_cmp(&other.raw_value()),
            Self::Vmin(left) => left.partial_cmp(&other.raw_value()),
            Self::Vmax(left) => left.partial_cmp(&other.raw_value()),
        }
    }
}

impl Mul<f32> for ViewportRelativeLength {
    type Output = Self;
    #[inline]
    fn mul(self, rhs: f32) -> Self::Output {
        match self {
            Self::Vw  (v) => Self::Vw  (v * rhs),
            Self::Vh  (v) => Self::Vh  (v * rhs),
            Self::Vmin(v) => Self::Vmin(v * rhs),
            Self::Vmax(v) => Self::Vmax(v * rhs),
        }
    }
}

/// A container for the various specific length types, where the value is not a css `calc(` function
/// See also: https://drafts.csswg.org/css-values/#lengths
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum NoCalcLength {
    Absolute(AbsoluteLength),
    FontRelative(FontRelativeLength),
    ViewportRelative(ViewportRelativeLength),
}

impl NoCalcLength {
    #[inline]
    fn raw_value(&self) -> &dyn Numeric {
        match self {
            Self::Absolute(v) => v,
            Self::FontRelative(v) => v,
            Self::ViewportRelative(v) => v,
        }
    }

    pub fn parse_dimension<'i>(
        unit: &CowRcStr<'i>,
        value: f32
    ) -> Result<Self, BevyCssParsingErrorKind<'i>> {
        Ok(match_ignore_ascii_case! { unit,
            // Absolute
            "px" => Self::Absolute(AbsoluteLength::Px(value)),
            "cm" => Self::Absolute(AbsoluteLength::Cm(value)),
            "mm" => Self::Absolute(AbsoluteLength::Mm(value)),
            "q"  => Self::Absolute(AbsoluteLength::Q(value)),
            "in" => Self::Absolute(AbsoluteLength::In(value)),
            "pc" => Self::Absolute(AbsoluteLength::Pc(value)),
            "pt" => Self::Absolute(AbsoluteLength::Pt(value)),
            // Font Relative
            "em"  => Self::FontRelative(FontRelativeLength::Em(value)),
            "ex"  => Self::FontRelative(FontRelativeLength::Ex(value)),
            "ch"  => Self::FontRelative(FontRelativeLength::Ch(value)),
            "rem" => Self::FontRelative(FontRelativeLength::Rem(value)),
            // Viewport Relative
            "vw"   => Self::ViewportRelative(ViewportRelativeLength::Vw(value)),
            "vh"   => Self::ViewportRelative(ViewportRelativeLength::Vh(value)),
            "vmin" => Self::ViewportRelative(ViewportRelativeLength::Vmin(value)),
            "vmax" => Self::ViewportRelative(ViewportRelativeLength::Vmax(value)),

            _ => return Err(BevyCssParsingErrorKind::UnexpectedDimension(unit.clone()))
        })
    }

    /// It is the caller's responsibility to only pass `Token::Dimension` tokens
    fn from_dim_token<'i>(
        token: &Token<'i>,
        allowed_values: AllowedValues,
    ) -> Result<Self, BevyCssParsingErrorKind<'i>> {
        assert!(matches!(token, Token::Dimension {..}));
        if let Token::Dimension { ref unit, value, .. } = *token {
            if allowed_values.is_ok(value) {
                Self::parse_dimension(unit, value)
            } else {
                Err(BevyCssParsingErrorKind::InvalidValue(
                    allowed_values.into(),
                    Some(token.clone())
                ))
            }
        } else { unreachable!() }
    }

    /// It is the caller's responsibility to only pass `Token::Number` tokens
    fn from_num_token<'i>(
        token: &Token<'i>,
        allowed_values: AllowedValues,
    ) -> Result<Self, BevyCssParsingErrorKind<'i>> {
        let num = Number::from_num_token(token, allowed_values)?;
        // Apart from zero, a bare number (i.e. no dimension) is not allowed here
        if num.is_zero() {
            Ok(Self::zero())
        } else {
            Err(BevyCssParsingErrorKind::MissingDimension(token.clone()))
        }
    }

    #[inline]
    pub fn to_computed_px(&self, context: &CssContext) -> f32 {
        match self {
            Self::Absolute(v) => v.to_computed_px(context),
            Self::FontRelative(v) => v.to_computed_px(context),
            Self::ViewportRelative(v) => v.to_computed_px(context),
        }
    }
}

impl Numeric for NoCalcLength {
    #[inline]
    fn zero() -> Self {
        Self::Absolute(AbsoluteLength::zero())
    }

    #[inline]
    fn one() -> Self {
        Self::Absolute(AbsoluteLength::one())
    }

    #[inline]
    fn is_zero(&self) -> bool {
        self.raw_value().is_zero()
    }

    #[inline]
    fn is_negative(&self) -> bool {
        self.raw_value().is_negative()
    }

    #[inline]
    fn is_infinite(&self) -> bool {
        self.raw_value().is_infinite()
    }
}

impl PartialOrd for NoCalcLength {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        // No ordering between different enum types
        if std::mem::discriminant(self) != std::mem::discriminant(other) {
            return None
        }
        // Because of the discriminant check, we know `self` and `right` are the same enum variant
        match (self, other) {
            (Self::Absolute(this), Self::Absolute(other)) =>
                this.partial_cmp(other),
            (Self::FontRelative(this), Self::FontRelative(other)) =>
                this.partial_cmp(other),
            (Self::ViewportRelative(this), Self::ViewportRelative(other)) =>
                this.partial_cmp(other),
            _ => unreachable!()
        }
    }
}

impl Mul<f32> for NoCalcLength {
    type Output = NoCalcLength;
    #[inline]
    fn mul(self, rhs: f32) -> Self::Output {
        match self {
            Self::Absolute(v)              => Self::Absolute(v * rhs),
            Self::FontRelative(v)        => Self::FontRelative(v * rhs),
            Self::ViewportRelative(v) => Self::ViewportRelative(v * rhs)
        }
    }
}

impl From<f32> for NoCalcLength {
    #[inline]
    fn from(px: f32) -> Self {
        Self::from(AbsoluteLength::Px(px))
    }
}

impl From<AbsoluteLength> for NoCalcLength {
    #[inline]
    fn from(len: AbsoluteLength) -> Self {
        Self::Absolute(len)
    }
}

impl From<FontRelativeLength> for NoCalcLength {
    #[inline]
    fn from(len: FontRelativeLength) -> Self {
        Self::FontRelative(len)
    }
}

impl From<ViewportRelativeLength> for NoCalcLength {
    #[inline]
    fn from(len: ViewportRelativeLength) -> Self {
        Self::ViewportRelative(len)
    }
}

/// A container for any specific length type, including where the value is a css `calc()` function
/// See also: https://drafts.csswg.org/css-values/#lengths
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum Length {
    NoCalc(NoCalcLength),
    //Calc(Box<???>), @todo Add support for css `calc()` functions
}

impl Length {
    // @todo Add support for css `calc()` functions
    // @fixme this is a bit of a hack until `calc()` support is added
    #[inline]
    fn raw_value(&self) -> impl Numeric {
        assert!(matches!(self, Self::NoCalc(_)));
        let Self::NoCalc(value) = *self;
        value
    }

    /// It is the caller's responsibility to only pass `Token::Function` tokens
    pub(super) fn from_func_token<'i>(
        token: &Token<'i>,
        _allowed_values: AllowedValues,
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
            Token::Dimension { .. } =>
                NoCalcLength::from_dim_token(token, allowed_values)
                    .map(Self::NoCalc)
                    .map_err(|err| start.new_custom_error(err)),
            Token::Number { .. } =>
                NoCalcLength::from_num_token(token, allowed_values)
                    .map(Self::NoCalc)
                    .map_err(|err| start.new_custom_error(err)),
            Token::Function { .. } =>
                Self::from_func_token(token, allowed_values)
                    .map_err(|err| start.new_custom_error(err)),
            _ => Err(start.new_unexpected_token_error(token.clone()))
        }
    }
}

impl Numeric for Length {
    #[inline]
    fn zero() -> Self {
        Self::NoCalc(NoCalcLength::zero())
    }

    #[inline]
    fn one() -> Self {
        Self::NoCalc(NoCalcLength::one())
    }

    #[inline]
    fn is_zero(&self) -> bool {
        self.raw_value().is_zero()
    }

    #[inline]
    fn is_negative(&self) -> bool {
        self.raw_value().is_negative()
    }

    #[inline]
    fn is_infinite(&self) -> bool {
        self.raw_value().is_infinite()
    }
}

impl PartialOrd for Length {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        // No ordering between different enum types
        if std::mem::discriminant(self) != std::mem::discriminant(other) {
            return None
        }
        // Because of the discriminant check, we know `self` and `right` are the same enum variant
        // @todo Add support for css `calc()` functions
        match (self, other) {
            (Self::NoCalc(this), Self::NoCalc(other))
            => this.partial_cmp(other),
        }
    }
}

impl Mul<f32> for Length {
    type Output = Length;
    #[inline]
    fn mul(self, rhs: f32) -> Self::Output {
        match self {
            Self::NoCalc(nc_len) => Length::NoCalc(nc_len * rhs),
            // @todo Add support for css `calc()` functions
            //Self::Calc(..) => panic!("Can't multiply calculated length")
        }
    }
}

impl From<NoCalcLength> for Length {
    #[inline]
    fn from(len: NoCalcLength) -> Self {
        Self::NoCalc(len)
    }
}

impl From<f32> for Length {
    #[inline]
    fn from(px: f32) -> Self {
        Self::from(NoCalcLength::from(px))
    }
}

impl From<AbsoluteLength> for Length {
    #[inline]
    fn from(len: AbsoluteLength) -> Self {
        Self::from(NoCalcLength::from(len))
    }
}

impl From<FontRelativeLength> for Length {
    #[inline]
    fn from(len: FontRelativeLength) -> Self {
        Self::from(NoCalcLength::from(len))
    }
}

impl From<ViewportRelativeLength> for Length {
    #[inline]
    fn from(len: ViewportRelativeLength) -> Self {
        Self::from(NoCalcLength::from(len))
    }
}

impl Parse for Length {
    #[inline]
    fn parse<'i, 't>(input: &mut Parser<'i, 't>) -> Result<Self, BevyCssParsingError<'i>> {
        Self::parse_internal(input, AllowedValues::All)
    }
}

/// A container for any specific length (inc. css `calc()`) where percentage (`%`) can be used too
/// See also: https://drafts.csswg.org/css-values-4/#typedef-length-percentage
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum LengthPercentage {
    Length(NoCalcLength),
    Percentage(Percentage),
    //Calc(Box<???>), @todo Add support for css `calc()` functions
}

impl LengthPercentage {
    #[inline]
    fn raw_value(&self) -> &dyn Numeric {
        match self {
            Self::Length(len) => len,
            Self::Percentage(pc) => pc,
        }
    }

    /// It is the caller's responsibility to only pass `Token::Function` tokens
    pub(super) fn from_func_token<'i>(
        token: &Token<'i>,
        _allowed_values: AllowedValues,
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
            Token::Dimension { .. } =>
                NoCalcLength::from_dim_token(token, allowed_values)
                    .map(Self::Length)
                    .map_err(|err| start.new_custom_error(err)),
            Token::Percentage { .. } =>
                Percentage::from_pc_token(token, allowed_values)
                    .map(Self::Percentage)
                    .map_err(|err| start.new_custom_error(err)),
            Token::Number { .. } =>
                NoCalcLength::from_num_token(token, allowed_values)
                    .map(Self::Length)
                    .map_err(|err| start.new_custom_error(err)),
            Token::Function { .. } =>
                Self::from_func_token(token, allowed_values)
                    .map_err(|err| start.new_custom_error(err)),
            _ => Err(start.new_unexpected_token_error(token.clone()))
        }
    }
}

impl Numeric for LengthPercentage {
    #[inline]
    fn zero() -> Self {
        Self::Length(NoCalcLength::zero())
    }

    #[inline]
    fn one() -> Self {
        Self::Length(NoCalcLength::one())
    }

    #[inline]
    fn is_zero(&self) -> bool {
        self.raw_value().is_zero()
    }

    #[inline]
    fn is_negative(&self) -> bool {
        self.raw_value().is_negative()
    }

    #[inline]
    fn is_infinite(&self) -> bool {
        self.raw_value().is_infinite()
    }
}

impl PartialOrd for LengthPercentage {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        // No ordering between different enum types
        if std::mem::discriminant(self) != std::mem::discriminant(other) {
            return None
        }
        // Because of the discriminant check, we know `self` and `right` are the same enum variant
        // @todo Add support for css `calc()` functions
        match (self, other) {
            (Self::Length(this), Self::Length(other)) =>
                this.partial_cmp(other),
            (Self::Percentage(this), Self::Percentage(other)) =>
                this.partial_cmp(other),
            _ => unreachable!()
        }
    }
}

impl Mul<f32> for LengthPercentage {
    type Output = LengthPercentage;
    #[inline]
    fn mul(self, rhs: f32) -> Self::Output {
        match self {
            Self::Length(len) => LengthPercentage::Length(len * rhs),
            Self::Percentage(pc) => LengthPercentage::Percentage(pc * rhs),
            // @todo Add support for css `calc()` functions
            //Self::Calc(..) => panic!("Can't multiply calculated length")
        }
    }
}

impl From<Length> for LengthPercentage {
    #[inline]
    fn from(length: Length) -> Self {
        match length {
            Length::NoCalc(len) => Self::Length(len),
            // @todo Add support for css `calc()` functions
            //Length::Calc(calc) => Self::Calc(calc),
        }
    }
}

impl From<NoCalcLength> for LengthPercentage {
    #[inline]
    fn from(length: NoCalcLength) -> Self {
        Self::Length(length)
    }
}

impl From<Percentage> for LengthPercentage {
    #[inline]
    fn from(pc: Percentage) -> Self {
        Self::Percentage(pc)
    }
}

impl From<f32> for LengthPercentage {
    #[inline]
    fn from(px: f32) -> Self {
        Self::from(NoCalcLength::from(px))
    }
}

impl From<AbsoluteLength> for LengthPercentage {
    #[inline]
    fn from(len: AbsoluteLength) -> Self {
        Self::from(NoCalcLength::from(len))
    }
}

impl From<FontRelativeLength> for LengthPercentage {
    #[inline]
    fn from(len: FontRelativeLength) -> Self {
        Self::from(NoCalcLength::from(len))
    }
}

impl From<ViewportRelativeLength> for LengthPercentage {
    #[inline]
    fn from(len: ViewportRelativeLength) -> Self {
        Self::from(NoCalcLength::from(len))
    }
}

impl Parse for LengthPercentage {
    #[inline]
    fn parse<'i, 't>(input: &mut Parser<'i, 't>) -> Result<Self, BevyCssParsingError<'i>> {
        Self::parse_internal(input, AllowedValues::All)
    }
}

/// A wrapper around `Length` that disallows negative values (i.e. < 0.0)
pub type NonNegativeLength = NonNegative<Length>;

impl Parse for NonNegativeLength {
    #[inline]
    fn parse<'i, 't>(input: &mut Parser<'i, 't>) -> Result<Self, BevyCssParsingError<'i>> {
        Ok(Self(
            Length::parse_internal(input, AllowedValues::NonNegative)?
        ))
    }
}

/// A wrapper around `LengthPercentage` that disallows negative values (i.e. < 0.0)
pub type NonNegativeLengthPercentage = NonNegative<LengthPercentage>;

impl Parse for NonNegativeLengthPercentage {
    #[inline]
    fn parse<'i, 't>(input: &mut Parser<'i, 't>) -> Result<Self, BevyCssParsingError<'i>> {
        Ok(Self(
            LengthPercentage::parse_internal(input, AllowedValues::NonNegative)?
        ))
    }
}

//// A wrapper around `Length` that allows the use of `auto`
//pub type LengthOrAuto = MaybeAuto<Length>;

/// A wrapper around `LengthPercentage` that allows the use of `auto`
pub type LengthPercentageOrAuto = MaybeAuto<LengthPercentage>;

//// A wrapper around `NonNegativeLength` that allows the use of `auto`
//pub type NonNegativeLengthOrAuto = MaybeAuto<NonNegativeLength>;

//// A wrapper around `NonNegativeLengthPercentage` that allows the use of `auto`
//pub type NonNegativeLengthPercentageOrAuto = MaybeAuto<NonNegativeLengthPercentage>;