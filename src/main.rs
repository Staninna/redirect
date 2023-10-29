use config::Config;
use database::Db;
use rocket::{get, launch, post, routes};

// TODO: Add custom 404 handler
// TODO: Add custom 500 handler
// TODO: Add redirect stats

mod config;
mod database;

#[launch]
async fn rocket() -> _ {
    let config = Config::new();

    rocket::build()
        .mount("/", routes![redirect, create_redirect])
        .manage(Db::new(&config).await)
        .manage(config)
}

#[get("/<code>")]
async fn redirect(
    code: String,
    database: &rocket::State<Db>,
) -> Option<rocket::response::Redirect> {
    let url = database.get_redirect(&code).await;
    match url {
        Some(url) => Some(rocket::response::Redirect::to(url)),
        None => None,
    }
}

#[post("/?<code>&<url>")]
async fn create_redirect(
    code: String,
    url: String,
    database: &rocket::State<Db>,
) -> Result<String, String> {
    let result = database.create_redirect(&code, &url).await;

    match result {
        Ok(_) => Ok(format!("Created redirect /{} -> {}", code, url)),
        Err(err) => Err(err),
    }
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
