use bevy::prelude::*;
use smallvec::SmallVec;
use crate::{
    rules::BevyCssRule,
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

#[derive(Component, Debug)]
pub struct CssId(String);

impl From<String> for CssId {
    fn from(id: String) -> Self {
        assert!(!id.contains(' '));
        Self(id)
    }
}

impl From<&str> for CssId {
    fn from(id: &str) -> Self {
        Self::from(id.to_string())
    }
}

#[derive(Component, Debug)]
pub struct CssClass(SmallVec<[String; 1]>);

impl From<&str> for CssClass {
    fn from(classes: &str) -> Self {
        Self(classes.split(' ').map(|s| s.to_string()).collect())
    }
}

impl From<String> for CssClass {
    fn from(classes: String) -> Self {
        Self::from(classes.as_str())
    }
}

fn apply_styles(
    mut commands: Commands,
    stylesheets: Res<Assets<CssStylesheet>>,
    styles_query: Query<(&mut Style, Option<&CssId>, Option<&CssClass>), With<Node>>,
) {
    // @todo Only update styles when the style context changes or a stylesheet is loaded/changed
    // @todo Make the order of allied sheet deterministic (need to decided on cascading rules)
    // @todo Add support of `CssType`s
    for (handle, stylesheet) in stylesheets.iter() {
        for rule in stylesheet.rules.iter() {
            if let BevyCssRule::Style(style_rule) = rule {
                for (style, css_id, css_classes) in styles_query.iter() {
                    if style_rule.is_applied(css_id, css_classes) {
                        println!("{:?} applies to [{:?}, {:?}]", style_rule.selectors.0, css_id, css_classes)
                    }
                }
            }
        }
    }
}