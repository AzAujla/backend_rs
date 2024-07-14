use askama::Template;
use axum::{middleware, response::Html, routing::get, Router};
use tower_cookies::{CookieManagerLayer, Cookies};
use tower_http::services::ServeDir;

use crate::{middlewares::{self, mw_require_auth}, templates::NavButton, templates::Navbar, DEFAULT_CONTENT};

pub fn routes_all() -> Router {
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
        .layer(middleware::from_fn(mw_require_auth))
        .nest("/auth", crate::auth::routes())
        .layer(middleware::from_fn(middlewares::use_layout));

    async fn index_page(_cookies: Cookies) -> Html<&'static str> {
        println!("->> GET {:<12} PAGE", "indexPage");
        return Html::from(DEFAULT_CONTENT);
    }
}
