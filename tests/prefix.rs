use cookie_monster::{Cookie, CookieJar, Error, SameSite};

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

#[test]
fn jar_finds_host_cookie_by_logical_name() {
    let jar = CookieJar::from_cookie("__Host-id=abc");

    // The unprefixed (logical) name resolves to the parsed cookie.
    assert_eq!(jar.get("id").map(|c| c.value()), Some("abc"));
    // The prefix is preserved on the stored cookie (it is the trust signal).
    assert_eq!(jar.get("id").map(|c| c.name()), Some("__Host-id"));
    // The full name still resolves too.
    assert_eq!(jar.get("__Host-id").map(|c| c.value()), Some("abc"));
}

#[test]
fn jar_finds_secure_cookie_by_logical_name() {
    let jar = CookieJar::from_cookie("__Secure-sid=xyz");

    assert_eq!(jar.get("sid").map(|c| c.value()), Some("xyz"));
}

#[test]
fn non_prefixed_cookie_cannot_shadow_prefixed() {
    // A plain `id` cookie must never be returned in place of the legit `__Host-id`,
    // regardless of header order.
    let jar = CookieJar::from_cookie("__Host-id=good; id=evil");
    assert_eq!(jar.get("id").map(|c| c.value()), Some("good"));

    let jar = CookieJar::from_cookie("id=evil; __Host-id=good");
    assert_eq!(jar.get("id").map(|c| c.value()), Some("good"));
}

#[test]
fn host_prefix_preferred_over_secure_for_logical_name() {
    let jar = CookieJar::from_cookie("__Secure-id=sec; __Host-id=host");

    assert_eq!(jar.get("id").map(|c| c.value()), Some("host"));
}
