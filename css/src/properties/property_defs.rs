use bevy::{
    render::color,
    ui,
};
use cssparser::Parser;
use crate::{
    errors::BevyCssParsingError,
    properties::BevyPropertyDeclaration,
    values::*,
};

pub trait Property {
    type ValueType: Parse;

    fn value_to_declaration(value: Self::ValueType) -> BevyPropertyDeclaration;

    fn parse_css<'i, 't>(
        input: &mut Parser<'i, 't>
    ) -> Result<Self::ValueType, BevyCssParsingError<'i>> {
        <Self::ValueType as Parse>::parse(input)
    }

    fn parse_declaration<'i, 't>(
        input: &mut Parser<'i, 't>
    ) -> Result<BevyPropertyDeclaration, BevyCssParsingError<'i>> {
        Self::parse_css(input).map(Self::value_to_declaration)
    }
}

macro_rules! property_def {
    ( $property:ident, $value_type:ty, $converter_func:expr ) => {
        pub struct $property;
        impl Property for $property {
            type ValueType = $value_type;
            fn value_to_declaration(value: Self::ValueType) -> BevyPropertyDeclaration {
                $converter_func(value)
            }
        }
    }
}

// Display
property_def!(Display, ui::Display, BevyPropertyDeclaration::Display);
property_def!(Direction, ui::Direction, BevyPropertyDeclaration::Direction);
property_def!(Width, LengthPercentageOrAuto, BevyPropertyDeclaration::Width);
property_def!(Height, LengthPercentageOrAuto, BevyPropertyDeclaration::Height);
property_def!(MinWidth, LengthPercentageOrAuto, BevyPropertyDeclaration::MinWidth);
property_def!(MinHeight, LengthPercentageOrAuto, BevyPropertyDeclaration::MinHeight);
property_def!(MaxWidth, LengthPercentageOrAuto, BevyPropertyDeclaration::MaxWidth);
property_def!(MaxHeight, LengthPercentageOrAuto, BevyPropertyDeclaration::MaxHeight);
property_def!(Overflow, ui::Overflow, BevyPropertyDeclaration::Overflow);

// Position
property_def!(Position, ui::PositionType, BevyPropertyDeclaration::Position);
property_def!(Top, LengthPercentageOrAuto, BevyPropertyDeclaration::Top);
property_def!(Right, LengthPercentageOrAuto, BevyPropertyDeclaration::Right);
property_def!(Bottom, LengthPercentageOrAuto, BevyPropertyDeclaration::Bottom);
property_def!(Left, LengthPercentageOrAuto, BevyPropertyDeclaration::Left);

// Flex Box
property_def!(FlexDirection, ui::FlexDirection, BevyPropertyDeclaration::FlexDirection);
property_def!(FlexWrap, ui::FlexWrap, BevyPropertyDeclaration::FlexWrap);
property_def!(FlexGrow, NonNegativeNumber, BevyPropertyDeclaration::FlexGrow);
property_def!(FlexShrink, NonNegativeNumber, BevyPropertyDeclaration::FlexShrink);
property_def!(FlexBasis, LengthPercentageOrAuto, BevyPropertyDeclaration::FlexBasis);
property_def!(AspectRatio, RatioOrAuto, BevyPropertyDeclaration::AspectRatio);

// Alignment
property_def!(AlignItems, ui::AlignItems, BevyPropertyDeclaration::AlignItems);
property_def!(AlignSelf, ui::AlignSelf, BevyPropertyDeclaration::AlignSelf);
property_def!(AlignContent, ui::AlignContent, BevyPropertyDeclaration::AlignContent);
property_def!(JustifyContent, ui::JustifyContent, BevyPropertyDeclaration::JustifyContent);

// Margin
property_def!(Margin, SidedValue<LengthPercentageOrAuto>, BevyPropertyDeclaration::Margin);
property_def!(MarginTop, LengthPercentageOrAuto, BevyPropertyDeclaration::Top);
property_def!(MarginRight, LengthPercentageOrAuto, BevyPropertyDeclaration::Right);
property_def!(MarginBottom, LengthPercentageOrAuto, BevyPropertyDeclaration::Bottom);
property_def!(MarginLeft, LengthPercentageOrAuto, BevyPropertyDeclaration::Left);

// Padding
property_def!(Padding, SidedValue<LengthPercentageOrAuto>, BevyPropertyDeclaration::Padding);
property_def!(PaddingTop, LengthPercentageOrAuto, BevyPropertyDeclaration::PaddingTop);
property_def!(PaddingRight, LengthPercentageOrAuto, BevyPropertyDeclaration::PaddingRight);
property_def!(PaddingBottom, LengthPercentageOrAuto, BevyPropertyDeclaration::PaddingBottom);
property_def!(PaddingLeft, LengthPercentageOrAuto, BevyPropertyDeclaration::PaddingLeft);

// Borders
property_def!(BorderWidth, SidedValue<LengthPercentageOrAuto>, BevyPropertyDeclaration::BorderWidth);
property_def!(BorderWidthTop, LengthPercentageOrAuto, BevyPropertyDeclaration::BorderWidthTop);
property_def!(BorderWidthRight, LengthPercentageOrAuto, BevyPropertyDeclaration::BorderWidthRight);
property_def!(BorderWidthBottom, LengthPercentageOrAuto, BevyPropertyDeclaration::BorderWidthBottom);
property_def!(BorderWidthLeft, LengthPercentageOrAuto, BevyPropertyDeclaration::BorderWidthLeft);

// Color
property_def!(Color, color::Color, BevyPropertyDeclaration::Color);