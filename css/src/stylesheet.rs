use bevy::{
    asset::{AssetLoader, LoadContext, LoadedAsset},
    prelude::Style,
    reflect::TypeUuid,
    utils:: BoxedFuture,
};
use cssparser::{Parser, ParserInput};
use crate::{
    CssContext,
    parser::{BevySheetParser, BevyPropertyListParser},
    properties::BevyPropertyDeclaration,
    rules::BevyCssRule
};

/// This struct doesn't actually store any styles in memory.  It is just a way to create a `Style`
/// struct from a CSS declaration block string (inline).
pub struct CssStyle<'i>(pub &'i str);

impl<'i> CssStyle<'i> {
    pub fn parse_inline(&self) -> Vec<BevyPropertyDeclaration> {
        let mut parser_input = ParserInput::new(self.0);
        let mut input = Parser::new(&mut parser_input);
        BevyPropertyListParser::parse_with(&mut input)
    }

    pub fn to_style(&self, context: &CssContext) -> Style {
        let mut style = Style::default();
        let properties = self.parse_inline();
        for property in properties.iter() {
            property.modify_style(context, &mut style)
        }
        style
    }
}

/// Stored as an asset
#[derive(Debug, TypeUuid)]
#[uuid = "da9c2e27-0fe0-4fca-b9d1-5012c042a882"]  // from: https://www.uuidgenerator.net/version4
pub struct CssStylesheet {
    pub rules: Vec<BevyCssRule>,
}

impl CssStylesheet {
    pub fn parse_sheet(css_string: &str) -> Vec<BevyCssRule> {
        let mut parser_input = ParserInput::new(css_string);
        let mut input = Parser::new(&mut parser_input);
        BevySheetParser::parse_with(&mut input)
    }
}

impl From<&str> for CssStylesheet {
    fn from(css_string: &str) -> Self {
        Self {
            rules: Self::parse_sheet(css_string)
        }
    }
}

#[derive(Default)]
pub(crate) struct CssStylesheetLoader;

impl AssetLoader for CssStylesheetLoader {
    fn load<'a>(
        &'a self,
        bytes: &'a [u8],
        load_context: &'a mut LoadContext
    ) -> BoxedFuture<'a, anyhow::Result<()>> {
        Box::pin(async move {
            let css_file_string = std::str::from_utf8(bytes)?;
            let stylesheet = CssStylesheet::from(css_file_string);
            load_context.set_default_asset(LoadedAsset::new(stylesheet));
            Ok(())
        })
    }

    fn extensions(&self) -> &[&str] {
        &["css"]
    }
}