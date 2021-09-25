pub mod execution {
    use crate::common::*;
    use std::fs;
    use std::time::Instant;
    use tokio::runtime::Runtime;
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
    pub fn update_collections(datasets: Vec<DataSet>, config: Configurations) {
        match config.mode {
            SeedMode::Dynamic => Runtime::new()
                .unwrap()
                .block_on(handle_dynamic_mode(datasets, config.mongo_uri.unwrap())),
            SeedMode::Disk => handle_disk_collection(datasets),
        }
    }
    async fn handle_dynamic_mode(_datasets: Vec<DataSet>, _uri: String) {
        panic!("Dynamic mode not implemented yet");
    }
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
