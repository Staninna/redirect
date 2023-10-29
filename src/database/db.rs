use crate::{conf_get, config::Config};
use sqlx::{migrate, mysql::MySqlPoolOptions, MySql, Pool};

pub struct Db {
    pool: Pool<MySql>,
}

impl Db {
    pub async fn new(config: &Config) -> Self {
        let url = conf_get!(config, "DATABASE_URL", String);
        let max_conn = conf_get!(config, "DATABASE_MAX_CONNECTIONS", u32);

        let pool = MySqlPoolOptions::new()
            .max_connections(max_conn)
            .connect(&url)
            .await
            .expect("Failed to connect to database");

        let result = migrate!().run(&pool).await;

        match result {
            Ok(_) => (),
            Err(err) => {
                log::error!("Failed to migrate database: {}", err);
                std::process::exit(1);
            }
        }

        log::info!("Connected to database");
        Self { pool }
    }

    pub async fn get_redirect(&self, code: &str) -> Option<String> {
        let result = sqlx::query!("SELECT url FROM redirects WHERE code = ?", code)
            .fetch_optional(&self.pool)
            .await
            .expect("Failed to get redirect");

        match result {
            Some(row) => Some(row.url),
            None => None,
        }
    }
}
