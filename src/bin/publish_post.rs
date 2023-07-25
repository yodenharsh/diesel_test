use self::models::Post;
use diesel::prelude::*;
use diesel_test::*;
use std::env::args;

fn main() {
    use self::schema::posts::dsl::{posts, published};

    let id = args()
        .nth(1)
        .expect("Expecting 1 argument - (post ID)")
        .parse::<i32>()
        .expect("Expecting an integer");

    let connection = &mut establish_connection();

    let post = connection
        .transaction(|connection| {
            let post = posts.find(id).select(Post::as_select()).first(connection)?;

            diesel::update(posts.find(id))
                .set(published.eq(true))
                .execute(connection)?;
            Ok(post)
        })
        .unwrap_or_else(|_: diesel::result::Error| panic!("Unable to find post {}", id));
}
