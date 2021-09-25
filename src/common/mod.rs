pub type EntityGenerator = fn() -> bson::Document;

pub struct DataSet {
    pub collection_name: String,
    pub output: Vec<bson::Bson>,
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
