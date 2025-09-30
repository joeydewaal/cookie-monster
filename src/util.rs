use std::borrow::Cow;

#[derive(Clone)]
pub(crate) enum TinyStr {
    Static(&'static str),
    Owned(Box<str>),
    Indexed(usize, usize),
}

impl TinyStr {
    pub fn as_str<'a>(&'a self, buf: Option<&'a str>) -> &'a str {
        match self {
            TinyStr::Static(s) => s,
            TinyStr::Owned(owned) => owned,
            TinyStr::Indexed(start, end) => &buf.unwrap()[(*start)..(*end)],
        }
    }

    pub fn index(needle: &str, haystack: *const u8) -> Self {
        let haystack_start = haystack as usize;
        let needle_start = needle.as_ptr() as usize;

        let start = needle_start - haystack_start;
        let end = start + needle.len();
        Self::Indexed(start, end)
    }

    pub fn empty() -> TinyStr {
        TinyStr::Static("")
    }
}

impl<T> From<T> for TinyStr
where
    T: Into<Cow<'static, str>>,
{
    fn from(value: T) -> Self {
        match value.into() {
            Cow::Owned(owned) => TinyStr::Owned(owned.into()),
            Cow::Borrowed(borrowed) => TinyStr::Static(borrowed),
        }
    }
}

impl Default for TinyStr {
    fn default() -> Self {
        Self::empty()
    }
}
