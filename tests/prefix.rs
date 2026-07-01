use cookie_monster::{Cookie, CookieJar, SameSite};

#[test]
fn host_constructor_builds_valid_cookie() {
    let cookie = Cookie::host("id", "abc").build();

    // The prefix is stored as a flavour; the name itself is the logical (unprefixed) name.
    assert_eq!(cookie.name(), "id");
    assert!(cookie.is_secure());
    assert_eq!(cookie.path(), Some("/"));
    assert_eq!(cookie.domain(), None);

    // The prefix is re-applied on serialization.
    assert_eq!(
        cookie.serialize().as_deref(),
        Ok("__Host-id=abc; Path=/; Secure")
    );
}

#[test]
fn secure_constructor_builds_valid_cookie() {
    let cookie = Cookie::secure("id", "abc").build();

    assert_eq!(cookie.name(), "id");
    assert!(cookie.is_secure());

    assert_eq!(cookie.serialize().as_deref(), Ok("__Secure-id=abc; Secure"));
}

#[test]
fn new_does_not_infer_a_prefix() {
    // `Cookie::new` treats the name literally: no prefix flavour is inferred, so the
    // `__Host-` text stays part of the name and no prefix defaults are applied.
    let cookie = Cookie::new("__Host-id", "abc");

    assert_eq!(cookie.name(), "__Host-id");
    assert!(!cookie.is_secure());
    assert_eq!(cookie.serialize().as_deref(), Ok("__Host-id=abc"));
}

#[test]
fn set_name_does_not_infer_a_prefix() {
    let mut cookie = Cookie::new("id", "abc");
    cookie.set_name("__Host-id");

    assert_eq!(cookie.name(), "__Host-id");
    assert_eq!(cookie.serialize().as_deref(), Ok("__Host-id=abc"));
}

#[test]
fn parse_strips_host_prefix() {
    let cookie = Cookie::parse_cookie("__Host-id=abc").unwrap();

    // The prefix is stripped from the name and remembered as a flavour.
    assert_eq!(cookie.name(), "id");
    assert_eq!(cookie.value(), "abc");
    // Serialization re-applies the prefix.
    assert_eq!(cookie.serialize().as_deref(), Ok("__Host-id=abc"));
}

#[test]
fn parse_strips_secure_prefix() {
    let cookie = Cookie::parse_cookie("__Secure-sid=xyz").unwrap();

    assert_eq!(cookie.name(), "sid");
    assert_eq!(cookie.serialize().as_deref(), Ok("__Secure-sid=xyz"));
}

#[test]
fn parse_leaves_plain_name_untouched() {
    let cookie = Cookie::parse_cookie("id=abc").unwrap();
    assert_eq!(cookie.name(), "id");
}

#[test]
fn parse_prefix_matching_is_case_sensitive() {
    // `__host-` (lowercase) is not the `__Host-` prefix, so nothing is stripped.
    let cookie = Cookie::parse_cookie("__host-id=abc").unwrap();
    assert_eq!(cookie.name(), "__host-id");
}

// The prefix rules are not enforced: the constructors only set sensible defaults, so
// callers can build non-standard / partial prefixed cookies.

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
fn serialize_encoded_reapplies_prefix() {
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

    // The prefix was stripped at parse time, so the cookie is keyed by its logical name.
    assert_eq!(jar.get("id").map(|c| c.value()), Some("abc"));
    assert_eq!(jar.get("id").map(|c| c.name()), Some("id"));
}

#[test]
fn jar_finds_secure_cookie_by_logical_name() {
    let jar = CookieJar::from_cookie("__Secure-sid=xyz");

    assert_eq!(jar.get("sid").map(|c| c.value()), Some("xyz"));
}
