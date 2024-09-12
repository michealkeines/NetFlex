use crate::parsers::ParsedData;

pub struct StorageManager;

impl StorageManager {
    pub fn store_data(&self, data: ParsedData) {
        println!("Storing data: {:?}", data);
    }
}
