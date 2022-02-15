use cssparser::Parser;
use crate::{
    errors::BevyCssParsingError,
    values::Parse,
};

#[derive(Copy, Clone, Debug)]
pub struct SidedValue<T> {
    pub top: T,
    pub right: T,
    pub bottom: T,
    pub left: T,
}

impl<T: Clone + Copy> SidedValue<T> {
    #[inline]
    fn new_4(top: T, right: T, bottom: T, left: T) -> Self {
        Self { top, right, bottom, left, }
    }

    #[inline]
    fn new_3(top: T, right_left: T, bottom: T) -> Self {
        Self { top, right: right_left, bottom, left: right_left, }
    }

    #[inline]
    fn new_2(top_bottom: T, right_left: T) -> Self {
        Self { top: top_bottom, right: right_left, bottom: top_bottom, left: right_left, }
    }

    #[inline]
    fn new_1(value: T) -> Self {
        Self { top: value, right: value, bottom: value, left: value, }
    }

    pub fn parse_internal<'i, 't>(
        input: &mut Parser<'i, 't>,
        sides_parser: impl Fn(&mut Parser<'i, 't>) -> Result<T, BevyCssParsingError<'i>>,
    ) -> Result<Self, BevyCssParsingError<'i>> {
        let first = sides_parser(input)?;
        let second =
            if let Ok(second) = input.try_parse(|i| sides_parser(i)) { second }
            // only 1 value was given
            else { return Ok(Self::new_1(first)) };
        let third =
            if let Ok(third) = input.try_parse(|i| sides_parser(i)) { third }
            // only 2 values were given
            else { return Ok(Self::new_2(first, second)) };
        let fourth =
            if let Ok(fourth) = input.try_parse(|i| sides_parser(i)) { fourth }
            // only 3 values were given
            else { return Ok(Self::new_3(first, second, third)) };
        // 4 values were given
        Ok(Self::new_4(first, second, third, fourth))
    }
}

impl<T: Parse + Clone + Copy> Parse for SidedValue<T> {
    fn parse<'i, 't>(input: &mut Parser<'i, 't>) -> Result<Self, BevyCssParsingError<'i>> {
        Self::parse_internal(input, <T as Parse>::parse)
    }
}