use std::sync::Arc;
use crate::{CssClass, CssId, properties::BevyPropertyDeclaration, selectors::BevySelectorList};

/** Does not cover all possible top level CSS rules, only the ones that make sense within Bevy **/
#[derive(Debug, Clone)]
pub enum BevyCssRule {

    // @font-face
    //FontFace(FontFaceAtRule),

    // Normal styles (e.g. node { margin: 2px; }
    Style(BevyStyleRule),

}

/// A rule for one style block.  I.e. one selector group, and the declarations (between the curly
/// braces - `{ ... }`) for the selector block.
#[derive(Debug, Clone)]
pub struct BevyStyleRule {
    // A list of all the selectors specified (i.e. could be a comma separated list of selectors)
    pub selectors: BevySelectorList,

    // A list of all the declarations.  I.e. everything between the `{ /* ... */ }`
    // Rc is used to avoid cloning of the declarations vec for every selector in the list above
    pub declarations: Arc<Vec<BevyPropertyDeclaration>>
}

impl BevyStyleRule {
    pub fn is_applied(&self, id: Option<&CssId>, classes: Option<&CssClass>) -> bool {
        match self.selectors.0.first() {
            None => false,
            Some(selector) => match selector {
                _ if selector.is_universal() => true,
                _ if id.is_none() && classes.is_none() => false,
                _ => {
                    for part in selector.iter() {
                        todo!("Implement selector matching")
                    }
                    false
                }
            }
        }
    }
}