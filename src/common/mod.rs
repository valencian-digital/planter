use std::collections::HashMap;

pub type GeneratedData = HashMap<String, Vec<bson::Document>>;
pub type EntityGenerator = fn(history: &GeneratedData) -> bson::Document;

#[derive(Debug)]
pub struct DataSet {
    pub collection_name: String,
    pub output: Vec<bson::Bson>,
}

#[derive(Debug)]
pub enum SeedMode {
    Dynamic,
    Disk,
}

#[derive(Debug)]
pub struct Configurations {
    pub documents_per_collection: i32,
    pub mode: SeedMode,
    pub mongo_uri: Option<String>,
}
impl Configurations {
    pub fn new(documents_per_collection: i32, mode: SeedMode) -> Configurations {
        match mode {
            SeedMode::Dynamic => panic!("Dynamic seed mode not implemented yet"),
            SeedMode::Disk => Configurations {
                documents_per_collection,
                mode,
                mongo_uri: None,
            },
        }
    }
    pub fn new_dynamic(
        documents_per_collection: i32,
        mode: SeedMode,
        mongo_uri: String,
    ) -> Configurations {
        match mode {
            SeedMode::Dynamic => Configurations {
                documents_per_collection,
                mode,
                mongo_uri: Some(mongo_uri),
            },
            SeedMode::Disk => panic!("Disk seed mode not implemented yet"),
        }
    }
}
