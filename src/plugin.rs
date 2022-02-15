use bevy::prelude::*;
use crate::{
    context::CssContext,
    css_tag::CssTag,
    rules::{BevyCssRule, BevyStyleRule},
    stylesheet::{CssStylesheet, CssStylesheetLoader}
};

pub struct CssPlugin;

impl Plugin for CssPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_asset::<CssStylesheet>()
            .init_asset_loader::<CssStylesheetLoader>()
            .add_system(apply_styles);
    }
}

/// System to manage stylesheet application to entities
// @todo Only update styles when the style context changes
// @todo Make the order of allied sheets deterministic (need to decided on cascading rules)
// @todo Add support for Component matching/selectors
fn apply_styles(
    mut stylesheet_events: EventReader<AssetEvent<CssStylesheet>>,
    assets: Res<Assets<CssStylesheet>>,
    mut styles_query: Query<(&CssTag, Option<&mut Style>, Option<&mut UiColor>)>,
) {
    for event in stylesheet_events.iter() {
        match event {
            AssetEvent::Created { handle } | AssetEvent::Modified { handle } =>
                apply_stylesheet(assets.get(handle).unwrap(), &mut styles_query),
            _ => ()
        }
    }
}

fn apply_stylesheet(
    stylesheet: &CssStylesheet,
    styles_query: &mut Query<(&CssTag, Option<&mut Style>, Option<&mut UiColor>)>,
) {
    for rule in stylesheet.rules.iter() {
        match rule {
            BevyCssRule::Style(style_rule) => apply_style_rule(style_rule, styles_query)
        }
    }
}

fn apply_style_rule(
    style_rule: &BevyStyleRule,
    query: &mut Query<(&CssTag, Option<&mut Style>, Option<&mut UiColor>)>
) {
    for (tag, mut style_opt, mut color_opt) in query.iter_mut() {
        let CssTag { id, classes } = tag;
        // @fixme Create a proper context, not a default
        let context = CssContext::default();
        if style_rule.selectors.matches(&id, &classes) {
            for property in style_rule.declarations.iter() {
                if let Some(mut style) = style_opt.as_mut() { property.modify_style(&context, &mut style) }
                if let Some(mut color) = color_opt.as_mut() { property.modify_color(&mut color) }
            }
        }
    }
}