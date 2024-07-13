use std::collections::HashMap;

use askama::Template;

#[derive(Template)]
#[template(path = "pages/auth/login.html")]
pub struct LoginPage {
    pub errors: Option<Vec<HashMap<String, String>>>
}