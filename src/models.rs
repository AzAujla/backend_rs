use diesel::{deserialize::Queryable, Selectable};
use tower_cookies::cookie::time::PrimitiveDateTime;

use crate::BackendDbType;

#[derive(Queryable, Selectable)]
#[diesel(table_name = crate::schema::users)]
#[diesel(check_for_backend(BackendDbType))]
pub struct User {
    pub id: i32,
    pub username: String,
    pub pswd: String,
    pub user_type_id: i32,
    pub account_status_id: i32,
    pub created_at: PrimitiveDateTime,
    updated_at: PrimitiveDateTime,
}
#[derive(Queryable, Selectable)]
#[diesel(table_name = crate::schema::sessions)]
#[diesel(check_for_backend(BackendDbType))]
pub struct Session {
    id: i32,
    user_id: i32,
    token: String,
    created_at: PrimitiveDateTime,
}