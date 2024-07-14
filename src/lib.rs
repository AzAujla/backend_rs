#![allow(dead_code)]

use axum::{extract::Request, ServiceExt};
use routes::routes_all;
use tower::Layer;
use tower_http::normalize_path::NormalizePathLayer;

mod auth;
mod schema;
mod templates;
mod database;
mod models;
mod middlewares;
mod routes;
mod helpers;

pub type BackendDbType = diesel::sqlite::Sqlite;
pub type DbConnectionType = diesel::sqlite::SqliteConnection;
pub const DEFAULT_CONTENT:&str = "Ohayou Sekai";

#[tokio::main]
async fn main() -> () {
    let routes = NormalizePathLayer::trim_trailing_slash().layer(routes_all());

    println!("SERVER ONLINE");
    println!(
        "LISTENING ON http://{}",
        dotenvy::var("SERVER_URL").unwrap()
    );
    axum::serve(
        tokio::net::TcpListener::bind(
            dotenvy::var("SERVER_URL").unwrap_or_else(|_| String::from("127.0.0.1:8080")),
        )
        .await
        .unwrap(),
        ServiceExt::<Request>::into_make_service(routes),
    )
    .await
    .unwrap();

    return ();
}