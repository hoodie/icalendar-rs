use std::borrow::Cow;

use aho_corasick::AhoCorasick;

/// A zero-copy string parsed from an iCal input.
#[derive(Debug, Eq, Clone)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct ParseString<'a>(Cow<'a, str>);

impl ParseString<'_> {
    pub fn to_owned(&self) -> ParseString<'static> {
        match self.0 {
            Cow::Borrowed(s) => ParseString(Cow::Owned(s.to_owned())),
            Cow::Owned(ref s) => ParseString(Cow::Owned(s.clone())),
        }
    }

    pub fn into_owned(self) -> ParseString<'static> {
        match self.0 {
            Cow::Borrowed(s) => ParseString(Cow::Owned(s.to_owned())),
            Cow::Owned(s) => ParseString(Cow::Owned(s)),
        }
    }

    pub fn as_str(&self) -> &str {
        self.0.as_ref()
    }
}

impl<'a> ParseString<'a> {
    pub fn unescape_text(self) -> ParseString<'a> {
        let ac = AhoCorasick::builder()
            .match_kind(aho_corasick::MatchKind::LeftmostFirst)
            .build([r#"\\"#, r#"\,"#, r#"\;"#, r#"\:"#, r#"\N"#, r#"\n"#])
            .unwrap();
        ac.replace_all(self.0.as_ref(), &[r#"\"#, ",", ";", ":", "\n", "\n"])
            .into()
    }
}

impl PartialEq<Self> for ParseString<'_> {
    fn eq(&self, rhs: &Self) -> bool {
        self.as_ref() == rhs.as_ref()
    }
}

impl PartialEq<&str> for ParseString<'_> {
    fn eq(&self, rhs: &&str) -> bool {
        self.as_ref() == *rhs
    }
}

impl AsRef<str> for ParseString<'_> {
    fn as_ref(&self) -> &str {
        self.0.as_ref()
    }
}

impl From<ParseString<'static>> for String {
    fn from(val: ParseString<'static>) -> Self {
        val.0.into_owned()
    }
}

impl From<String> for ParseString<'static> {
    fn from(s: String) -> Self {
        ParseString(Cow::Owned(s))
    }
}

impl std::fmt::Display for ParseString<'_> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl<'a> From<&'a str> for ParseString<'a> {
    fn from(s: &'a str) -> Self {
        ParseString(Cow::Borrowed(s))
    }
}
