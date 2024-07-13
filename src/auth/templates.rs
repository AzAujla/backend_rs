use std::collections::HashMap;

use askama::Template;

#[derive(Template)]
#[template(path = "pages/auth/login.html")]
pub struct LoginPage<'a> {
    pub errors: HashMap<&'a str, &'a str>
}

#[derive(Template)]
#[template(path = "pages/auth/register.html")]
pub struct RegisterPage<'a> {
    pub errors: HashMap<&'a str, &'a str>
}