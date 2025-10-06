use std::fmt::{Debug, Write};

use super::Cookie;

#[cfg(feature = "time")]
pub mod dep_time;

#[cfg(feature = "chrono")]
pub mod dep_chrono;

#[cfg(feature = "jiff")]
pub mod dep_jiff;

const REMOVE: &str = "Thu, 01 Jan 1970 00:00:00 GMT";

#[cfg(any(feature = "chrono", feature = "jiff"))]
pub mod formats {
    // Sun, 06 Nov 1994 08:49:37 GMT (RFC)
    pub static FMT1: &str = "%a, %d %b %Y %T GMT";
    // Sunday, 06-Nov-94 08:49:37 GMT (RFC)
    pub static FMT2: &str = "%A, %d-%b-%y %T GMT";
    // Sun Nov  6 08:49:37 1994 (RFC)
    pub static FMT3: &str = "%a %b %e %T %Y";
    // Thu, 10-Sep-2069 20:00:00 GMT
    pub static FMT4: &str = "%a, %d-%b-%Y %T GMT";
}

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
    jiff: Option<jiff::Timestamp>,
}

impl Expires {
    pub fn remove() -> Self {
        Self::Remove
    }
}

pub fn parse_expires(_value: &str) -> Expires {
    Expires::Exp(ExpVal {
        #[cfg(feature = "time")]
        time: dep_time::parse_expires_time(_value),
        #[cfg(feature = "chrono")]
        chrono: dep_chrono::parse_expires(_value),
        #[cfg(feature = "jiff")]
        jiff: dep_jiff::parse_expires(_value),
    })
}

impl Cookie {
    pub fn serialize_expire(&self, buf: &mut String) -> crate::Result<()> {
        // Only one can be set at all times.
        match self.expires {
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

#[cfg(test)]
pub mod test_cases {
    #[allow(unused)]
    pub const ABBREVIATED_YEARS: &[(&str, u32, u32, i32, u32, u32, u32)] = &[
        (
            "foo=bar; expires=Thu, 10-Sep-20 20:00:00 GMT",
            10,
            9,
            2020,
            20,
            00,
            00,
        ),
        (
            "foo=bar; expires=Mon, 10-Sep-68 20:00:00 GMT",
            10,
            9,
            2068,
            20,
            00,
            00,
        ),
        (
            "foo=bar; expires=Wed, 10-Sep-69 20:00:00 GMT",
            10,
            9,
            1969,
            20,
            00,
            00,
        ),
        (
            "foo=bar; expires=Fri, 10-Sep-99 20:00:00 GMT",
            10,
            9,
            1999,
            20,
            00,
            00,
        ),
        (
            "foo=bar; expires=Tue, 10-Sep-2069 20:00:00 GMT",
            10,
            9,
            2069,
            20,
            00,
            00,
        ),
    ];

    #[allow(unused)]
    pub const ALTERNATIVE_FMTS: &[(&str, u32, u32, i32, u32, u32, u32)] = &[
        (
            "foo=bar; expires=Sun, 06 Nov 1994 08:49:37 GMT",
            6,
            11,
            1994,
            8,
            49,
            37,
        ),
        (
            "foo=bar; expires=Sunday, 06-Nov-94 08:49:37 GMT",
            6,
            11,
            1994,
            8,
            49,
            37,
        ),
        (
            "foo=bar; expires=Sun Nov  6 08:49:37 1994",
            6,
            11,
            1994,
            8,
            49,
            37,
        ),
    ];
}
