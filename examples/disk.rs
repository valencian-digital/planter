use bson::doc;
use planter;
use std::time::Instant;

fn user_generator(_history: &planter::GeneratedData) -> bson::Document {
    return doc! {
        "_id": bson::Bson::ObjectId(bson::oid::ObjectId::new()),
        "name": "Jane Doe"
    };
}

fn post_generator(_history: &planter::GeneratedData) -> bson::Document {
    return doc! {
        "_id": bson::Bson::ObjectId(bson::oid::ObjectId::new()),
        "text": "Hello World"
    };
}

fn main() {
    let amount = 50000;
    let now = Instant::now();
    let collections: Vec<(String, planter::EntityGenerator)> = vec![
        (String::from("users"), user_generator),
        (String::from("posts"), post_generator),
        (String::from("comments"), post_generator),
        (String::from("communities"), post_generator),
    ];

    planter::seed_data(
        collections,
        planter::Configurations::new(amount, planter::SeedMode::Disk),
    );

    println!("Total Time - {:?}", now.elapsed());
}
