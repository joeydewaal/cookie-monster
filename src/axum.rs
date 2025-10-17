use std::convert::Infallible;

use axum_core::{
    extract::FromRequestParts,
    response::{IntoResponse, IntoResponseParts, Response, ResponseParts},
};
use http::request::Parts;

use crate::CookieJar;

impl<S> FromRequestParts<S> for CookieJar
where
    S: Send + Sync,
{
    type Rejection = Infallible;

    async fn from_request_parts(parts: &mut Parts, _: &S) -> Result<Self, Self::Rejection> {
        Ok(CookieJar::from_headers(&parts.headers))
    }
}

impl IntoResponseParts for CookieJar {
    type Error = Infallible;

    fn into_response_parts(self, mut res: ResponseParts) -> Result<ResponseParts, Self::Error> {
        self.write_cookies(res.headers_mut());
        Ok(res)
    }
}

impl IntoResponse for CookieJar {
    fn into_response(self) -> Response {
        (self, ()).into_response()
    }
}
