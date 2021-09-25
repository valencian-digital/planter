pub mod seeding {
    use super::*;
    use crate::common::*;
    use crate::execution::execution;
    use std::string::String;

    pub fn seed_data(collections: Vec<(String, EntityGenerator)>, config: Configurations) {
        let datasets = generation::generate_collections(collections, config.amount);
        execution::update_collections(datasets, config);
    }
}

mod generation {
    use crate::common::*;
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
        collection_definition: Vec<(String, EntityGenerator)>,
        amount: i32,
    ) -> Vec<DataSet> {
        let now = Instant::now();
        let datasets = collection_definition
            .iter()
            .map(|(key, generator)| DataSet {
                collection_name: key.to_string(),
                output: generate_collection(*generator, amount as usize),
            })
            .collect();
        println!("Data Generation Time - {:?}", now.elapsed());
        return datasets;
    }
}
