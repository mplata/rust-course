
    let conn = PgConnection::establish(&db_url).expect("DB Connection failed");
    
    use self::models::{Post, NewPost};
    use self::schema::posts;
    use self::schema::posts::dsl::*;

    let mut new_post;
    for i in 1..5 {
      let slug_text = format!("Post: {i}", i=i);
      new_post = NewPost {
          title: "Mi segundo Post",
          body: "Lorem ipsum fact",
          slug: slug_text.as_str()
      };
      let _post: Post = diesel::insert_into(posts::table).values(new_post).get_result(&conn).expect("Insert fails");
    }

    /*let filtered_post = posts.filter(id.eq(3));
    let update_post = diesel::update(filtered_post).set(slug.eq("tercer-post")).get_result::<Post>(&conn).expect("Update error");*/
    // Select * from posts

    //diesel::delete(posts.filter(id.eq(3))).execute(&conn).expect("Delete fail");
    /*println!("Query in limites");
    let posts_result = posts.load::<Post>(&conn).expect("Querying Error");

    for post in posts_result {
        println!("{:?}", post);
    }**
