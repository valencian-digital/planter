pub mod execution {
    use crate::common::*;
    use std::fs;
    use std::time::Instant;
    use tokio::runtime::Runtime;

    pub fn update_collections(datasets: GeneratedData, config: Configurations) {
        match config.mode {
            SeedMode::Dynamic => Runtime::new()
                .unwrap()
                .block_on(handle_dynamic_mode(datasets, config.mongo_uri.unwrap())),
            SeedMode::Disk => handle_disk_collection(datasets),
        }
    }
    fn write_collection(name: String, collection: String) {
        let now = Instant::now();
        fs::write(name, collection).expect("An error ocurred while writing a collection");
        println!("Writing Data Time - {:?}", now.elapsed());
    }

    fn convert_collection(data: Vec<bson::Bson>) -> String {
        let output = data
            .into_iter()
            .map(|doc| doc.into_canonical_extjson().to_string())
            .collect::<Vec<String>>();
        let final_output = String::from("[") + &output.join(",") + "]";
        return final_output;
    }

    fn create_filepath(collection_name: String) -> String {
        return String::from("./data/") + &collection_name + ".json";
    }

    async fn handle_dynamic_mode(_datasets: GeneratedData, _uri: String) {
        panic!("Dynamic mode not implemented yet");
    }

    fn handle_disk_collection(datasets: GeneratedData) {
        datasets.into_iter().for_each(|(key, output)| {
            write_collection(create_filepath(key), convert_collection(output))
        });
    }
}
