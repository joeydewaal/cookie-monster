use std::fmt::{self, Write};

use crate::Cookie;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum SameSite {
    Strict,
    Lax,
    None,
}

impl SameSite {
    pub(crate) fn from_attribute_value(value: &mut str) -> Option<Self> {
        value.make_ascii_lowercase();

        match &*value {
            "strict" => Some(SameSite::Strict),
            "lax" => Some(SameSite::Lax),
            "none" => Some(SameSite::None),
            _ => None,
        }
    }
}

impl Cookie {
    pub(crate) fn serialize_same_site(&self, buf: &mut String) {
        let Some(same_site) = self.same_site else {
            return;
        };

        let _ = write!(buf, "; SameSite={same_site}");
    }
}

impl fmt::Display for SameSite {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            SameSite::Strict => write!(f, "Strict"),
            SameSite::Lax => write!(f, "Lax"),
            SameSite::None => write!(f, "None"),
        }
    }
}
