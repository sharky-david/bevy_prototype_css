use bevy::{ui};
use bevy::reflect::Reflect;
use crate::{
    context::CssContext,
    values::{LengthPercentage, LengthPercentageOrAuto, SidedValue}
};

/// Very similar to the standard library `From<T>` trait, but allows the `::from()` function to take
/// a `CssContext` reference for the conversion.
pub trait ContextualFrom<T>: Sized {
    fn contextual_from(context: &CssContext, _: T) -> Self;
}

pub trait ContextualInto<T>: Sized {
    fn contextual_into(self, context: &CssContext) -> T;
}

impl<T, U: ContextualFrom<T>> ContextualInto<U> for T {
    fn contextual_into(self, context: &CssContext) -> U {
        U::contextual_from(context, self)
    }
}

impl ContextualFrom<LengthPercentageOrAuto> for ui::Val {
    fn contextual_from(context: &CssContext, len: LengthPercentageOrAuto) -> Self {
        match len {
            LengthPercentageOrAuto::Auto => ui::Val::Auto,
            LengthPercentageOrAuto::NotAuto(len_pc) => match len_pc {
                // ui::Val::Percent takes values of 0.0 to 100.0 (not 0.0 to 1.0)
                LengthPercentage::Percentage(pc) => ui::Val::Percent(pc.as_number()),
                LengthPercentage::Length(len) => ui::Val::Px(len.to_computed_px(context))
            },
        }
    }
}

impl<U, T> ContextualFrom<SidedValue<T>> for ui::UiRect<U>
where
    U: Reflect + PartialEq,
    T: ContextualInto<U>
{
    fn contextual_from(context: &CssContext, sided_value: SidedValue<T>) -> Self {
        Self {
            top: sided_value.top.contextual_into(context),
            right: sided_value.right.contextual_into(context),
            bottom: sided_value.bottom.contextual_into(context),
            left: sided_value.left.contextual_into(context),
        }
    }
}