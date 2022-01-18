use std::{
    borrow::{Borrow, Cow},
    fmt::{self, Debug,},
    ops::Deref,
    str::FromStr,
};
use cssparser::{CowRcStr, ToCss};

#[derive(Default, Debug, Clone, Ord, PartialOrd, Eq, PartialEq, Hash)]
pub struct CssString(String);

impl Deref for CssString {
    type Target = str;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl ToCss for CssString {
    fn to_css<W>(&self, dest: &mut W) -> std::fmt::Result where W: fmt::Write {
        cssparser::serialize_identifier(&self.0, dest)
    }
}

impl fmt::Display for CssString {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        fmt::Display::fmt(&self.0, f)
    }
}

impl From<String> for CssString {
    fn from(value: String) -> Self {
        Self(value)
    }
}

impl<'a> From<&'a str> for CssString {
    fn from(value: &'a str) -> Self {
        Self(value.to_owned())
    }
}

impl<'i> From<CowRcStr<'i>> for CssString {
    fn from(value: CowRcStr<'i>) -> Self {
        Self::from(value.as_ref())
    }
}

impl<'a> From<Cow<'a, str>> for CssString {
    fn from(value: Cow<'a, str>) -> Self {
        Self(value.into_owned())
    }
}

impl<'a, 'i> From<&'a CowRcStr<'i>> for CssString {
    fn from(value: &'a CowRcStr<'i>) -> Self {
        Self::from(value.as_ref())
    }
}

impl FromStr for CssString {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(Self(s.to_owned()))
    }
}

impl Borrow<str> for CssString {
    fn borrow(&self) -> &str {
        self.deref()
    }
}