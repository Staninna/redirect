use crate::conf_set;
use hashbrown::HashMap;

pub struct Config {
    store: HashMap<String, String>,
}

impl Config {
    pub fn new() -> Self {
        let mut store = HashMap::new();

        conf_set!(store, "DATABASE_URL", String);
        conf_set!(store, "DATABASE_NAME", String);

        Self { store }
    }

    pub fn get(&self, key: &str) -> Option<&String> {
        self.store.get(key)
    }

    pub fn insert(&mut self, key: String, value: String) {
        self.store.insert(key, value);
    }
}
