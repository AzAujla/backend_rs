use chrono;
use diesel::{prelude::Insertable, query_dsl::methods::FilterDsl, ExpressionMethods, RunQueryDsl};
use tower_cookies::Cookie;

use crate::{database::establish_connection, schema::sessions, DbConnectionType};

#[derive(Insertable, Clone)]
#[diesel(table_name = sessions)]
struct NewSessionCookie {
    user_id: i32,
    token: String,
}


const EXPIRE_AFTER: Option<u64> = None; // in days

pub fn generate_cookie(user_id: i32) -> Cookie<'static> {
    let conn = &mut establish_connection();
    
    let token = cookie_hash(user_id);
    revoke_old_sessions(conn);

    diesel::insert_into(sessions::table).values(
        NewSessionCookie {token: token.clone(), user_id}
    ).execute(conn).unwrap();

    Cookie::build((
        dotenvy::var("AUTH_TOKEN").unwrap(),
        token,
    ))
    .path("/")
    .build()
}

pub fn revoke_old_sessions(conn: &mut DbConnectionType) -> () {
    if EXPIRE_AFTER.is_some() {
        diesel::delete(sessions::table.filter(sessions::created_at.lt(
            chrono::NaiveDateTime::new(chrono::Utc::now().date_naive(), chrono::Utc::now().time()).checked_sub_days(chrono::Days::new(EXPIRE_AFTER.unwrap())).unwrap(),
        )))
        .execute(conn)
        .expect("Error occurered while revoking old tokens");
    }
}

fn cookie_hash(user_id: i32) -> String {
    return pwhash::bcrypt::hash(format!("{}AZEEM_RUST_AUTH_TOKEN", user_id)).unwrap();
}
