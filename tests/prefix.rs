use cookie_monster::{Cookie, CookieJar, CookiePrefix, SameSite};

#[test]
fn host_constructor_builds_valid_cookie() {
    let cookie = Cookie::host("id", "abc").build();

    assert_eq!(cookie.name(), "__Host-id");
    assert!(cookie.is_secure());
    assert_eq!(cookie.path(), Some("/"));
    assert_eq!(cookie.domain(), None);

    assert_eq!(
        cookie.serialize().as_deref(),
        Ok("__Host-id=abc; Path=/; Secure")
    );
}

#[test]
fn secure_constructor_builds_valid_cookie() {
    let cookie = Cookie::secure("id", "abc").build();

    assert_eq!(cookie.name(), "__Secure-id");
    assert!(cookie.is_secure());

    assert_eq!(cookie.serialize().as_deref(), Ok("__Secure-id=abc; Secure"));
}

#[test]
fn host_constructor_stores_host_prefix() {
    let cookie = Cookie::host("id", "abc").build();
    assert_eq!(cookie.prefix(), Some(CookiePrefix::Host));
}

#[test]
fn secure_constructor_stores_secure_prefix() {
    let cookie = Cookie::secure("id", "abc").build();
    assert_eq!(cookie.prefix(), Some(CookiePrefix::Secure));
}

#[test]
fn plain_cookie_has_no_prefix() {
    assert_eq!(Cookie::new("id", "abc").prefix(), None);
}

#[test]
fn parsed_host_cookie_stores_prefix() {
    let cookie = Cookie::parse_cookie("__Host-id=abc").unwrap();
    assert_eq!(cookie.prefix(), Some(CookiePrefix::Host));
}

#[test]
fn parsed_secure_cookie_stores_prefix() {
    let cookie = Cookie::parse_cookie("__Secure-id=abc").unwrap();
    assert_eq!(cookie.prefix(), Some(CookiePrefix::Secure));
}

#[test]
fn parsed_plain_cookie_has_no_prefix() {
    let cookie = Cookie::parse_cookie("id=abc").unwrap();
    assert_eq!(cookie.prefix(), None);
}

#[test]
fn set_name_updates_the_stored_prefix() {
    let mut cookie = Cookie::new("id", "abc");
    assert_eq!(cookie.prefix(), None);

    cookie.set_name("__Host-id");
    assert_eq!(cookie.prefix(), Some(CookiePrefix::Host));

    cookie.set_name("id");
    assert_eq!(cookie.prefix(), None);
}

// The prefix rules are no longer enforced: the constructors only set sensible
// defaults, so users can build non-standard / partial prefixed cookies.

#[test]
fn host_prefix_with_domain_is_allowed() {
    let mut cookie = Cookie::host("id", "abc").build();
    cookie.set_domain("example.com");

    assert_eq!(
        cookie.serialize().as_deref(),
        Ok("__Host-id=abc; Domain=example.com; Path=/; Secure")
    );
}

#[test]
fn host_prefix_with_non_root_path_is_allowed() {
    let mut cookie = Cookie::host("id", "abc").build();
    cookie.set_path("/foo");

    assert_eq!(
        cookie.serialize().as_deref(),
        Ok("__Host-id=abc; Path=/foo; Secure")
    );
}

#[test]
fn host_prefix_without_secure_is_allowed() {
    let mut cookie = Cookie::host("id", "abc").build();
    cookie.set_secure(false);

    assert_eq!(cookie.serialize().as_deref(), Ok("__Host-id=abc; Path=/"));
}

#[test]
fn secure_prefix_without_secure_is_allowed() {
    let mut cookie = Cookie::secure("id", "abc").build();
    cookie.set_secure(false);

    assert_eq!(cookie.serialize().as_deref(), Ok("__Secure-id=abc"));
}

#[cfg(feature = "percent-encode")]
#[test]
fn serialize_encoded_allows_non_standard_prefix() {
    let mut cookie = Cookie::host("id", "abc").build();
    cookie.set_secure(false);

    assert_eq!(
        cookie.serialize_encoded().as_deref(),
        Ok("__Host-id=abc; Path=/")
    );
}

#[test]
fn non_prefixed_cookie_is_unaffected() {
    assert_eq!(
        Cookie::new("id", "abc").serialize().as_deref(),
        Ok("id=abc")
    );
}

#[test]
fn prefix_matching_is_case_sensitive() {
    // `__host-` (lowercase) is not the `__Host-` prefix, so it is not detected.
    let mut cookie = Cookie::new("__host-id", "abc");
    assert_eq!(cookie.prefix(), None);

    cookie.set_domain("example.com");
    cookie.set_path("/foo");

    assert_eq!(
        cookie.serialize().as_deref(),
        Ok("__host-id=abc; Domain=example.com; Path=/foo")
    );
}

#[test]
fn host_prefix_defaults_survive_chaining() {
    let cookie = Cookie::host("id", "abc")
        .http_only()
        .same_site(SameSite::Lax)
        .build();

    assert_eq!(
        cookie.serialize().as_deref(),
        Ok("__Host-id=abc; Path=/; Secure; HttpOnly; SameSite=Lax")
    );
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
