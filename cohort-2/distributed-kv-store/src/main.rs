use std::collections::HashMap;

struct KeyValueStore {
    data: HashMap<String, String>,
}

impl KeyValueStore {
    fn new() -> Self {
        KeyValueStore {
            data: HashMap::new(),
        }
    }

    fn put(&mut self, key: String, value: String) {
        self.data.insert(key, value);
    }

    fn get(&self, key: &str) -> Option<&String> {
        self.data.get(key)
    }
}

fn main() {
    let mut kv_store = KeyValueStore::new();

    kv_store.put("key1".to_string(), "value1".to_string());
    kv_store.put("key2".to_string(), "value2".to_string());

    if let Some(value) = kv_store.get("key1") {
        println!("Key1: {}", value);
    } else {
        println!("Key1 not found.");
    }

    if let Some(value) = kv_store.get("key3") {
        println!("Key3: {}", value);
    } else {
        println!("Key3 not found.");
    }
}
