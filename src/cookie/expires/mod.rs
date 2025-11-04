use std::fmt::{Debug, Write};

use super::Cookie;

#[cfg(feature = "time")]
pub mod dep_time;

#[cfg(feature = "chrono")]
pub mod dep_chrono;

#[cfg(feature = "jiff")]
pub mod dep_jiff;

const REMOVE: &str = "Thu, 01 Jan 1970 00:00:00 GMT";

/// The Expires attribute.
#[derive(Clone, Default)]
pub enum Expires {
    // So a user can still remove a cookie without needing any of the datetime features.
    Remove,
    // No expiry time.
    #[default]
    Session,
    Exp(ExpVal),
}

#[derive(Clone, Default)]
pub struct ExpVal {
    #[cfg(feature = "time")]
    time: Option<time::OffsetDateTime>,
    #[cfg(feature = "chrono")]
    chrono: Option<chrono::DateTime<chrono::Utc>>,
    #[cfg(feature = "jiff")]
    jiff: Option<jiff::Zoned>,
}

impl Expires {
    pub fn remove() -> Self {
        #![allow(unreachable_code)]

        #[cfg(feature = "jiff")]
        return Self::remove_jiff();

        #[cfg(feature = "chrono")]
        return Self::remove_chrono();

        #[cfg(feature = "time")]
        return Self::remove_time();

        Self::Remove
    }
}

impl Cookie {
    /// If the Expires attribute is not set, the expiration of the cookie is tied to the session
    /// with the user-agent.
    pub fn is_expires_set(&self) -> bool {
        !matches!(self.expires, Expires::Session)
    }

    pub(crate) fn serialize_expire(&self, buf: &mut String) -> crate::Result<()> {
        // Only one can be set at all times, except while parsing but then the first match is used.
        match &self.expires {
            #[cfg(feature = "time")]
            Expires::Exp(ExpVal { time: Some(t), .. }) => dep_time::ser_expires(t, buf),
            #[cfg(feature = "chrono")]
            Expires::Exp(ExpVal {
                chrono: Some(c), ..
            }) => dep_chrono::ser_expires(c, buf),
            #[cfg(feature = "jiff")]
            Expires::Exp(ExpVal { jiff: Some(j), .. }) => dep_jiff::ser_expires(j, buf),
            Expires::Remove => {
                let _ = write!(buf, "; Expires={REMOVE}");
                Ok(())
            }

            _ => Ok(()),
        }
    }
}

impl PartialEq for Expires {
    fn eq(&self, other: &Self) -> bool {
        match (self, other) {
            (Expires::Remove, Expires::Remove) => true,
            (Expires::Session, Expires::Session) => true,
            (Expires::Exp(_s), Expires::Exp(_o)) => {
                // TODO: double check this.
                #[cfg(feature = "time")]
                if _s.time == _o.time {
                    return true;
                }

                #[cfg(feature = "chrono")]
                if _s.chrono == _o.chrono {
                    return true;
                }
                #[cfg(feature = "jiff")]
                if _s.jiff == _o.jiff {
                    return true;
                }

                false
            }
            _ => false,
        }
    }
}

impl Debug for Expires {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Remove => write!(f, "{REMOVE}"),
            Self::Session => write!(f, "Session"),
            Self::Exp(_exp) => {
                let mut debug = f.debug_struct("Exp");

                #[cfg(feature = "time")]
                let debug = debug.field("expires_time", &_exp.time);

                #[cfg(feature = "chrono")]
                let debug = debug.field("expires_chrono", &_exp.chrono);

                #[cfg(feature = "jiff")]
                let debug = debug.field("expires_jiff", &_exp.jiff);

                debug.finish()
            }
        }
    }
}
