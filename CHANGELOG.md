# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased]

### Added

- `Cookie::host` and `Cookie::secure` constructors for building `__Host-` / `__Secure-`
  prefixed cookies ([RFC 6265bis §4.1.3](https://datatracker.ietf.org/doc/html/draft-ietf-httpbis-rfc6265bis#section-4.1.3)).
  They set the attributes the prefix requires as defaults (which can be overridden to build
  non-standard cookies) and apply the prefix to the name on serialization.
- Parsing a `__Host-` / `__Secure-` cookie now strips the prefix from the cookie name and
  remembers the prefix flavour, re-applying it on serialization. As a result such cookies are
  looked up in a `CookieJar` by their logical (unprefixed) name.

### Changed

- **Breaking:** renamed the boolean getters `Cookie::secure`, `Cookie::http_only` and
  `Cookie::partitioned` to `Cookie::is_secure`, `Cookie::is_http_only` and
  `Cookie::is_partitioned`. The `CookieBuilder` flag-setters (`.secure()` / `.http_only()` /
  `.partitioned()`) and the `set_*` setters keep their names.

## [0.2.1](https://github.com/joeydewaal/cookie-monster/compare/v0.2.0...v0.2.1) - 2026-03-06

### Other

- Add setters ([#17](https://github.com/joeydewaal/cookie-monster/pull/17))

## [0.1.1](https://github.com/joeydewaal/cookie-monster/compare/v0.1.0...v0.1.1) - 2026-02-06

### Other

- Update axum support and add getters ([#16](https://github.com/joeydewaal/cookie-monster/pull/16))
- Update README
