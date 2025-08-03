use super::Cookie;

#[cfg(feature = "time")]
pub mod dep_time;

#[cfg(feature = "chrono")]
pub mod dep_chrono;

#[cfg(feature = "jiff")]
pub mod dep_jiff;

#[cfg(any(feature = "chrono", feature = "jiff"))]
pub mod formats {
    // Sun, 06 Nov 1994 08:49:37 GMT (RFC)
    pub static FMT1: &'static str = "%a, %d %b %Y %H:%M:%S GMT";
    // Sunday, 06-Nov-94 08:49:37 GMT (RFC)
    pub static FMT2: &'static str = "%A, %d-%b-%y %H:%M:%S GMT";
    // Sun Nov  6 08:49:37 1994 (RFC)
    pub static FMT3: &'static str = "%a %b %e %H:%M:%S %Y";
    // Thu 10-Sep-20 20:00:00 GMT
    pub static FMT4: &'static str = "%a, %d-%b-%y %H:%M:%S GMT";
    // Thu 10-Sep-2069 20:00:00 GMT
    pub static FMT5: &'static str = "%a, %d-%b-%Y %H:%M:%S GMT";
}

pub struct Expires(ExpiresInner);

pub(crate) enum ExpiresInner {
    Remove,
    Expires {
        #[cfg(feature = "time")]
        time: Option<time::OffsetDateTime>,
        #[cfg(feature = "chrono")]
        chrono: Option<chrono::DateTime<chrono::Utc>>,
        #[cfg(feature = "jiff")]
        jiff: Option<jiff::Timestamp>,
    },
}

impl Expires {
    pub fn in_the_past() -> Self {
        Self(ExpiresInner::Remove)
    }
}

pub fn parse_expires(value: &str) -> Option<Expires> {
    Some(Expires(ExpiresInner::Expires {
        #[cfg(feature = "time")]
        time: dep_time::parse_expires_time(value),
        #[cfg(feature = "chrono")]
        chrono: dep_chrono::parse_expires(value),
        #[cfg(feature = "jiff")]
        jiff: dep_jiff::parse_expires(value),
    }))
}

impl Cookie {
    pub fn serialize_expire(&self, buf: &mut String) -> crate::Result<()> {
        #[cfg(feature = "time")]
        {
            if self.serialize_expires_time(buf)? {
                return Ok(());
            }
        }

        #[cfg(feature = "chrono")]
        {
            if self.serialize_expires_chrono(buf)? {
                return Ok(());
            };
        }

        #[cfg(feature = "jiff")]
        {
            self.serialize_expires_jiff(buf)?;
            return Ok(());
        }

        #[allow(unreachable_code)]
        Ok(())
    }
}

#[cfg(test)]
pub mod test_cases {
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
