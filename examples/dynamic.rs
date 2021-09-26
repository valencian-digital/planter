use bson::doc;
use mongoseed;
use std::time::Instant;

fn user_generator(_history: &mongoseed::GeneratedData) -> bson::Document {
    return doc! {
        "_id": bson::Bson::ObjectId(bson::oid::ObjectId::new()),
        "name": "Jane Doe"
    };
}

fn post_generator(_history: &mongoseed::GeneratedData) -> bson::Document {
    return doc! {
        "_id": bson::Bson::ObjectId(bson::oid::ObjectId::new()),
        "text": "Hello World"
    };
}

fn main() {
    let amount = 10000;
    let now = Instant::now();
    let collections: Vec<(String, mongoseed::EntityGenerator)> = vec![
        (String::from("users"), user_generator),
        (String::from("posts"), post_generator),
        (String::from("comments"), post_generator),
        (String::from("communities"), post_generator),
    ];

    let mongo_uri = std::env::var("MONGO_URI").unwrap_or(String::from(""));
    println!("Mongo URI - {}", mongo_uri);

    println!("Total Time - {:?}", now.elapsed());
}
