use axum::{middleware, response::Html, routing::get, Router};
use serde::Deserialize;
use templates::LoginPage;

use crate::{middlewares::use_layout, view};

mod templates;

pub fn routes() -> Router {
    return Router::new().route("/login", get(login));
}

async fn login() -> Html<String> {
    view::<LoginPage>(LoginPage {errors:None}).await
}

#[derive(Debug, Deserialize)]
struct LoginPayload {
    username: String,
    pswd: String,
}