pub mod seeding {
    use super::*;
    use crate::common::*;
    use crate::execution::execution;
    use std::string::String;

    // Takes vector of tuples where each item in the vector will be used to define that given collection.
    // The generator will be trigerred N times where N is the amount specified in teh config struct passed in.
    pub fn seed_data(collections: Vec<(String, EntityGenerator)>, config: Configurations) {
        let datasets =
            generation::generate_collections(collections, config.documents_per_collection);
        execution::update_collections(datasets, config);
    }
}

mod generation {
    use crate::common::*;
    use rayon::prelude::*;
    use std::collections::HashMap;
    use std::time::Instant;

    fn generate_collection(
        entity_generator: EntityGenerator,
        history: &GeneratedData,
        amount: usize,
    ) -> Vec<bson::Document> {
        let collection: Vec<bson::Document> = rayon::iter::repeatn(entity_generator, amount)
            .map(|f| f(history))
            .collect();

        return collection;
    }

    pub fn generate_collections(
        collection_definition: Vec<(String, EntityGenerator)>,
        amount: i32,
    ) -> GeneratedData {
        let mut history: GeneratedData = HashMap::new();

        let now = Instant::now();
        collection_definition
            .into_iter()
            .for_each(|(key, generator)| {
                let coll_now = Instant::now();
                let generated_collection =
                    generate_collection(generator, &history, amount as usize);
                println!(
                    "Collection {} Generation Time - {:?}",
                    key,
                    coll_now.elapsed()
                );

                history.insert(key, generated_collection);
            });
        println!("Data Generation Time - {:?}", now.elapsed());
        return history;
    }
}
