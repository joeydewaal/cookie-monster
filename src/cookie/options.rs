pub(crate) enum Strictness {
    Strict,
    Relaxed,
    Unchecked,
}

pub(crate) struct CookieOptions {
    strictness: Strictness,
    encode: bool,
}

impl CookieOptions {
    #[inline]
    pub fn strict() -> Self {
        Self {
            strictness: Strictness::Strict,
            encode: false,
        }
    }

    #[inline]
    pub fn relaxed() -> Self {
        Self {
            strictness: Strictness::Relaxed,
            encode: false,
        }
    }

    #[inline]
    pub fn unchecked() -> Self {
        Self {
            strictness: Strictness::Unchecked,
            encode: false,
        }
    }

    #[inline]
    pub fn strictness(&self) -> &Strictness {
        &self.strictness
    }

    #[inline]
    fn encode(mut self) -> Self {
        self.encode = true;
        self
    }

    #[inline]
    pub fn is_unchecked(&self) -> bool {
        matches!(self.strictness, Strictness::Unchecked)
    }

    #[inline]
    pub fn is_strict(&self) -> bool {
        matches!(self.strictness, Strictness::Strict)
    }
}
