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
        .map(|f| bson::Bson::from(f()))
        .collect();

    return collection;
}

pub fn create_data(names: Vec<String>, generators: Vec<EntityGenerator>, amount: i32) {
    let now = Instant::now();

    let entity_generators = names.into_iter().zip(generators.into_iter());
    let datasets = entity_generators
        .map(|(key, generator)| DataSet {
            filename: key.to_string() + ".json",
            output: generate_collection(generator, amount as usize),
        })
        .collect();
    println!("Data Generation Time - {:?}", now.elapsed());
    write_files(datasets);
}

fn write_collection(name: String, collection: String) {
    let now = Instant::now();

    fs::write(name, collection).expect("An error ocurred while writing a collection");
    println!("Writing Data Time - {:?}", now.elapsed());
}

fn convert_collections(datasets: &Vec<DataSet>) -> Vec<String> {
    let now = Instant::now();

    let collections: Vec<String> = datasets
        .iter()
        .map(|dataset| {
            let output = dataset
                .output
                .iter()
                .map(|doc| doc.clone().into_canonical_extjson().to_string())
                .collect::<Vec<String>>();

            let final_output = String::from("[") + &output.join(",") + "]";
            return final_output;
        })
        .collect();
    println!("Converting to JSON Time - {:?}", now.elapsed());
    return collections;
}

fn write_files(datasets: Vec<DataSet>) {
    let collections = convert_collections(&datasets);
    datasets
        .into_iter()
        .zip(collections.into_iter())
        .for_each(|(datasets, collection)| write_collection(datasets.filename, collection));
}
