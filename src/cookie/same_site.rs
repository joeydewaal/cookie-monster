use std::fmt::{self, Write};

use crate::Cookie;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum SameSite {
    Strict,
    Lax,
    None,
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
