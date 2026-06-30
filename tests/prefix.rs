use cookie_monster::{Cookie, Error, SameSite};

#[test]
fn host_constructor_builds_valid_cookie() {
    let cookie = Cookie::host("id", "abc").build();

    assert_eq!(cookie.name(), "__Host-id");
    assert!(cookie.is_secure());
    assert_eq!(cookie.path(), Some("/"));
    assert_eq!(cookie.domain(), None);

    assert_eq!(cookie.serialize().as_deref(), Ok("__Host-id=abc; Path=/; Secure"));
}

#[test]
fn secure_constructor_builds_valid_cookie() {
    let cookie = Cookie::secure("id", "abc").build();

    assert_eq!(cookie.name(), "__Secure-id");
    assert!(cookie.is_secure());

    assert_eq!(cookie.serialize().as_deref(), Ok("__Secure-id=abc; Secure"));
}

#[test]
fn host_prefix_with_domain_fails() {
    let mut cookie = Cookie::host("id", "abc").build();
    cookie.set_domain("example.com");

    assert_eq!(cookie.serialize(), Err(Error::HostPrefixHasDomain));
}

#[test]
fn host_prefix_with_non_root_path_fails() {
    let mut cookie = Cookie::host("id", "abc").build();
    cookie.set_path("/foo");

    assert_eq!(cookie.serialize(), Err(Error::HostPrefixBadPath));
}

#[test]
fn host_prefix_without_secure_fails() {
    let mut cookie = Cookie::host("id", "abc").build();
    cookie.set_secure(false);

    assert_eq!(cookie.serialize(), Err(Error::HostPrefixNotSecure));
}

#[test]
fn secure_prefix_without_secure_fails() {
    let mut cookie = Cookie::secure("id", "abc").build();
    cookie.set_secure(false);

    assert_eq!(cookie.serialize(), Err(Error::SecurePrefixNotSecure));
}

#[test]
fn non_prefixed_cookie_is_unaffected() {
    assert_eq!(Cookie::new("id", "abc").serialize().as_deref(), Ok("id=abc"));
}

#[test]
fn prefix_matching_is_case_sensitive() {
    // `__host-` (lowercase) is not the `__Host-` prefix, so no constraints apply.
    let mut cookie = Cookie::new("__host-id", "abc");
    cookie.set_domain("example.com");
    cookie.set_path("/foo");

    assert_eq!(
        cookie.serialize().as_deref(),
        Ok("__host-id=abc; Domain=example.com; Path=/foo")
    );
}

#[test]
fn host_prefix_invariants_survive_chaining() {
    let cookie = Cookie::host("id", "abc")
        .http_only()
        .same_site(SameSite::Lax)
        .build();

    assert_eq!(
        cookie.serialize().as_deref(),
        Ok("__Host-id=abc; Path=/; Secure; HttpOnly; SameSite=Lax")
    );
}

#[cfg(feature = "percent-encode")]
#[test]
fn serialize_encoded_enforces_prefix_rules() {
    let mut cookie = Cookie::host("id", "abc").build();
    cookie.set_secure(false);

    assert_eq!(cookie.serialize_encoded(), Err(Error::HostPrefixNotSecure));
}
