use self::models::Post;
use diesel::prelude::*;
use diesel_test::{schema::posts::id, *};
use std::env::args;

fn main() {
    use self::schema::posts::dsl::{posts, published};

    let given_id = args()
        .nth(1)
        .expect("publish_post requires a post id")
        .parse::<i32>()
        .expect("Invalid ID");
    let connection: &mut MysqlConnection = &mut establish_connection();

    let post: Post = connection
        .transaction(|connection: &mut MysqlConnection| {
            let post: Post = posts
                .select(Post::as_select())
                .find(given_id)
                .first(connection)?;
            println!("{}", post.title);

            diesel::update(posts)
                .filter(id.eq(1))
                .set(published.eq(true))
                .execute(connection)?;
            Ok(post)
        })
        .unwrap_or_else(|_: diesel::result::Error| panic!("Unable to find post {}", given_id));

    println!("Published post {}", post.title);
}
