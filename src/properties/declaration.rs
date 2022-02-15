use bevy::{
    prelude::Color,
    ui,
};
use cssparser::{CowRcStr, Parser};
use crate::{
    context::CssContext,
    errors::{BevyCssParsingError, BevyCssParsingErrorKind},
    properties::{self, Property},
    values::{
        bevy_converters::ContextualInto,
        LengthPercentageOrAuto, NonNegativeNumber, RatioOrAuto, SidedValue
    },
};

/// Corresponds to `bevy::ui::Style`
#[derive(Debug, Clone)]
pub enum BevyPropertyDeclaration {
    // Display
    Display(ui::Display),
    Direction(ui::Direction),
    Width(LengthPercentageOrAuto),
    Height(LengthPercentageOrAuto),
    MinWidth(LengthPercentageOrAuto),
    MinHeight(LengthPercentageOrAuto),
    MaxWidth(LengthPercentageOrAuto),
    MaxHeight(LengthPercentageOrAuto),
    Overflow(ui::Overflow),

    // Position
    Position(ui::PositionType),
    Top(LengthPercentageOrAuto),
    Right(LengthPercentageOrAuto),
    Bottom(LengthPercentageOrAuto),
    Left(LengthPercentageOrAuto),

    // Flex Box
    FlexDirection(ui::FlexDirection),
    FlexWrap(ui::FlexWrap),
    FlexGrow(NonNegativeNumber),
    FlexShrink(NonNegativeNumber),
    FlexBasis(LengthPercentageOrAuto),
    AspectRatio(RatioOrAuto),

    // Alignment
    AlignItems(ui::AlignItems),
    AlignSelf(ui::AlignSelf),
    AlignContent(ui::AlignContent),
    JustifyContent(ui::JustifyContent),

    // Margins
    Margin(SidedValue<LengthPercentageOrAuto>),
    MarginTop(LengthPercentageOrAuto),
    MarginRight(LengthPercentageOrAuto),
    MarginBottom(LengthPercentageOrAuto),
    MarginLeft(LengthPercentageOrAuto),

    // Padding
    Padding(SidedValue<LengthPercentageOrAuto>),
    PaddingTop(LengthPercentageOrAuto),
    PaddingRight(LengthPercentageOrAuto),
    PaddingBottom(LengthPercentageOrAuto),
    PaddingLeft(LengthPercentageOrAuto),

    // Borders
    BorderWidth(SidedValue<LengthPercentageOrAuto>),
    BorderWidthTop(LengthPercentageOrAuto),
    BorderWidthRight(LengthPercentageOrAuto),
    BorderWidthBottom(LengthPercentageOrAuto),
    BorderWidthLeft(LengthPercentageOrAuto),

    // Color
    Color(Color)
}

// Convenience type
type ParsingFunc =
    for<'i, 'a> fn(
    input: &mut Parser<'i, 'a>
) -> Result<BevyPropertyDeclaration, BevyCssParsingError<'i>>;

impl BevyPropertyDeclaration {
    pub(crate) fn modify_style(&self, context: &CssContext, style: &mut ui::Style) {
        match *self {
            // Display
            Self::Display(display) => style.display = display,
            Self::Direction(direction) => style.direction = direction,
            Self::Width(width) => style.size.width = width.contextual_into(context),
            Self::Height(height) => style.size.height = height.contextual_into(context),
            Self::MinWidth(min_width) => style.min_size.width = min_width.contextual_into(context),
            Self::MinHeight(min_height) => style.min_size.height = min_height.contextual_into(context),
            Self::MaxWidth(max_width) => style.max_size.width = max_width.contextual_into(context),
            Self::MaxHeight(max_height) => style.max_size.height = max_height.contextual_into(context),
            Self::Overflow(overflow) => style.overflow = overflow,

            // Position
            Self::Position(position_type) => style.position_type = position_type,
            Self::Top(top) => style.position.top = top.contextual_into(context),
            Self::Right(right) => style.position.right = right.contextual_into(context),
            Self::Bottom(bottom) => style.position.bottom = bottom.contextual_into(context),
            Self::Left(left) => style.position.left = left.contextual_into(context),

            // Flex Box
            Self::FlexDirection(flex_direction) => style.flex_direction = flex_direction,
            Self::FlexWrap(flex_wrap) => style.flex_wrap = flex_wrap,
            Self::FlexGrow(flex_grow) => style.flex_grow = flex_grow.into(),
            Self::FlexShrink(flex_shrink) => style.flex_shrink = flex_shrink.into(),
            Self::FlexBasis(flex_basis) => style.flex_basis = flex_basis.contextual_into(context),
            Self::AspectRatio(aspect_ratio) => style.aspect_ratio = aspect_ratio.non_auto().map(|r| r.as_fraction()),

            // Alignment
            Self::AlignItems(align_items) => style.align_items = align_items,
            Self::AlignSelf(align_self) => style.align_self = align_self,
            Self::AlignContent(align_content) => style.align_content = align_content,
            Self::JustifyContent(justify_content) => style.justify_content = justify_content,

            // Margins
            Self::Margin(margin) => style.margin = margin.contextual_into(context),
            Self::MarginTop(margin_top) => style.margin.top = margin_top.contextual_into(context),
            Self::MarginRight(margin_right) => style.margin.right = margin_right.contextual_into(context),
            Self::MarginBottom(margin_bottom) => style.margin.bottom = margin_bottom.contextual_into(context),
            Self::MarginLeft(margin_left) => style.margin.left = margin_left.contextual_into(context),

            // Padding
            Self::Padding(padding) => style.padding = padding.contextual_into(context),
            Self::PaddingTop(padding_top) => style.padding.top = padding_top.contextual_into(context),
            Self::PaddingRight(padding_right) => style.padding.right = padding_right.contextual_into(context),
            Self::PaddingBottom(padding_bottom) => style.padding.bottom = padding_bottom.contextual_into(context),
            Self::PaddingLeft(padding_left) => style.padding.left = padding_left.contextual_into(context),

            // Borders
            Self::BorderWidth(border_width) => style.border = border_width.contextual_into(context),
            Self::BorderWidthTop(border_width_top) => style.border.top = border_width_top.contextual_into(context),
            Self::BorderWidthRight(border_width_right) => style.border.right = border_width_right.contextual_into(context),
            Self::BorderWidthBottom(border_width_bottom) => style.border.bottom = border_width_bottom.contextual_into(context),
            Self::BorderWidthLeft(border_width_left) => style.border.left = border_width_left.contextual_into(context),

            _ => (),
        }
    }

