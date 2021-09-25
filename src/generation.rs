use std::collections::HashMap;
use std::fs;
use std::iter;
use std::string::String;
use std::time::Instant;

pub type EntityGenerator = fn() -> bson::Document;

struct DataSet {
    filename: String,
    output: Vec<bson::Bson>,
}

fn generate_collection(entity_generator: EntityGenerator, amount: usize) -> Vec<bson::Bson> {
    let collection: Vec<bson::Bson> = iter::repeat(entity_generator)
        .take(amount)
        .map(|f| bson::to_bson(&f()).expect("Error ocurred while converting to bson"))
        .collect();

    return collection;
}

pub fn create_data(names: Vec<String>, generators: Vec<EntityGenerator>, amount: i32) {
    let now = Instant::now();

    let entity_generators: HashMap<String, EntityGenerator> =
        names.into_iter().zip(generators.into_iter()).collect();
    let datasets = entity_generators
        .keys()
        .map(|key| DataSet {
            filename: key.to_string() + ".json",
            output: generate_collection(entity_generators[key], amount as usize),
        })
        .collect();
    println!("Data Generation Time - {:?}", now.elapsed());
    write_files(datasets);
}

fn update_collection(dataset: &DataSet) {
    let output = dataset
        .output
        .iter()
        .map(|doc| doc.clone().into_canonical_extjson().to_string())
        .collect::<Vec<String>>();

    let final_output = String::from("[") + &output.join(",") + "]";
    fs::write(dataset.filename.clone(), final_output)
        .expect("An error ocurred while writing a collection");
}

fn write_files(datasets: Vec<DataSet>) {
    let now = Instant::now();

    datasets
        .iter()
        .for_each(|dataset| update_collection(dataset));
    println!("Writing Data Time - {:?}", now.elapsed());
}
