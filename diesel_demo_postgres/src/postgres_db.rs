use crate::schema::posts::{published, title};
use crate::schema::posts::dsl::posts;
use crate::models::{NewPost, Post};
use diesel::pg::PgConnection;
use diesel::prelude::*;
use dotenvy::dotenv;
use std::env;


// 获取数据库连接
pub fn db_establish_connection() -> PgConnection {
    dotenv().ok();

    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    PgConnection::establish(&database_url)
        .unwrap_or_else(|_| panic!("Error connecting to {}", database_url))
}

// 增加NewPost数据
pub fn db_create_post(connection: &mut PgConnection, blog_title: &str, blog_body: &str) -> Post {
    use crate::schema::posts;

    let new_post = NewPost { title:blog_title, body:blog_body };

    diesel::insert_into(posts::table)
        .values(&new_post)
        .returning(Post::as_returning())
        .get_result(connection)
        .expect("Error saving new post")
}

//更新发布状态
pub fn db_update_post(connection: &mut PgConnection, post_id: i32) {
    let post = diesel::update(posts.find(post_id))
        .set(published.eq(true))
        .returning(Post::as_returning())
        .get_result(connection).optional();

    match post {
        Ok(Some(result)) => { println!("Published post {}", result.title); }
        Ok(None) => { println!("unable fo find post {}", post_id) }
        Err(error) => { println!("db_update_post:{}", error) }
    }
}

// 查询指定id的数据
pub fn db_get_post(connection: &mut PgConnection, post_id: i32) {
    let post = posts
        .find(post_id)
        .select(Post::as_select())
        .first(connection)
        .optional(); // This allows for returning an Option<Post>, otherwise it will throw an error

    match post {
        Ok(Some(post)) => println!("Post with id: {} has a title: {}", post.id, post.title),
        Ok(None) => println!("Unable to find post {}", post_id),
        Err(_) => println!("An error occured while fetching post {}", post_id),
    }
}

// 查询发布状态的数据
pub fn db_show_posts(connection: &mut PgConnection, limit: i64) -> Vec<Post> {
    let results = posts
        .filter(published.eq(true))
        .limit(limit)
        .select(Post::as_select())
        .load::<Post>(connection)
        .expect("Error loading posts");

    println!("Displaying {} posts", results.len());
    for post in &results {
        println!("id:{}", post.id);
        println!("title:{}", post.title);
        println!("------body-----\n");
        println!("{}", post.body);
    }

    results
}

// 查询最多10000条数据，并作为数组返回
pub fn db_show_all_posts(connection: &mut PgConnection) -> Vec<Post> {
    let results = posts
        .limit(10000)
        .select(Post::as_select())
        .load::<Post>(connection)
        .expect("Error loading posts");

    println!("Displaying {} posts", results.len());
    for post in &results {
        println!("id:{}", post.id);
        println!("title:{}", post.title);
        println!("------body-----\n");
        println!("{}", post.body);
    }

    results
}

//删除标题中含有某个字符串的数据
pub fn db_delete_post(connection: &mut PgConnection, pattern: &str) {

    let num_deleted = diesel::delete(posts.filter(title.like(pattern)))
        .execute(connection)
        .expect("Error deleting posts");

    println!("Deleted {} posts", num_deleted);
}