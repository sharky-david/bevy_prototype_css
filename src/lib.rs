pub use css as internal;

pub mod prelude {
    pub use css::{
        context::CssContext,
        css_tag::CssTag,
        plugin::CssPlugin,
        stylesheet::{
            CssStyle, CssStylesheet,
        },
    };
}

pub use prelude::{
    CssPlugin, CssTag, CssStylesheet,       // For Stylesheets
    CssContext, CssStyle,                   // For inline styles
};