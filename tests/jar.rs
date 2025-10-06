use cookie_monster::{Cookie, CookieJar};

#[test]
fn basic_jar() {
    let header = "sessionId=abc123def456; userId=12345; theme=dark; lang=en-US; authenticated=true";
    let jar = CookieJar::from_cookie(header);

    let session_id = Cookie::new("sessionId", "abc123def456");
    assert_eq!(jar.get("sessionId"), Some(&session_id));

    let user_id = Cookie::new("userId", "12345");
    assert_eq!(jar.get("userId"), Some(&user_id));

    let session_id = Cookie::new("sessionId", "abc123def456");
    assert_eq!(jar.get("sessionId"), Some(&session_id));

    let session_id = Cookie::new("sessionId", "abc123def456");
    assert_eq!(jar.get("sessionId"), Some(&session_id));
}

#[test]
fn round_trip() {
    let header = "sessionId=abc123def456; userId=12345; theme=dark; lang=en-US; authenticated=true";
    let jar = CookieJar::from_cookie(header);

    let mut set_cookie = jar.set_cookie_headers();

    assert_eq!(set_cookie.next(), None);
}

#[test]
fn round_trip_add() {
    let header = "sessionId=abc123def456; userId=12345; theme=dark; lang=en-US; authenticated=true";
    let mut jar = CookieJar::from_cookie(header);

    jar.add(Cookie::new("sessionId", "abc"));

    let mut set_cookie = jar.set_cookie_headers();

    assert_eq!(set_cookie.next().unwrap().as_deref(), Ok("sessionId=abc"));
}

#[test]
#[cfg(all(not(feature = "time"), not(feature = "chrono"), not(feature = "jiff")))]
fn basic_remove() {
    let header = "theme=dark";
    let mut jar = CookieJar::from_cookie(header);

    jar.remove(Cookie::new("theme", ""));

    let mut set_cookie = jar.set_cookie_headers();
    assert_eq!(
        set_cookie.next().unwrap().as_deref(),
        Ok("theme=; Max-Age=0; Expires=Thu, 01 Jan 1970 00:00:00 GMT")
    );

    // Empty jar
    let mut jar = CookieJar::empty();

    jar.remove(Cookie::new("theme", ""));

    assert_eq!(jar.get("theme"), None);

    let mut set_cookie = jar.set_cookie_headers();
    assert_eq!(
        set_cookie.next().unwrap().as_deref(),
        Ok("theme=; Max-Age=0; Expires=Thu, 01 Jan 1970 00:00:00 GMT")
    );
}

#[test]
fn basic_reassign() {
    let mut jar = CookieJar::empty();

    jar.add_original(Cookie::new("theme", "val"));

    assert!(jar.get("theme").unwrap().value() == "val");

    jar.add(Cookie::new("theme", "val2"));

    assert!(jar.get("theme").unwrap().value() == "val2");

    let mut set_cookie = jar.set_cookie_headers();
    assert_eq!(set_cookie.next().unwrap().as_deref(), Ok("theme=val2"));
}
