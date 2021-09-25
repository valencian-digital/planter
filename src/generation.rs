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
