use axum::{body::Body, http::Request, http::StatusCode, middleware::Next, response::Response};
use axum_extra::{
    headers::{authorization::Bearer, Authorization},
    TypedHeader,
};

pub async fn auth(
    TypedHeader(_auth): TypedHeader<Authorization<Bearer>>,
    request: Request<Body>,
    next: Next,
) -> Result<Response, StatusCode> {
    let response = next.run(request).await;
    Ok(response)
}
