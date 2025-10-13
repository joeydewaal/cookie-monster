use std::convert::Infallible;

use axum_core::{
    extract::{FromRequest, Request},
    response::{IntoResponse, IntoResponseParts, Response, ResponseParts},
};

use crate::CookieJar;

impl<S> FromRequest<S> for CookieJar
where
    S: Send + Sync,
{
    type Rejection = Infallible;

    async fn from_request(req: Request, _: &S) -> Result<Self, Self::Rejection> {
        Ok(CookieJar::from_headers(req.headers()))
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
