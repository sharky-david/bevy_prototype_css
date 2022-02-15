use bevy::{
    math::Vec2,
    text::TextStyle,
};

/// A simple data holding struct that can be passed around to help construct or convert various css
/// values that may depend on the app context somehow.
// @fixme this is a bit hacky.  It works, but feels clumsy.
#[derive(Copy, Clone, Debug)]
pub struct CssContext {
    pub font_size: f32,
    pub root_font_size: f32,
    pub vertical_text: bool,
    pub viewport_size: Vec2,
}

impl Default for CssContext {
    fn default() -> Self {
        Self {
            font_size: TextStyle::default().font_size,
            root_font_size: TextStyle::default().font_size,
            vertical_text: false,
            viewport_size: Vec2::default(),
        }
    }
}