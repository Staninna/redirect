use super::Redirect;
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
        if let Err(err) = result {
            log::error!("Failed to migrate database: {}", err);
            std::process::exit(1);
        }

        Self { pool }
    }

    pub async fn get_redirect(&self, code: &str) -> Option<String> {
        let redirect = sqlx::query_as!(
            Redirect,
            "SELECT id, code, url FROM redirects WHERE code = ?",
            code
        )
        .fetch_optional(&self.pool)
        .await
        .expect("Failed to query database");

        match redirect {
            Some(redirect) => Some(redirect.url),
            None => None,
        }
    }

    pub async fn create_redirect(&self, code: &str, url: &str) -> Result<(), String> {
        let result = sqlx::query_as!(
            Redirect,
            "INSERT INTO redirects (code, url) VALUES (?, ?)",
            code,
            url
        )
        .execute(&self.pool)
        .await;

        match result {
            Ok(_) => Ok(()),
            Err(err) => Err(err.to_string()),
        }
    }
}
