use diesel::{Connection, SqliteConnection};
use dotenvy::dotenv;

pub fn establish_connection() -> SqliteConnection {
    dotenv().ok();

    let db_url = dotenvy::var("DATABASE_URL").expect("DATABASE URL MISSING");
    SqliteConnection::establish(&db_url).unwrap_or_else(|_| panic!("Error connecting to Database"))
}