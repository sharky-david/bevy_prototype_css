use bevy::prelude::{Component, debug, warn};
use smallvec::SmallVec;

/// Component used to:
///     a) denote that an entity should be included in CSS styling passes
///     b) define the `id` and `classes` that will be used for said styling
/// An entity without a `CssTag` component will not be styled!
/// An entity with a `CssTag` component but no `id` or `classes` could still be styled; with a
/// wildcard (`*`) css selector for example.
#[derive(Component, Debug, Clone, Default)]
pub struct CssTag {
    pub(crate) id: Option<String>,
    // SmallVec is used for classes as there is often only one class specified
    pub(crate) classes: SmallVec<[String; 1]>,
}

impl CssTag {
    pub fn new() -> Self {
        Self {
            id: None,
            classes: SmallVec::new(),
        }
    }

    /// Will set the `id` of this `CssTag` to that supplied, overwriting any existing id.
    /// Supplying an empty string ("") will set the `id` to None
    /// The supplied `id_string` must not contain any ASCII whitespace
    /// See also: https://html.spec.whatwg.org/multipage/dom.html#the-id-attribute
    pub fn id(mut self, id_string: String) -> Self {
        if !id_string.is_empty() {
            no_whitespace(id_string.as_str());
            self.id = Some(id_string);
        } else {
            self.id = None;
            debug!("Empty id string supplied for CssTag::id")
        }
        self
    }

    pub fn new_id(id_string: String) -> Self {
        Self::new().id(id_string)
    }

    pub fn new_id_str(id_str: &str) -> Self {
        Self::new().id(id_str.to_string())
    }

    /// Will set the `classes` of this `CssTag` to those supplied, overwriting any that have already
    /// been set.
    /// The supplied `classes_string` is a series of string tokens separated by spaces.  Therefore
    /// spaces cannot be used as class names.  "a class" yields two classes: ["a", "class"].
    /// See also: https://html.spec.whatwg.org/multipage/dom.html#classes
    pub fn class(mut self, classes_string: String) -> Self {
        if !classes_string.is_empty() {
            self.classes = classes_string
                .split_ascii_whitespace()
                .filter(|s| !s.is_empty())
                .map(|s| {
                    // This is (strictly) side effecting, in that it can throw an assert panic!.
                    no_whitespace(s);
                    s.to_string()
                })
                .collect();
        } else {
            self.classes = SmallVec::new();
            debug!("Empty class string supplied for CssTag::class")
        }
        self
    }

    pub fn new_class(classes_string: String) -> Self {
        Self::new().class(classes_string)
    }

    pub fn new_class_str(classes_str: &str) -> Self {
        Self::new().class(classes_str.to_string())
    }
}

impl From<&str> for CssTag {
    /// Allows `CssTag`s to be defined from a css selectors style strings.
    ///     eg: "#id.class1.class2"
    fn from(selectors: &str) -> Self {
        Self::from(selectors.to_string())
    }
}

impl From<String> for CssTag {
    /// Allows `CssTag`s to be defined from a css selectors style strings.
    /// '#' indicates the following string slice is an ID.  Only the last id given is used.
    /// '.' indicates the following string slice is a class.
    ///
    /// If the given string does not start with '#'/'.', a warning is produced, but the starting
    /// slice is otherwise ignored.
    ///
    /// Any ASCII whitespace in id/class string slices will panic! with an assert error.
    ///
    /// See also: https://html.spec.whatwg.org/multipage/dom.html#the-id-attribute
    /// See also: https://html.spec.whatwg.org/multipage/dom.html#classes
    ///
    /// Example: "#id.class1.class2"
    fn from(selectors: String) -> Self {
        let mut id = String::new();
        let mut classes = String::new();
        let (mut is_id, mut is_class) = (false, false);
        let mut warned = false;
        if selectors.is_empty() {
            debug!("Empty selectors string supplied for CssTag::from")
        }
        for char in selectors.chars() {
            match char {
                '#' => {
                    is_id = true;
                    is_class = false;
                    id = String::new();
                },
                '.' => {
                    is_id = false;
                    is_class = true;
                    classes.push(' ')
                },
                c if is_id => id.push(c),
                c if is_class => classes.push(c),
                _ => {
                    if !warned {
                        warn!("Selectors string does not start with '#' (id) or '.' (class)")
                    }
                    warned = true;
                },
            }
        }
        Self::new().id(id).class(classes)
    }
}

fn no_whitespace(str: &str) {
    // Supposedly this is faster than str.contains(char::::is_ascii_whitespace)
    // Ref: comment on https://stackoverflow.com/a/64361042
    assert!(
        !str.as_bytes().iter().any(u8::is_ascii_whitespace),
        "A CSS id/class cannot contain any ASCII whitespace"
    );
}