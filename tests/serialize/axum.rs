use axum::{Router, response::IntoResponse, routing::get};
use cookie_monster::{Cookie, CookieJar};

#[test]
fn extract_axum() {
    // This should just compile.
    let _ = Router::<()>::new().route("/", get(handler));
}

static COOKIE_NAME: &str = "session";

async fn handler(mut jar: CookieJar) -> impl IntoResponse {
    if let Some(cookie) = jar.get(COOKIE_NAME) {
        // Remove cookie
        println!("Removing cookie {cookie:?}");
        jar.remove(Cookie::named(COOKIE_NAME));
    } else {
        // Set cookie.
        let cookie = Cookie::new(COOKIE_NAME, "hello,world");
        println!("Setting cookie {cookie:?}");
        jar.add(cookie);
    }

    jar
}
