use diesel::prelude::*;
use diesel_test::models::*;
use diesel_test::*;

fn main() {
    use self::schema::posts::dsl::*;

    let connection = &mut establish_connection();
    let results = posts
        .select(Post::as_select())
        .limit(5)
        .filter(published.eq(true))
        .load(connection)
        .expect("Error loading post");

    println!("Displaying {} posts", results.len());
    for post in results {
        println!("{}", post.title);
        println!("----------------\n");
        println!("{}", post.body);
    }
}
