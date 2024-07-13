use std::collections::HashMap;

use axum::{http::StatusCode, response::Html, routing::get, Form, Json, Router};
use diesel::{prelude::*, ExpressionMethods, QueryDsl, SelectableHelper};
use serde::Deserialize;
use templates::{LoginPage, RegisterPage};
use tower_cookies::{Cookie, Cookies};

use crate::{
    database::establish_connection, models::User, schema::{self, users::{self, pswd}}, view, HelloTemplate, DbConnectionType, DEFAULT_CONTENT
};

mod templates;

pub fn routes() -> Router {
    return Router::new()
        .route("/login", get(login))
        .route("/register", get(register).post(create_user));
}

async fn login() -> Html<String> {
    println!("->> GET {:<12} LogInPage", "HANDLER");

    let errors = HashMap::new();
    return view::<LoginPage>(LoginPage { errors: errors });
}

async fn register() -> Html<String> {
    println!("->> GET {:<12} RegisterPage", "HANDLER");

    let errors = HashMap::new();
    return view::<RegisterPage>(RegisterPage { errors: errors });
}

async fn create_user(cookies: Cookies, Form(payload): Form<RegisterPayload>) -> Html<String> {
    let mut errors = HashMap::new();
    let connection = &mut establish_connection();
    let is_unique: Option<User> = users::table
        .filter(users::username.eq(&payload.username))
        .select(User::as_select())
        .first(connection)
        .optional()
        .unwrap();
    if is_unique.is_some() {
        errors.insert("username", "Username is not unique");
    }

    if payload.password.len() < 6 {
        errors.insert("password", "Password must be 6 characters at minimum");
    } else {
        if !payload.password.eq(&payload.confirm_password) {
            errors.insert(
                "confirm_password",
                "Password and Password Confirmation does not match.",
            );
        }
    }

    if errors.is_empty() {
        let new_user = insert_user(connection, &payload.username, &pwhash::bcrypt::hash(payload.password).unwrap()).unwrap();
        cookies.add(Cookie::build((dotenvy::var("AUTH_TOKEN").unwrap(), format!("{}.expr.sign", new_user.id))).build());
        return Html::from(String::from(DEFAULT_CONTENT));
    } else {
        return view::<RegisterPage>(RegisterPage { errors: errors });
    }
}

pub fn insert_user(
    conn: &mut DbConnectionType,
    username: &str,
    password: &str,
) -> Result<User, diesel::result::Error> {
    conn.transaction(|conn: &mut SqliteConnection| {
        diesel::insert_into(users::table)
            .values(&NewUser{username, pswd: password})
            .execute(conn)?;

        let inserted_user = users::table
            .order(users::id.desc())
            .first(conn)?;

        return Ok(inserted_user);
    })
}

#[derive(Debug, Deserialize)]
struct LoginPayload {
    username: String,
    password: String,
}
#[derive(Debug, Deserialize)]
struct RegisterPayload {
    username: String,
    password: String,
    confirm_password: String,
}
#[derive(Insertable)]
#[diesel(table_name = schema::users)]
struct NewUser<'a> {
    username: &'a str,
    pswd: &'a str,
}
