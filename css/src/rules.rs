use std::sync::Arc;
use crate::{
    properties::BevyPropertyDeclaration,
    selectors::BevySelectorList
};

/// Does not cover all possible top level CSS rules, only the ones that make sense within Bevy
#[derive(Debug, Clone)]
pub enum BevyCssRule {

    // @font-face
    //FontFace(FontFaceAtRule),

    /// Normal styles (e.g. node { margin: 2px; }
    Style(BevyStyleRule),

}

/// A rule for one style block.  I.e. one selector group, and the declarations (between the curly
/// braces - `{ ... }`) for the selector block.
#[derive(Debug, Clone)]
pub struct BevyStyleRule {
    /// A list of all the selectors specified in the `.css` document
    pub selectors: BevySelectorList,

    /// A list of all the declarations.  I.e. everything between the `{ /* ... */ }`
    // Want to use Rc to avoid cloning of the declarations vec for every selector in the list above
    // Use Arc instead of Rc as bevy systems can run on any/many threads
    pub declarations: Arc<Vec<BevyPropertyDeclaration>>
}