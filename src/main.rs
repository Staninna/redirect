use config::Config;
use database::Db;
use rocket::{launch, routes, Config as RocketConfig};
use routes::{create_redirect, redirect};

// TODO: Add custom 404 handler
// TODO: Add custom 500 handler
// TODO: Add redirect stats

mod config;
mod database;
mod routes;

#[launch]
async fn rocket() -> _ {
    let config = Config::new();
    let rocket_config = RocketConfig::figment()
        .merge(("port", conf_get!(config, "PORT", i64)))
        .merge(("address", conf_get!(config, "IP", String)));

    rocket::build()
        .configure(rocket_config)
        .mount("/", routes![redirect, create_redirect])
        .manage(Db::new(&config).await)
        .manage(config)
}

#[macro_export]
macro_rules! conf_get {
    ($config:expr, $key:expr, $type:ty) => {
        $config
            .get($key)
            .expect(&format!("{} must be set", $key))
            .parse::<$type>()
            .expect(&format!("{} must be a {}", $key, stringify!($type)))
    };
}

#[macro_export]
macro_rules! conf_set {
    ($config:ident, $env_var:literal, $type:ty) => {
        let value = dotenvy::var($env_var).expect(&format!("{} must be set", $env_var));
        let parsed_value: $type = value.parse().expect(&format!(
            "{} must be a valid {}",
            $env_var,
            stringify!($type)
        ));
        $config.insert($env_var.to_string(), parsed_value.to_string())
    };
}
