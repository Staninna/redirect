use config::Config;
use rocket::{get, launch, routes};

mod config;

#[launch]
fn rocket() -> _ {
    rocket::build().mount("/", routes![index])
}

#[get("/")]
fn index() -> &'static str {
    "Hello, world!"
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
        let value = dotenv::var($env_var).expect(&format!("{} must be set", $env_var));
        let parsed_value: $type = value.parse().expect(&format!(
            "{} must be a valid {}",
            $env_var,
            stringify!($type)
        ));
        $config.insert($env_var.to_string(), parsed_value.to_string())
    };
}
