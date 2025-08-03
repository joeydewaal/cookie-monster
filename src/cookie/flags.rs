use super::same_site::SameSite;

#[repr(transparent)]
#[derive(Default)]
pub struct BitFlags(u8);

// 0 -> secure
// 1 -> http_only
// 2 -> partitioned
// 3
// 4 (TODO: we can be more efficient here by only using 2 bits)
// 5 -> 1 == Some(SameSite::Strict) | 0
// 6 -> 1 == Some(SameSite::Lax)    | 0 -> None
// 7 -> 1 == Some(SameSite::None)   | 0

impl BitFlags {
    #[inline]
    pub fn empty() -> Self {
        Self::default()
    }

    #[inline]
    pub fn get<const INDEX: u8>(&self) -> bool {
        (self.0 & 1 << INDEX) != 0
    }

    #[inline]
    pub fn set<const INDEX: u8>(&mut self, value: bool) {
        if value {
            self.0 |= 1u8 << INDEX
        } else {
            self.0 &= !(1u8 << INDEX)
        }
    }

    #[inline]
    pub fn secure(&self) -> bool {
        self.get::<0>()
    }

    #[inline]
    pub fn set_secure(&mut self, secure: bool) {
        self.set::<0>(secure);
    }

    #[inline]
    pub fn http_only(&self) -> bool {
        self.get::<1>()
    }

    #[inline]
    pub fn set_http_only(&mut self, http_only: bool) {
        self.set::<1>(http_only);
    }

    #[inline]
    pub fn partitioned(&self) -> bool {
        self.get::<2>()
    }

    #[inline]
    pub fn set_partitioned(&mut self, partitioned: bool) {
        self.set::<2>(partitioned);
    }

    pub fn same_site(&self) -> Option<SameSite> {
        match self.0 & 0b1110_0000 {
            0b0000_0000 => None,
            0b0010_0000 => Some(SameSite::Strict),
            0b0100_0000 => Some(SameSite::Lax),
            0b1000_0000 => Some(SameSite::None),
            _ => unreachable!(),
        }
    }

    pub fn set_same_site(&mut self, same_site: Option<SameSite>) {
        // Clear out the first bits
        self.0 &= !0b1110_0000;
        match same_site {
            Some(SameSite::Strict) => self.set::<5>(true),
            Some(SameSite::Lax) => self.set::<6>(true),
            Some(SameSite::None) => self.set::<7>(true),
            None => {
                // Keep the first three bits 0
            }
        }
    }
}

#[cfg(test)]
mod test {
    use crate::cookie::same_site::SameSite;

    use super::BitFlags;

    #[test]
    fn test_flags() {
        let mut flags = BitFlags::empty();
        assert!(flags.secure() == false);

        flags.set_secure(true);
        assert!(flags.secure() == true);
        flags.set_secure(false);
        assert!(flags.secure() == false);

        assert!(flags.http_only() == false);

        flags.set_http_only(true);
        assert!(flags.http_only() == true);

        flags.set_http_only(false);
        assert!(flags.http_only() == false);

        assert!(flags.same_site() == None);

        flags.set_same_site(Some(SameSite::None));
        assert!(flags.same_site() == Some(SameSite::None));

        flags.set_same_site(Some(SameSite::Lax));
        assert!(flags.same_site() == Some(SameSite::Lax));

        flags.set_same_site(Some(SameSite::Strict));
        assert!(flags.same_site() == Some(SameSite::Strict));
    }
}
