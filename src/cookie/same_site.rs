#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum SameSite {
    Strict,
    Lax,
    None,
}