    pub(crate) fn modify_color(&self, ui_color: &mut ui::UiColor) {
        // Color
        match *self {
            Self::Color(color) => ui_color.0 = color,

            _ => (),
        }
    }

    fn parsing_func_from_name(name: &CowRcStr) -> Option<ParsingFunc> {
        Some(match name.to_ascii_lowercase().as_str() {
            // Display
            "display"           => properties::Display::parse_declaration,
            "direction"         => properties::Direction::parse_declaration,
            "width"             => properties::Width::parse_declaration,
            "height"            => properties::Height::parse_declaration,
            "min-width"         => properties::MinWidth::parse_declaration,
            "min-height"        => properties::MinHeight::parse_declaration,
            "max-width"         => properties::MaxWidth::parse_declaration,
            "max-height"        => properties::MaxHeight::parse_declaration,
            "overflow"          => properties::Overflow::parse_declaration,

            // Position
            "position"          => properties::Position::parse_declaration,
            "top"               => properties::Top::parse_declaration,
            "right"             => properties::Right::parse_declaration,
            "bottom"            => properties::Bottom::parse_declaration,
            "left"              => properties::Left::parse_declaration,

            // Flex Box
            "flex-direction"    => properties::FlexDirection::parse_declaration,
            "flex-wrap"         => properties::FlexWrap::parse_declaration,
            "flex-grow"         => properties::FlexGrow::parse_declaration,
            "flex-shrink"       => properties::FlexShrink::parse_declaration,
            "flex-basis"        => properties::FlexBasis::parse_declaration,
            "aspect-ratio"      => properties::AspectRatio::parse_declaration,

            // Alignment
            "align-items"       => properties::AlignItems::parse_declaration,
            "align-self"        => properties::AlignSelf::parse_declaration,
            "align-content"     => properties::AlignContent::parse_declaration,
            "justify-content"   => properties::JustifyContent::parse_declaration,

            // Margins
            "margin"            => properties::Margin::parse_declaration,
            "margin-top"        => properties::MarginTop::parse_declaration,
            "margin-right"      => properties::MarginRight::parse_declaration,
            "margin-bottom"     => properties::MarginBottom::parse_declaration,
            "margin-left"       => properties::MarginLeft::parse_declaration,

            // Padding
            "padding"           => properties::Padding::parse_declaration,
            "padding-top"       => properties::PaddingTop::parse_declaration,
            "padding-right"     => properties::PaddingRight::parse_declaration,
            "padding-bottom"    => properties::PaddingBottom::parse_declaration,
            "padding-left"      => properties::PaddingLeft::parse_declaration,

            // Borders
            "border-width"            => properties::BorderWidth::parse_declaration,
            "border-width-top"        => properties::BorderWidthTop::parse_declaration,
            "border-width-right"      => properties::BorderWidthRight::parse_declaration,
            "border-width-bottom"     => properties::BorderWidthBottom::parse_declaration,
            "border-width-left"       => properties::BorderWidthLeft::parse_declaration,

            // Color
            "color"             => properties::Color::parse_declaration,

            _ => return None
        })
    }

    pub fn parse_input<'i, 't>(
        property_name: CowRcStr<'i>,
        input: &mut Parser<'i, 't>
    ) -> Result<Self, BevyCssParsingError<'i>> {
        match Self::parsing_func_from_name(&property_name) {
            Some(property_parsing_func) => property_parsing_func(input),
            None => Err(
                input.new_custom_error(BevyCssParsingErrorKind::UnknownProperty(property_name.to_owned()))
            )
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn testtest() {
        assert_eq!(1+1, 3);
    }
}