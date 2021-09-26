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
    fn write_collection(name: String, collection: Vec<u8>) {
        let now = Instant::now();
        fs::write(name.as_str(), collection).expect("An error ocurred while writing a collection");
        println!(
            "Writing Collection Time for {} - {:?}",
            name.as_str(),
            now.elapsed()
        );
    }

    fn convert_collection(data: Vec<bson::Document>) -> Vec<u8> {
        let now = Instant::now();
        let mut bytes = vec![];
        data.into_iter().for_each(|doc| {
            doc.to_writer(&mut bytes)
                .expect("Error writing to byte array")
        });
        println!("Collection Conversion Time - {:?}", now.elapsed());
        return bytes;
    }

    fn create_filepath(collection_name: String) -> String {
        return String::from("./data/") + &collection_name + ".bson";
    }

    async fn handle_dynamic_mode(_datasets: GeneratedData, _uri: String) {
        panic!("Dynamic mode not implemented yet");
    }

    fn handle_disk_collection(datasets: GeneratedData) {
        let now = Instant::now();
        datasets.into_iter().for_each(|(key, output)| {
            write_collection(create_filepath(key), convert_collection(output))
        });
        println!("Disk Execution Time - {:?}", now.elapsed());
    }
}
