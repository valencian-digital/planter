mod generation;
use bson::doc;
use std::time::Instant;

fn user_generator() -> bson::Document {
    return doc! {
        "_id": bson::Bson::ObjectId(bson::oid::ObjectId::new()),
        "name": "Jane Doe"
    };
}

fn post_generator() -> bson::Document {
    return doc! {
        "_id": bson::Bson::ObjectId(bson::oid::ObjectId::new()),
        "text": "Hello World"
    };
}
fn main() {
    let amount = 100;
    let now = Instant::now();
    let collections: Vec<(String, generation::EntityGenerator)> = vec![
        (String::from("users"), user_generator),
        (String::from("posts"), post_generator),
        (String::from("comments"), post_generator),
        (String::from("communities"), post_generator),
    ];

    let mongo_uri = std::env::var("MONGO_URI").unwrap_or(String::from(""));
    println!("Mongo URI - {}", mongo_uri);
    generation::seeding::seed_data(
        collections,
        generation::Configurations::new(amount, generation::SeedMode::Disk),
    );

    println!("Total Time - {:?}", now.elapsed());
}
