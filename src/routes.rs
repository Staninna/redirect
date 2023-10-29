use crate::database::Db;
use rocket::{get, post};

#[get("/<code>")]
pub async fn redirect(
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
pub async fn create_redirect(
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
