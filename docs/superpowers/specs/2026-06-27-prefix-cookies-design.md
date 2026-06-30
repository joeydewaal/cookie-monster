# Design: `__Host-` / `__Secure-` prefix cookies

**Date:** 2026-06-27
**Crate:** cookie-monster
**Status:** Approved design, pending implementation plan

## Summary

Add support for the [RFC 6265bis §4.1.3](https://datatracker.ietf.org/doc/html/draft-ietf-httpbis-rfc6265bis#section-4.1.3)
cookie name prefixes `__Host-` and `__Secure-`. The feature provides two ergonomic,
symmetrically-named constructors — `Cookie::host` and `Cookie::secure` — that build
correctly-configured prefixed cookies, plus an internal validation step that runs
during serialization and reports rule violations as detailed errors. There is no
separate public validation method, and the behavior of serializing **non-prefixed**
cookies is unchanged.

To free the `secure` name for the constructor (and to keep the boolean attribute
getters parallel), this change also renames the three `bool` getters
`secure`/`http_only`/`partitioned` to `is_secure`/`is_http_only`/`is_partitioned`.
This is a breaking change, acceptable at the crate's pre-1.0 (v0.2.x) stage. See
[Getter rename](#getter-rename-breaking).

## Background: the prefix rules

A cookie's name prefix imposes constraints that user-agents enforce. cookie-monster
mirrors the attribute-level subset of these rules (the HTTPS-transport requirement
is out of scope — a server-side serializer cannot observe transport):

| Prefix       | Requires `Secure` | `Domain` allowed | `Path` requirement |
|--------------|-------------------|------------------|--------------------|
| `__Secure-`  | yes               | yes              | none               |
| `__Host-`    | yes               | **no**           | must be exactly `/` |

Prefix matching is **case-sensitive** per the spec: `__host-` is not a prefix.

## Goals

- Make it easy to construct a valid `__Host-` / `__Secure-` cookie.
- Provide an explicit, detailed validity check for prefixed cookies.
- Add zero new dependencies and keep MSRV at 1.85.

## Non-goals

- No transport (HTTPS) validation.
- No changes to the request-`Cookie`-header parser.
- No new public validation method — validation is internal to serialization.

## API

### Constructors (on `Cookie`, returning `CookieBuilder`)

Names are passed **unprefixed**; the constructor prepends the prefix. Both return a
`CookieBuilder` so all existing chaining (`.http_only()`, `.same_site(..)`,
`.max_age(..)`, `.value(..)`, …) continues to work. Returning a builder is consistent
with the existing `Cookie::named`, which also returns a `CookieBuilder` (only
`Cookie::new` returns a `Cookie`).

The two constructors are named after the prefixes themselves — `host` and `secure` —
mirroring the wider Rust ecosystem (the `cookie` crate exposes `prefix::Host` /
`prefix::Secure`). The `secure` constructor is only available because the `secure`
getter is renamed to `is_secure` in this same change; see
[Getter rename](#getter-rename-breaking).

```rust
/// Builds a `__Host-` prefixed cookie: prepends `__Host-`, sets `Secure`, sets `Path=/`.
pub fn host<N, V>(name: N, value: V) -> CookieBuilder
where N: Into<Cow<'static, str>>, V: Into<Cow<'static, str>>;

/// Builds a `__Secure-` prefixed cookie: prepends `__Secure-`, sets `Secure`.
pub fn secure<N, V>(name: N, value: V) -> CookieBuilder
where N: Into<Cow<'static, str>>, V: Into<Cow<'static, str>>;
```

Note three distinct `secure`-flavored items remain, on different types/forms, so there
is no conflict: the `Cookie::secure(name, value)` constructor (associated fn), the
`CookieBuilder::secure()` flag-setter (builder chain, unchanged), and the
`Cookie::set_secure(bool)` setter (unchanged).

Example:

```rust
Cookie::host("id", "abc")
    .http_only()
    .same_site(SameSite::Lax)
    .build();
// __Host-id=abc; Path=/; Secure; HttpOnly; SameSite=Lax

Cookie::secure("id", "abc").build();
// __Secure-id=abc; Secure
```

### Validation: `Cookie::check_prefix` (internal)

```rust
/// Validates the cookie against the rules implied by its name prefix.
/// Returns `Ok(())` when the cookie has no recognized prefix.
pub(crate) fn check_prefix(&self) -> crate::Result<()>;
```

Behavior:

- Detect the prefix from `self.name()` (case-sensitive).
- `__Host-`: require `secure == true`, `domain.is_none()`, and `path == Some("/")`.
- `__Secure-`: require `secure == true`.
- No prefix: `Ok(())`.

`check_prefix` is **not public**. It is invoked from `serialize_inner` (see
[Serialize integration](#serialize-integration)), so a prefixed cookie that violates
its rules — e.g. after a later `set_domain(..)` / `set_path(..)` / `set_secure(false)`
— fails to serialize with the corresponding detailed `Error`.

### Serialize integration

`serialize()` and `serialize_encoded()` both delegate to `Cookie::serialize_inner`.
`check_prefix` is called once at the top of `serialize_inner`, before the buffer is
built, so both the plain and percent-encoded paths enforce it. For a non-prefixed
cookie `check_prefix` returns `Ok(())`, so serialization of arbitrary names is
behaviorally unchanged.

Consequence for the integrations: `CookieJar::set_cookie_headers`, the `http`
`write_cookies`, and the axum `IntoResponseParts` impls already call the `serialize`
variants and **silently drop** cookies whose serialization errors (`.ok()`). With this
change, an invalid prefixed cookie is therefore dropped from the response rather than
emitted — i.e. an invalid `__Host-`/`__Secure-` cookie is never sent to the
user-agent. This is the desired fail-safe and requires no code change in those
integrations.

## Getter rename (breaking)

To free the `secure` name for the new constructor, the `Cookie::secure` **getter** is
renamed to `Cookie::is_secure`. For consistency, the other two boolean attribute
getters are renamed in the same change so all three share the `is_` prefix:

| Old (`-> bool` getter) | New           |
|------------------------|---------------|
| `Cookie::secure`       | `is_secure`   |
| `Cookie::http_only`    | `is_http_only`|
| `Cookie::partitioned`  | `is_partitioned`|

**Unchanged** (deliberately, to keep the rename tightly scoped):

- `CookieBuilder::secure` / `http_only` / `partitioned` — these are flag-setting
  builder methods, not getters, and live on a different type. They keep their names,
  so existing chains like `Cookie::build(..).secure().http_only()` are unaffected.
- The `set_secure` / `set_http_only` / `set_partitioned` setters keep their names
  (`is_secure` / `set_secure` is the idiomatic getter/setter pairing).
- `Expires::expires_is_set` is already in `_is_set` form and reads naturally; it is
  **out of scope** for this rename.

**Internal call sites to update** (all within the crate, mechanical):

- `src/cookie/serialize.rs` — the `secure()` / `partitioned()` / `http_only()` reads
  in `serialize_inner`.
- The `Debug` impl and the `PartialEq` impl in `src/cookie/mod.rs`.
- Doctest `assert!(cookie.secure())` / `.http_only()` / `.partitioned()` lines in
  `src/cookie/mod.rs` and `src/cookie/builder.rs` (the *getter* asserts only — the
  builder-method calls in the same examples stay).

This is the only breaking part of the change and should be called out in the
changelog / release notes for the next version bump.

## Error variants

Added to the existing `#[non_exhaustive]` `Error` enum in `src/error.rs` (additions
are non-breaking), each with a `Display` arm:

- `HostPrefixHasDomain` — a `__Host-` cookie has a `Domain` attribute.
- `HostPrefixBadPath` — a `__Host-` cookie's `Path` is not exactly `/`.
- `HostPrefixNotSecure` — a `__Host-` cookie lacks `Secure`.
- `SecurePrefixNotSecure` — a `__Secure-` cookie lacks `Secure`.

**Resolved decision A:** `HostPrefixNotSecure` and `SecurePrefixNotSecure` are kept
as separate variants (rather than a single shared `PrefixNotSecure`) so the error
message names the exact prefix involved.

## Module layout

- New file `src/cookie/prefix.rs`, declared `mod prefix;` in `src/cookie/mod.rs`,
  containing:
  - Prefix constants: `const HOST_PREFIX: &str = "__Host-";`,
    `const SECURE_PREFIX: &str = "__Secure-";`.
  - An **internal** `enum CookiePrefix { Host, Secure }` plus
    `fn detect(name: &str) -> Option<CookiePrefix>`.
  - The `host` / `secure` constructors (`impl Cookie`).
  - `pub(crate) check_prefix` (`impl Cookie`).
- A `self.check_prefix()?` call near the top of `serialize_inner` in
  `src/cookie/serialize.rs`.
- New `Error` variants + `Display` arms in `src/error.rs`.
- The boolean getter renames in `src/cookie/mod.rs` and their internal call sites
  (see [Getter rename](#getter-rename-breaking)).

**Resolved decision B:** `CookiePrefix` and `detect` are `pub(crate)` (internal only).
A public `Cookie::prefix() -> Option<CookiePrefix>` detection method is intentionally
omitted to keep the public surface small; it can be added later if a use case appears.

## Parse side — explicitly unchanged

Request `Cookie:` headers carry only `name=value` pairs, so a received
`__Host-id=abc` parses with name `__Host-id` and no attributes. Validation runs only
during `serialize`, which is the **outgoing** (Set-Cookie) path — parsed request
cookies are never validated. The parser, `CookieJar`, and the axum/http integrations
need no code changes; the jar/axum/http paths inherit the fail-safe drop described in
[Serialize integration](#serialize-integration).

## Testing

New test file `tests/prefix.rs`, registered as a `[[test]]` in `Cargo.toml`. Because
`check_prefix` is internal, validity is asserted through the public `serialize()` /
`serialize_encoded()` results. Cases:

- `Cookie::host("id", "abc").build()` → name `__Host-id`, `is_secure`, `path == "/"`,
  no domain; `serialize()` == `Ok("__Host-id=abc; Path=/; Secure")`.
- `Cookie::secure("id", "abc").build()` → `serialize()` == `Ok("__Secure-id=abc; Secure")`.
- `__Host-` + `set_domain(..)` → `serialize()` is `Err(HostPrefixHasDomain)`.
- `__Host-` + `set_path("/foo")` → `serialize()` is `Err(HostPrefixBadPath)`.
- `__Host-` + `set_secure(false)` → `serialize()` is `Err(HostPrefixNotSecure)`.
- `__Secure-` + `set_secure(false)` → `serialize()` is `Err(SecurePrefixNotSecure)`.
- Non-prefixed cookie → `serialize()` is unaffected (existing behavior).
- Case sensitivity: `__host-id` (lowercase) is not detected as a prefix → serializes
  with no added constraints.
- Chaining `.http_only().same_site(..)` preserves the prefix invariants.
- `serialize_encoded()` enforces the same rules (one violation case is enough).

The existing `tests/`, doctests, `Debug`/`PartialEq`, and `serialize.rs` are updated to
the renamed `is_secure` / `is_http_only` / `is_partitioned` getters; the whole suite
plus `cargo test --doc` must pass after the rename.

## Docs

- Doctest examples on `host` and `secure`.
- Document on both constructors that an invalid prefixed cookie (e.g. mutated to add a
  `Domain`) fails to serialize with a detailed `Error`.
- Short note in `src/lib.rs` crate docs (and optionally README) describing the prefix
  helpers.
- Changelog entry calling out the breaking getter renames
  (`secure`/`http_only`/`partitioned` → `is_*`).

## Dependencies / MSRV

No new dependencies; pure `std`. MSRV remains 1.85.
