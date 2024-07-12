use axum::{body::Body, http::{Request, StatusCode}, middleware::Next, response::{Response, Result}};
use tower_cookies::Cookies;

pub async fn mw_require_auth(
    cookies: Cookies,
    request: Request<Body>,
    next: Next,
) -> Result<Response, StatusCode>  {
    match cookies.get(&dotenvy::var("AUTH_TOKEN").unwrap()) {
        Some(_) => Ok(next.run(request).await),
        _ => Err(StatusCode::UNAUTHORIZED),
    }
}
