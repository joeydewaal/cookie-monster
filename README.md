# cookie-monster
A Cookie library for parsing and managing HTTP Cookies.

Support for:
* The `time`,`chrono` and `jiff` crates.
* `axum`; Extractors and IntoResponse.
* Percent encoding/decoding

# Usage
```toml
[dependencies]
cookie-monster = "0.1"

# Adds support for the `time` crate
cookie-monster = { version = "0.1", features = ["time"] }
# Adds support for the `chrono` crate
cookie-monster = { version = "0.1", features = ["chrono"] }
# Adds support for the `jiff` crate
cookie-monster = { version = "0.1", features = ["jiff"] }

# Adds support for percent-encoding and percent-decoding cookies.
cookie-monster = { version = "0.1", features = ["percent-encoding"] }

# Adds integration with the `axum` crate.
cookie-monster = { version = "0.1", features = ["axum"] }
```

# Features
* `time`, adds support for the [time](https://docs.rs/time/latest/time/index.html) crate.

* `chrono`, adds support for the [chrono](https://docs.rs/chrono/latest/chrono/) crate.

* `jiff`, adds support for the [jiff](https://docs.rs/jiff/latest/jiff/) crate.

* `percent-encode`,

* `axum`, adds integrations with the [axum](https://docs.rs/axum/latest/axum/) crate.
This implicitly enables the `percent-encode` feature flag.

# License
