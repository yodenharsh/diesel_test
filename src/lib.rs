use self::models::{NewPost, Post};
use diesel::mysql::MysqlConnection;
use diesel::prelude::*;
use dotenvy::dotenv;
use std::env;

pub mod models;
pub mod schema;

pub fn establish_connection() -> MysqlConnection {
    dotenv().ok();

    let database_url = env::var("DATABASE_URL").expect("Database url must be set!");
    MysqlConnection::establish(&database_url)
        .unwrap_or_else(|_| panic!("Error connecting to given url"))
}

pub fn create_post(conn: &mut MysqlConnection, title: &str, body: &str) -> Post {
    use crate::schema::posts;

    let new_post = NewPost { title, body };
    conn.transaction(|conn| {
        diesel::insert_into(posts::table)
            .values(&new_post)
            .execute(conn)?;

        posts::table
            .order(posts::id.desc())
            .select(Post::as_select())
            .first(conn)
    })
    .expect("Error saving post")
}
