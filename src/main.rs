use std::collections::HashMap;

struct DataSet {
    filename: String,
    output: String,
}
type EntityGenerator = fn() -> String;

fn user_generator() -> String {
    return "Generating users".to_string();
}

fn post_generator() -> String {
    return "Generating posts".to_string();
}

const COLLECTION_NAMES: [&str; 2] = ["users", "posts"];
const ENTITY_GENERATORS: [EntityGenerator; 2] = [user_generator, post_generator];

fn main() {
    let entity_generators: HashMap<&str, EntityGenerator> = COLLECTION_NAMES
        .iter()
        .map(|name| *name)
        .zip(ENTITY_GENERATORS.iter().map(|gen| *gen))
        .collect();

    let datasets = COLLECTION_NAMES
        .iter()
        .map(|name| DataSet {
            filename: name.to_string() + ".json",
            output: entity_generators[name](),
        })
        .collect::<Vec<DataSet>>();
    write_files(datasets);
}

fn update_collection(dataset: &DataSet) {
    println!("Updating collection {}", dataset.filename);
    println!("{}", dataset.output);
}

fn write_files(datasets: Vec<DataSet>) {
    datasets
        .iter()
        .for_each(|dataset| update_collection(dataset));
}
