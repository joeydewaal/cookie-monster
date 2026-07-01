# cookie-monster
[![CI/CD](https://github.com/joeydewaal/cookie-monster/actions/workflows/ci.yml/badge.svg)](https://github.com/joeydewaal/cookie-monster/actions/workflows/ci.yml)
[![Current Crates.io Version](https://img.shields.io/crates/v/cookie-monster.svg)](https://crates.io/crates/cookie-monster)
[![Documentation](https://docs.rs/cookie-monster/badge.svg)](https://docs.rs/cookie-monster)

A Cookie library for parsing and managing HTTP Cookies.

### Features
* `jiff`, adds support for the [jiff](https://docs.rs/jiff/latest/jiff/) crate.
* `chrono`, adds support for the [chrono](https://docs.rs/chrono/latest/chrono/) crate.
* `time`, adds support for the [time](https://docs.rs/time/latest/time/index.html) crate.
* `percent-encode`, percent-encode/decode cookies.
* `axum`, adds integration with the [axum](https://docs.rs/axum/latest/axum/) crate.
* `http`, adds integration with the [http](https://docs.rs/http/latest/http/) crate.


### Install
```toml
# Cargo.toml
[dependencies]
cookie-monster = "0.1"

# Integration with the `time` crate
cookie-monster = { version = "0.1", features = ["time"] }
# Integration with the`chrono` crate
cookie-monster = { version = "0.1", features = ["chrono"] }
# Integration with the `jiff` crate
cookie-monster = { version = "0.1", features = ["jiff"] }

# Adds support for percent-encoding/decoding cookies.
cookie-monster = { version = "0.1", features = ["percent-encoding"] }

# Integration with the `axum` crate.
cookie-monster = { version = "0.1", features = ["axum"] }

# Integration with the `http` crate.
cookie-monster = { version = "0.1", features = ["http"] }
```

### Axum example

```rust
use axum::response::IntoResponse;
use cookie_monster::{Cookie, CookieJar, SameSite};

static COOKIE_NAME: &str = "session";

async fn handler(mut jar: CookieJar) -> impl IntoResponse {
    if let Some(cookie) = jar.get(COOKIE_NAME) {
        // Remove cookie
        println!("Removing cookie {cookie:?}");
        jar.remove(Cookie::named(COOKIE_NAME));
    } else {
        // Set cookie.
        let cookie = Cookie::build(COOKIE_NAME, "hello, world")
        .http_only()
        .same_site(SameSite::Strict);

        println!("Setting cookie {cookie:?}");
        jar.add(cookie);
    }
    // Return the jar so the cookies are updated
   jar
}
```

### Prefix cookies
`Cookie::host` and `Cookie::secure` build cookies using the `__Host-` and `__Secure-` name
prefixes (RFC 6265bis §4.1.3). They set the attributes the prefix requires as defaults (which
you can override to build non-standard cookies) and apply the prefix to the name on
serialization. Parsing strips a recognized prefix from the name, so a prefixed cookie is looked
up in a `CookieJar` by its logical (unprefixed) name.

```rust
use cookie_monster::Cookie;

let cookie = Cookie::host("id", "abc").build();
assert_eq!(cookie.name(), "id");
assert_eq!(cookie.serialize().as_deref(), Ok("__Host-id=abc; Path=/; Secure"));
```

### Minimum Supported Rust Version (MSRV)
The cookie-monster crate has rust version 1.85 as MSRV.

### Honorable mention
This crate takes a lot of inspiration from the [cookie](https://crates.io/crates/cookie) crate.


### License
This project is licensed under the [MIT license].

[MIT license]: https://github.com/joeydewaal/cookie-monster/blob/main/LICENSE
