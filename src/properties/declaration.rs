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
#[derive(Debug, Clone, PartialEq)]
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
    use bevy::ui;
    use cssparser::{ParseErrorKind, Parser, ParserInput};
    use crate::values::{
        generic::{NonNegative, Numeric},
        length::{AbsoluteLength, NoCalcLength},
        LengthPercentage,
        Number,
        percentage::Percentage,
        Ratio
    };

    //  Helpers //

    fn parse_property_value(property: &str, value: &str) -> BevyPropertyDeclaration {
        let mut parser_input = ParserInput::new(value);
        let mut input = Parser::new(&mut parser_input);
        match BevyPropertyDeclaration::parse_input(property.into(), &mut input) {
            Ok(declaration) => declaration,
            Err(error) => match error.kind {
                ParseErrorKind::Custom(e) => panic!("{:?}", e),
                ParseErrorKind::Basic(e) => panic!("{:?}", e)
            }
        }
    }

    fn parse_all_property_values<Value: Clone>(
        property: &str,
        variant: impl Fn(Value) -> BevyPropertyDeclaration,
        values_to_test: Vec<(&str, Value)>
    ) {
        for (val_string, value) in values_to_test.iter() {
            assert_eq!(
                parse_property_value(property, val_string),
                variant(value.clone())
            )
        }
    }

    fn length_percentage_auto_10px() -> LengthPercentageOrAuto {
        LengthPercentageOrAuto::NotAuto(
            LengthPercentage::Length(NoCalcLength::Absolute(AbsoluteLength::Px(10.0)))
        )
    }

    fn length_percentage_auto_10pc() -> LengthPercentageOrAuto {
        LengthPercentageOrAuto::NotAuto(
            LengthPercentage::Percentage(Percentage::new(0.1))
        )
    }

    fn auto_length_percentage_vec<'a>() -> Vec<(&'a str, LengthPercentageOrAuto)> {
        vec![
            ("auto", LengthPercentageOrAuto::Auto),
            ("0", LengthPercentageOrAuto::zero()),
            ("10px", length_percentage_auto_10px()),
            ("10%", length_percentage_auto_10pc()),
        ]
    }

    fn non_neg_number_vec<'a>() -> Vec<(&'a str, NonNegativeNumber)> {
        vec![
            ("0", NonNegativeNumber::zero()),
            ("1", NonNegativeNumber::one()),
            ("10", NonNegative(Number(10.0))),
        ]
    }

    fn sided_value_vec<'a>() -> Vec<(&'a str, SidedValue<LengthPercentageOrAuto>)> {
        vec![
            ("10px",
                SidedValue::<LengthPercentageOrAuto>::new_1(
                    length_percentage_auto_10px()
                )
            ),
            ("10% 10px",
                SidedValue::<LengthPercentageOrAuto>::new_2(
                    length_percentage_auto_10pc(),
                    length_percentage_auto_10px()
                )
            ),
            ("10px auto 10%",
                SidedValue::<LengthPercentageOrAuto>::new_3(
                    length_percentage_auto_10px(),
                    LengthPercentageOrAuto::Auto,
                    length_percentage_auto_10pc()
                )
            ),
            ("10px 10% auto 0",
             SidedValue::<LengthPercentageOrAuto>::new_4(
                 length_percentage_auto_10px(),
                 length_percentage_auto_10pc(),
                 LengthPercentageOrAuto::Auto,
                 LengthPercentageOrAuto::zero()
             )
            ),
        ]
    }

    // Generic Tests //

    #[test]
    #[should_panic]
    fn bad_value_test() {
        parse_property_value("display", "bad string");
    }

    #[test]
    #[should_panic]
    fn bad_property_test() {
        parse_property_value("bad_string", "auto");
    }

    #[test]
    fn test_display() {
        parse_all_property_values(
            "display",
            BevyPropertyDeclaration::Display,
            vec![
                ("flex", ui::Display::Flex),
                ("none", ui::Display::None),
            ]
        );
    }

    // Display //

    #[test]
    fn test_direction() {
        parse_all_property_values(
            "direction",
            BevyPropertyDeclaration::Direction,
            vec![
                ("ltr", ui::Direction::LeftToRight),
                ("rtl", ui::Direction::RightToLeft),
                ("inherit", ui::Direction::Inherit),
            ]
        );
    }

    #[test]
    fn test_width() {
        parse_all_property_values(
            "width",
            BevyPropertyDeclaration::Width,
            auto_length_percentage_vec()
        );
    }

    #[test]
    fn test_min_width() {
        parse_all_property_values(
            "min-width",
            BevyPropertyDeclaration::MinWidth,
            auto_length_percentage_vec()
        );
    }

    #[test]
    fn test_max_width() {
        parse_all_property_values(
            "max-width",
            BevyPropertyDeclaration::MaxWidth,
            auto_length_percentage_vec()
        );
    }

    #[test]
    fn test_height() {
        parse_all_property_values(
            "height",
            BevyPropertyDeclaration::Height,
            auto_length_percentage_vec()
        );
    }

    #[test]
    fn test_min_height() {
        parse_all_property_values(
            "min-height",
            BevyPropertyDeclaration::MinHeight,
            auto_length_percentage_vec()
        );
    }

    #[test]
    fn test_max_height() {
        parse_all_property_values(
            "max-height",
            BevyPropertyDeclaration::MaxHeight,
            auto_length_percentage_vec()
        );
    }

    #[test]
    fn test_overflow() {
        parse_all_property_values(
            "overflow",
            BevyPropertyDeclaration::Overflow,
            vec![
                ("visible", ui::Overflow::Visible),
                ("hidden", ui::Overflow::Hidden),
            ]
        );
    }

    // Position //

    #[test]
    fn test_position() {
        parse_all_property_values(
            "position",
            BevyPropertyDeclaration::Position,
            vec![
                ("relative", ui::PositionType::Relative),
                ("absolute", ui::PositionType::Absolute),
            ]
        );
    }

    #[test]
    fn test_top() {
        parse_all_property_values(
            "top",
            BevyPropertyDeclaration::Top,
            auto_length_percentage_vec()
        );
    }

    #[test]
    fn test_right() {
        parse_all_property_values(
            "right",
            BevyPropertyDeclaration::Right,
            auto_length_percentage_vec()
        );
    }

    #[test]
    fn test_bottom() {
        parse_all_property_values(
            "bottom",
            BevyPropertyDeclaration::Bottom,
            auto_length_percentage_vec()
        );
    }

    #[test]
    fn test_left() {
        parse_all_property_values(
            "left",
            BevyPropertyDeclaration::Left,
            auto_length_percentage_vec()
        );
    }

    // Flex Box //

    #[test]
    fn test_flex_direction() {
        parse_all_property_values(
            "flex-direction",
            BevyPropertyDeclaration::FlexDirection,
            vec![
                ("row", ui::FlexDirection::Row),
                ("row-reverse", ui::FlexDirection::RowReverse),
                ("column", ui::FlexDirection::Column),
                ("column-reverse", ui::FlexDirection::ColumnReverse),
            ]
        );
    }

    #[test]
    fn test_flex_wrap() {
        parse_all_property_values(
            "flex-wrap",
            BevyPropertyDeclaration::FlexWrap,
            vec![
                ("nowrap", ui::FlexWrap::NoWrap),
                ("wrap", ui::FlexWrap::Wrap),
                ("wrap-reverse", ui::FlexWrap::WrapReverse),
            ]
        );
    }

    #[test]
    fn test_flex_grow() {
        parse_all_property_values(
            "flex-grow",
            BevyPropertyDeclaration::FlexGrow,
            non_neg_number_vec()
        );
    }

    #[test]
    #[should_panic]
    fn test_flex_grow_negative() {
        parse_property_value("flex-grow", "-1");
    }

    #[test]
    fn test_flex_shrink() {
        parse_all_property_values(
            "flex-shrink",
            BevyPropertyDeclaration::FlexShrink,
            non_neg_number_vec()
        );
    }

    #[test]
    #[should_panic]
    fn test_flex_shrink_negative() {
        parse_property_value("flex-shrink", "-1");
    }

    #[test]
    fn test_flex_basis() {
        parse_all_property_values(
            "flex-basis",
            BevyPropertyDeclaration::FlexBasis,
            auto_length_percentage_vec()
        );
    }

    #[test]
    fn test_aspect_ratio() {
        parse_all_property_values(
            "aspect-ratio",
            BevyPropertyDeclaration::AspectRatio,
            vec![
                ("auto", RatioOrAuto::Auto),
                ("1 / 1", RatioOrAuto::NotAuto(Ratio(
                    NonNegative(Number(1.0)),
                    NonNegative(Number(1.0))
                ))),
                ("0 / 1", RatioOrAuto::NotAuto(Ratio(
                    NonNegative(Number(0.0)),
                    NonNegative(Number(1.0))
                ))),
                ("1 / 0", RatioOrAuto::NotAuto(Ratio(
                    NonNegative(Number(1.0)),
                    NonNegative(Number(0.0))
                ))),
                ("0.5", RatioOrAuto::NotAuto(Ratio(
                    NonNegative(Number(0.5)),
                    NonNegative(Number(1.0))
                ))),
                ("2", RatioOrAuto::NotAuto(Ratio(
                    NonNegative(Number(2.0)),
                    NonNegative(Number(1.0))
                ))),
            ]
        );
    }

    #[test]
    #[should_panic]
    fn test_aspect_ratio_negative() {
        parse_property_value("aspect-ratio", "-1");
    }

    // Alignment //

    #[test]
    fn test_align_items() {
        parse_all_property_values(
            "align-items",
            BevyPropertyDeclaration::AlignItems,
            vec![
                ("stretch", ui::AlignItems::Stretch),
                ("center", ui::AlignItems::Center),
                ("flex-start", ui::AlignItems::FlexStart),
                ("flex-end", ui::AlignItems::FlexEnd),
                ("baseline", ui::AlignItems::Baseline),
            ]
        );
    }

    #[test]
    fn test_align_self() {
        parse_all_property_values(
            "align-self",
            BevyPropertyDeclaration::AlignSelf,
            vec![
                ("auto", ui::AlignSelf::Auto),
                ("stretch", ui::AlignSelf::Stretch),
                ("center", ui::AlignSelf::Center),
                ("flex-start", ui::AlignSelf::FlexStart),
                ("flex-end", ui::AlignSelf::FlexEnd),
                ("baseline", ui::AlignSelf::Baseline),
            ]
        );
    }

    #[test]
    fn test_align_content() {
        parse_all_property_values(
            "align-content",
            BevyPropertyDeclaration::AlignContent,
            vec![
                ("stretch", ui::AlignContent::Stretch),
                ("center", ui::AlignContent::Center),
                ("flex-start", ui::AlignContent::FlexStart),
                ("flex-end", ui::AlignContent::FlexEnd),
                ("space-between", ui::AlignContent::SpaceBetween),
                ("space-around", ui::AlignContent::SpaceAround),
            ]
        );
    }

    #[test]
    fn test_justify_content() {
        parse_all_property_values(
            "justify-content",
            BevyPropertyDeclaration::JustifyContent,
            vec![
                ("flex-start", ui::JustifyContent::FlexStart),
                ("flex-end", ui::JustifyContent::FlexEnd),
                ("center", ui::JustifyContent::Center),
                ("space-between", ui::JustifyContent::SpaceBetween),
                ("space-around", ui::JustifyContent::SpaceAround),
                ("space-evenly", ui::JustifyContent::SpaceEvenly),
            ]
        );
    }

    // Margins //

    #[test]
    fn test_margin() {
        parse_all_property_values(
            "margin",
            BevyPropertyDeclaration::Margin,
            sided_value_vec()
        );
    }

    #[test]
    fn test_margin_top() {
        parse_all_property_values(
            "margin-top",
            BevyPropertyDeclaration::MarginTop,
            auto_length_percentage_vec()
        );
    }

    #[test]
    fn test_margin_right() {
        parse_all_property_values(
            "margin-right",
            BevyPropertyDeclaration::MarginRight,
            auto_length_percentage_vec()
        );
    }

    #[test]
    fn test_margin_bottom() {
        parse_all_property_values(
            "margin-bottom",
            BevyPropertyDeclaration::MarginBottom,
            auto_length_percentage_vec()
        );
    }

    #[test]
    fn test_margin_left() {
        parse_all_property_values(
            "margin-left",
            BevyPropertyDeclaration::MarginLeft,
            auto_length_percentage_vec()
        );
    }

    // Padding //

    #[test]
    fn test_padding() {
        parse_all_property_values(
            "padding",
            BevyPropertyDeclaration::Padding,
            sided_value_vec()
        );
    }

    #[test]
    fn test_padding_top() {
        parse_all_property_values(
            "padding-top",
            BevyPropertyDeclaration::PaddingTop,
            auto_length_percentage_vec()
        );
    }

    #[test]
    fn test_padding_right() {
        parse_all_property_values(
            "padding-right",
            BevyPropertyDeclaration::PaddingRight,
            auto_length_percentage_vec()
        );
    }

    #[test]
    fn test_padding_bottom() {
        parse_all_property_values(
            "padding-bottom",
            BevyPropertyDeclaration::PaddingBottom,
            auto_length_percentage_vec()
        );
    }

    #[test]
    fn test_padding_left() {
        parse_all_property_values(
            "padding-left",
            BevyPropertyDeclaration::PaddingLeft,
            auto_length_percentage_vec()
        );
    }

    // Borders //

    #[test]
    fn test_border_width() {
        parse_all_property_values(
            "border-width",
            BevyPropertyDeclaration::BorderWidth,
            sided_value_vec()
        );
    }

    #[test]
    fn test_border_width_top() {
        parse_all_property_values(
            "border-width-top",
            BevyPropertyDeclaration::BorderWidthTop,
            auto_length_percentage_vec()
        );
    }

    #[test]
    fn test_border_width_right() {
        parse_all_property_values(
            "border-width-right",
            BevyPropertyDeclaration::BorderWidthRight,
            auto_length_percentage_vec()
        );
    }

    #[test]
    fn test_border_width_bottom() {
        parse_all_property_values(
            "border-width-bottom",
            BevyPropertyDeclaration::BorderWidthBottom,
            auto_length_percentage_vec()
        );
    }

    #[test]
    fn test_border_width_left() {
        parse_all_property_values(
            "border-width-left",
            BevyPropertyDeclaration::BorderWidthLeft,
            auto_length_percentage_vec()
        );
    }

    // Color //

    #[test]
    fn test_color() {
        parse_all_property_values(
            "color",
            BevyPropertyDeclaration::Color,
            vec![
                ("none", Color::NONE),
                ("transparent", Color::NONE),
                ("rgb(10, 20, 30)", Color::rgb_u8(10, 20, 30)),
                ("rgba(10, 20, 30, 0.5)", Color::rgba_u8(10, 20, 30, 128)),
                // Test against rgb_u8, as all colors defined with CSS will bevy::Color::rgba
                ("hsl(180, 60%, 70%)", Color::rgb_u8(133, 224, 224)),
                ("hsla(180, 60%, 70%, 0.5)", Color::rgba_u8(133, 224, 224, 128)),
                ("#ba55d3", Color::rgb_u8(186, 85, 211)),
                ("#abc", Color::rgb_u8(170, 187, 204)),
                ("red", Color::RED),
                ("lightsalmon", Color::rgb_u8(255, 160, 122)),
            ]
        );
    }
}