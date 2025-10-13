# cookie-monster
A Cookie library for parsing and managing HTTP Cookies.


### Features
* `jiff`, adds support for the [jiff](https://docs.rs/jiff/latest/jiff/) crate.
* `chrono`, adds support for the [chrono](https://docs.rs/chrono/latest/chrono/) crate.
* `time`, adds support for the [time](https://docs.rs/time/latest/time/index.html) crate.
* `percent-encode`, percent-encode/decode cookies.
* `axum`, adds integrations with the [axum](https://docs.rs/axum/latest/axum/) crate.
* `http`, adds integrations with the [http](https://docs.rs/http/latest/http/) crate.


### Install
```toml
# Cargo.toml
[dependencies]
cookie-monster = "0.0.1"

# Integration with the `time` crate
cookie-monster = { version = "0.0.1", features = ["time"] }
# Integration with the`chrono` crate
cookie-monster = { version = "0.0.1", features = ["chrono"] }
# Integration with the `jiff` crate
cookie-monster = { version = "0.0.1", features = ["jiff"] }

# Adds support for percent-encoding/decoding cookies.
cookie-monster = { version = "0.0.1", features = ["percent-encoding"] }

# Integration with the `axum` crate.
cookie-monster = { version = "0.0.1", features = ["axum"] }

# Integration with the `http` crate.
cookie-monster = { version = "0.0.1", features = ["http"] }
```


### License
This project is licensed under the [MIT license].

[MIT license]: https://github.com/joeydewaal/cookie-monster/blob/main/LICENSE
