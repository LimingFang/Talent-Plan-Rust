use std::collections::HashMap;

pub struct KvStore {
    data: HashMap<String, String>,
}

impl KvStore {
    pub fn set(&mut self, key: String, value: String) {
        self.data.insert(key, value);
    }

    pub fn get(&self, key: String) -> Option<String> {
        self.data.get(&key).map(|s| s.to_owned())
    }

    pub fn remove(&mut self, key: String) {
        self.data.remove(&key);
    }

    pub fn new() -> KvStore {
        KvStore {
            data: HashMap::new(),
        }
    }
}

impl Default for KvStore {
    fn default() -> Self {
        Self::new()
    }
}
