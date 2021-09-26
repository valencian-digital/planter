use bson::doc;
use planter;
use std::time::Instant;

fn user_generator(_history: &planter::GeneratedData) -> bson::Document {
    return doc! {
        "_id": bson::Bson::ObjectId(bson::oid::ObjectId::new()),
        "name": "Jane Doe"
    };
}

fn company_generator(_history: &planter::GeneratedData) -> bson::Document {
    return doc! {
        "_id": bson::Bson::ObjectId(bson::oid::ObjectId::new()),
        "name": "Hello World",
        "location": "United States of America",
        "companySize": "1-10",
        "sector": bson::Bson::Array(vec![bson::Bson::String("Software".to_string())]),
        "website": "valenciandigital.com",
        "foundationDate": bson::DateTime::now(),
        "regionsCovered": bson::Bson::Array(vec![bson::Bson::String("Global".to_string())]),
        "rating": bson::Bson::Double(4.2),
        "totalReviews": bson::Bson::Int32(10),
        "contactEmail": "hello@valenciandigital.com"
    };
}

fn main() {
    let documents_per_collection: i32 = 10000;
    let now = Instant::now();
    let collections: Vec<(String, planter::EntityGenerator)> = vec![
        (String::from("users"), user_generator),
        (String::from("posts"), company_generator),
        (String::from("comments"), company_generator),
        (String::from("communities"), company_generator),
        (String::from("chat"), company_generator),
    ];

    println!(
        "Generating {} documents",
        documents_per_collection * collections.len() as i32
    );

    planter::seed_data(
        collections,
        planter::Configurations::new(documents_per_collection, planter::SeedMode::Disk),
    );

    println!("Total Time - {:?}", now.elapsed());
}
