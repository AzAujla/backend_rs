use axum::response::Html;

pub fn htmx_redirect(link: &str, method: &str) -> Html<String> {
    Html::from(format!(r#"<div hx-{}="{}" hx-target="main" hx-swap="innerHTML" hx-trigger="load" hx-push-url="true"></div>"#, method, link))
}
