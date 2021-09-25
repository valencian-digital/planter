pub type EntityGenerator = fn() -> bson::Document;

pub struct DataSet {
    collection_name: String,
    output: Vec<bson::Bson>,
}
pub enum SeedMode {
    Dynamic,
    Disk,
}
pub struct Configurations {
    pub amount: i32,
    pub mode: SeedMode,
    pub mongo_uri: Option<String>,
}

impl Configurations {
    pub fn new(amount: i32, mode: SeedMode) -> Configurations {
        match mode {
            SeedMode::Dynamic => panic!("Dynamic seed mode not implemented yet"),
            SeedMode::Disk => Configurations {
                amount,
                mode,
                mongo_uri: None,
            },
        }
    }
    pub fn new_dynamic(amount: i32, mode: SeedMode, mongo_uri: String) -> Configurations {
        match mode {
            SeedMode::Dynamic => Configurations {
                amount,
                mode,
                mongo_uri: Some(mongo_uri),
            },
            SeedMode::Disk => panic!("Disk seed mode not implemented yet"),
        }
    }
}

pub mod seeding {
    use super::*;
    use std::string::String;

    pub fn seed_data(names: Vec<String>, generators: Vec<EntityGenerator>, config: Configurations) {
        let datasets = generation::generate_collections(names, generators, config.amount);
        execution::update_collections(datasets, config.mode);
    }
}

mod generation {
    use super::*;
    use std::iter;
    use std::time::Instant;

    fn generate_collection(entity_generator: EntityGenerator, amount: usize) -> Vec<bson::Bson> {
        let collection: Vec<bson::Bson> = iter::repeat(entity_generator)
            .take(amount)
            .map(|f| bson::Bson::from(f()))
            .collect();
        return collection;
    }

    pub fn generate_collections(
        names: Vec<String>,
        generators: Vec<EntityGenerator>,
        amount: i32,
    ) -> Vec<DataSet> {
        let now = Instant::now();
        let entity_generators = names.into_iter().zip(generators.into_iter());
        let datasets = entity_generators
            .map(|(key, generator)| DataSet {
                collection_name: key,
                output: generate_collection(generator, amount as usize),
            })
            .collect();
        println!("Data Generation Time - {:?}", now.elapsed());
        return datasets;
    }
}

mod execution {
    use super::*;
    use std::fs;
    use std::time::Instant;

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
    pub fn update_collections(datasets: Vec<DataSet>, mode: SeedMode) {
        match mode {
            SeedMode::Dynamic => handle_dynamic_mode(datasets),
            SeedMode::Disk => handle_disk_collection(datasets),
        }
    }
    fn handle_dynamic_mode(_datasets: Vec<DataSet>) {}
    fn handle_disk_collection(datasets: Vec<DataSet>) {
        let collections = convert_collections(&datasets);
        datasets
            .into_iter()
            .zip(collections.into_iter())
            .for_each(|(dataset, collection)| {
                write_collection(
                    String::from("./data/") + &dataset.collection_name.to_string() + ".json",
                    collection,
                )
            });
    }
}
