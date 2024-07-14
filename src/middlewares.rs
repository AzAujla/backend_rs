use axum::{
    body::Body,
    http::{Request, StatusCode},
    middleware::Next,
    response::{Html, IntoResponse, Response, Result},
};
use diesel::*;
use tower_cookies::{Cookie, Cookies};
use askama::Template;

use crate::{auth::cookies::revoke_old_sessions, database::establish_connection, helpers::htmx_redirect, models::Session, schema::sessions, templates::HelloTemplate};

pub async fn mw_require_auth(
    cookies: Cookies,
    req: Request<Body>,
    next: Next,
) -> Result<Response, Html<String>> {
    match cookies.get(&dotenvy::var("AUTH_TOKEN").unwrap()) {
        Some(token) => match auth_validate(token) {
            Ok(_) => return Ok(next.run(req).await),
            Err(_) => Err(htmx_redirect("/auth/login", "get")),
        },
        _ => Err(htmx_redirect("/auth/login", "get")),
    }
}

pub fn auth_validate(token: Cookie) -> Result<(), StatusCode> {
    let token = token.value();
    let conn = &mut establish_connection();
    revoke_old_sessions(conn);

    let session: Option<Session> = sessions::table.filter(sessions::token.eq(token)).select(Session::as_select()).first(conn).optional().unwrap();
    match session {
        Some(_) => Ok(()),
        None => Err(StatusCode::UNAUTHORIZED)
    }
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