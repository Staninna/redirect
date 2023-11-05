use crate::conf_set;
use hashbrown::HashMap;
use std::net::IpAddr;

#[derive(Debug)]
pub struct Config {
    store: HashMap<String, String>,
}

impl Config {
    pub fn new() -> Self {
        let mut store = HashMap::new();

        dotenvy::dotenv().ok();
        conf_set!(store, "DATABASE_URL", String);
        conf_set!(store, "DATABASE_MAX_CONNECTIONS", u32);
        conf_set!(store, "PORT", u16);
        conf_set!(store, "IP", IpAddr);
        conf_set!(store, "TERA_TEMPLATE_PATH", String);

        Self { store }
    }

    pub fn get(&self, key: &str) -> Option<&String> {
        self.store.get(key)
    }
}
