use bevy::ui;
use cssparser::{
    Parser,
    match_ignore_ascii_case, _cssparser_internal_to_lowercase,
};
use crate::{
    errors::{BevyCssParsingError, BevyCssParsingErrorKind},
    values::Parse,
};

impl Parse for ui::Display {
    fn parse<'i, 't>(input: &mut Parser<'i, 't>) -> Result<Self, BevyCssParsingError<'i>> {
        let start = input.current_source_location();
        let ident = input.expect_ident()?;
        Ok(match_ignore_ascii_case! { ident,
            "flex" => ui::Display::Flex,
            "none" => ui::Display::None,
            _ => return Err(start.new_custom_error(
                BevyCssParsingErrorKind::InvalidValue(ident.clone(), None)
            ))
        })
    }
}

impl Parse for ui::PositionType {
    fn parse<'i, 't>(input: &mut Parser<'i, 't>) -> Result<Self, BevyCssParsingError<'i>> {
        let start = input.current_source_location();
        let ident = input.expect_ident()?;
        Ok(match_ignore_ascii_case! { ident,
            "relative" => ui::PositionType::Relative,
            "absolute" => ui::PositionType::Absolute,
            _ => return Err(start.new_custom_error(
                BevyCssParsingErrorKind::InvalidValue(ident.clone(), None)
            ))
        })
    }
}

impl Parse for ui::Direction {
    fn parse<'i, 't>(input: &mut Parser<'i, 't>) -> Result<Self, BevyCssParsingError<'i>> {
        let start = input.current_source_location();
        let ident = input.expect_ident()?;
        Ok(match_ignore_ascii_case! { ident,
            "inherit" => ui::Direction::Inherit,
            "ltr" => ui::Direction::LeftToRight,
            "rtl" => ui::Direction::RightToLeft,
            _ => return Err(start.new_custom_error(
                BevyCssParsingErrorKind::InvalidValue(ident.clone(), None)
            ))
        })
    }
}

impl Parse for ui::FlexDirection {
    fn parse<'i, 't>(input: &mut Parser<'i, 't>) -> Result<Self, BevyCssParsingError<'i>> {
        let start = input.current_source_location();
        let ident = input.expect_ident()?;
        Ok(match_ignore_ascii_case! { ident,
            "row" => ui::FlexDirection::Row,
            "column" => ui::FlexDirection::Column,
            "row-reverse" => ui::FlexDirection::RowReverse,
            "column-reverse" => ui::FlexDirection::ColumnReverse,
            _ => return Err(start.new_custom_error(
                BevyCssParsingErrorKind::InvalidValue(ident.clone(), None)
            ))
        })
    }
}

impl Parse for ui::FlexWrap {
    fn parse<'i, 't>(input: &mut Parser<'i, 't>) -> Result<Self, BevyCssParsingError<'i>> {
        let start = input.current_source_location();
        let ident = input.expect_ident()?;
        Ok(match_ignore_ascii_case! { ident,
            "nowrap" => ui::FlexWrap::NoWrap,
            "wrap" => ui::FlexWrap::Wrap,
            "wrap-reverse" => ui::FlexWrap::WrapReverse,
            _ => return Err(start.new_custom_error(
                BevyCssParsingErrorKind::InvalidValue(ident.clone(), None)
            ))
        })
    }
}

impl Parse for ui::AlignItems {
    fn parse<'i, 't>(input: &mut Parser<'i, 't>) -> Result<Self, BevyCssParsingError<'i>> {
        let start = input.current_source_location();
        let ident = input.expect_ident()?;
        Ok(match_ignore_ascii_case! { ident,
            "flex-start" => ui::AlignItems::FlexStart,
            "flex-end" => ui::AlignItems::FlexEnd,
            "center" => ui::AlignItems::Center,
            "baseline" => ui::AlignItems::Baseline,
            "stretch" => ui::AlignItems::Stretch,
            _ => return Err(start.new_custom_error(
                BevyCssParsingErrorKind::InvalidValue(ident.clone(), None)
            ))
        })
    }
}

impl Parse for ui::AlignSelf {
    fn parse<'i, 't>(input: &mut Parser<'i, 't>) -> Result<Self, BevyCssParsingError<'i>> {
        let start = input.current_source_location();
        let ident = input.expect_ident()?;
        Ok(match_ignore_ascii_case! { ident,
            "auto" => ui::AlignSelf::Auto,
            "flex-start" => ui::AlignSelf::FlexStart,
            "flex-end" => ui::AlignSelf::FlexEnd,
            "center" => ui::AlignSelf::Center,
            "baseline" => ui::AlignSelf::Baseline,
            "stretch" => ui::AlignSelf::Stretch,
            _ => return Err(start.new_custom_error(
                BevyCssParsingErrorKind::InvalidValue(ident.clone(), None)
            ))
        })
    }
}

impl Parse for ui::AlignContent {
    fn parse<'i, 't>(input: &mut Parser<'i, 't>) -> Result<Self, BevyCssParsingError<'i>> {
        let start = input.current_source_location();
        let ident = input.expect_ident()?;
        Ok(match_ignore_ascii_case! { ident,
            "flex-start" => ui::AlignContent::FlexStart,
            "flex-end" => ui::AlignContent::FlexEnd,
            "center" => ui::AlignContent::Center,
            "stretch" => ui::AlignContent::Stretch,
            "space-between" => ui::AlignContent::SpaceBetween,
            "space-around" => ui::AlignContent::SpaceAround,
            _ => return Err(start.new_custom_error(
                BevyCssParsingErrorKind::InvalidValue(ident.clone(), None)
            ))
        })
    }
}

impl Parse for ui::JustifyContent {
    fn parse<'i, 't>(input: &mut Parser<'i, 't>) -> Result<Self, BevyCssParsingError<'i>> {
        let start = input.current_source_location();
        let ident = input.expect_ident()?;
        Ok(match_ignore_ascii_case! { ident,
            "flex-start" => ui::JustifyContent::FlexStart,
            "flex-end" => ui::JustifyContent::FlexEnd,
            "center" => ui::JustifyContent::Center,
            "space-between" => ui::JustifyContent::SpaceBetween,
            "space-around" => ui::JustifyContent::SpaceAround,
            "space-evenly" => ui::JustifyContent::SpaceEvenly,
            _ => return Err(start.new_custom_error(
                BevyCssParsingErrorKind::InvalidValue(ident.clone(), None)
            ))
        })
    }
}

impl Parse for ui::Overflow {
    fn parse<'i, 't>(input: &mut Parser<'i, 't>) -> Result<Self, BevyCssParsingError<'i>> {
        let start = input.current_source_location();
        let ident = input.expect_ident()?;
        Ok(match_ignore_ascii_case! { ident,
            "visible" => ui::Overflow::Visible,
            "hidden" => ui::Overflow::Hidden,
            _ => return Err(start.new_custom_error(
                BevyCssParsingErrorKind::InvalidValue(ident.clone(), None)
            ))
        })
    }
}