use askama::Template;
use axum::{
    body::Body,
    http::{Request, StatusCode},
    middleware::Next,
    response::{IntoResponse, Response, Result},
};
use tower_cookies::{Cookie, Cookies};

use crate::HelloTemplate;

pub async fn mw_require_auth(
    cookies: Cookies,
    req: Request<Body>,
    next: Next,
) -> Result<Response, StatusCode> {
    match cookies.get(&dotenvy::var("AUTH_TOKEN").unwrap()) {
        Some(token) => match auth_validate(token) {
            Ok(_) => return Ok(next.run(req).await),
            Err(e) => return Err(e),
        },
        _ => Err(StatusCode::UNAUTHORIZED),
    }
}

pub fn auth_validate(token: Cookie) -> Result<(), StatusCode> {
    let _ = token.value();
    return Ok(());
}

pub async fn use_layout(req: Request<Body>, next: Next) -> impl IntoResponse {
    if req.headers().get("HX-REQUEST").is_some() {
        return next.run(req).await;
    } else {
        return Response::builder()
            .body(Body::from(
                HelloTemplate {
                    app_title: &dotenvy::var("APP_NAME").unwrap(),
                    content: &format!(r#"<div hx-get="{}" hx-trigger="load" hx-swap="innerHTML" hx-target="main"></div>"#, req.uri()),
                }
                .render()
                .unwrap(),
            ))
            .unwrap();
    }
}