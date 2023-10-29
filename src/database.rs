use crate::{conf_get, config::Config};
use sqlx::{mysql::MySqlPoolOptions, MySql, Pool};

pub struct Database {
    pool: Pool<MySql>,
}

impl Database {
    pub async fn new(config: &Config) -> Self {
        let schema = conf_get!(config, "DATABASE_URL", String);
        let name = conf_get!(config, "DATABASE_NAME", String);
        let url = format!("{}/{}", schema, name);

        let pool = MySqlPoolOptions::new()
            .max_connections(5)
            .connect(&url)
            .await
            .expect("Failed to connect to database");

        Self { pool }
    }
}
