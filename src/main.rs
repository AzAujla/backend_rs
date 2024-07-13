use askama::Template;
use axum::{extract::Request, middleware, response::Html, routing::get, Router, ServiceExt};
use templates::*;
use tower::Layer;
use tower_cookies::CookieManagerLayer;
use tower_http::{normalize_path::NormalizePathLayer, services::ServeDir};

mod auth;
mod schema;
mod templates;
// mod database;
mod middlewares;

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

fn routes_all() -> Router {
    return Router::new()
        .nest("/parts", routes_parts())
        .merge(routes_pages())
        .fallback_service(routes_files_static())
        .layer(CookieManagerLayer::new());
}

fn routes_files_static() -> Router {
    Router::new().nest_service("/assets", ServeDir::new("./assets/"))
}

fn routes_parts() -> Router {
    return Router::new().route(
        "/navbar",
        get(|| async {
            let nav = Navbar {
                buttons: vec![
                    NavButton {
                        name: "Home",
                        link: "/",
                        icon: "house",
                    },
                    NavButton {
                        name: "Search",
                        link: "/search",
                        icon: "search",
                    },
                    NavButton {
                        name: "Messages",
                        link: "/chat",
                        icon: "chat",
                    },
                    NavButton {
                        name: "Account",
                        link: "/account",
                        icon: "person-circle",
                    },
                ],
            };
            return Html::from(nav.render().unwrap());
        }),
    );
}

fn routes_pages() -> Router {
    return Router::new()
        .route("/", get(index_page))
        .nest("/auth", auth::routes())
        .layer(middleware::from_fn(middlewares::use_layout));

    async fn index_page() -> Html<&'static str> {
        println!("->> GET {:<12} PAGE", "indexPage");
        return Html::from("Hello");
    }
}
