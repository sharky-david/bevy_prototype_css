pub mod context;
pub mod css_strings;
pub mod css_tag;
pub mod errors;
pub mod parser;
pub mod plugin;
pub mod properties;
pub mod rules;
pub mod selectors;
pub mod stylesheet;
pub mod values;

pub mod prelude {
    pub use crate::context::CssContext;
    pub use crate::css_tag::CssTag;
    pub use crate::plugin::CssPlugin;
    pub use crate::stylesheet::{
        CssStyle, CssStylesheet,
    };
}

pub use crate::prelude::{
    CssPlugin, CssTag, CssStylesheet,       // For Stylesheets
    CssContext, CssStyle,                   // For inline styles
};