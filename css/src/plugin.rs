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

/// A CssContext used for node-relative style properties (e.g. `1em`)
fn create_context(_style: &Style) -> CssContext {
    // @fixme Create a proper context, not a default
    CssContext::default()
}

/// System to manage stylesheet application to entities
// @todo Only update styles when the style context changes
// @todo Make the order of allied sheets deterministic (need to decided on cascading rules)
// @todo Add support for Component matching/selectors
fn apply_styles(
    mut stylesheet_events: EventReader<AssetEvent<CssStylesheet>>,
    assets: Res<Assets<CssStylesheet>>,
    mut styles_query: Query<(&mut Style, &CssTag)>,
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
    styles_query: &mut Query<(&mut Style, &CssTag)>,
) {
    for rule in stylesheet.rules.iter() {
        match rule {
            BevyCssRule::Style(style_rule) => apply_style_rule(style_rule, styles_query)
        }
    }
}

fn apply_style_rule<'a>(
    style_rule: &BevyStyleRule,
    query: &mut Query<(&mut Style, &CssTag)>
) {
    for (mut style, CssTag { id, classes }) in query.iter_mut() {
        let context = create_context(&style);
        if style_rule.selectors.matches(&id, &classes) {
            for property in style_rule.declarations.iter() {
                property.modify_style(&context, &mut style)
            }
        }
    }
}