use crate::database::Db;
use rocket::{
    get, post,
    response::{Redirect, Responder},
    State,
};
use tera::{Context, Tera};

#[get("/<code>")]
pub async fn redirect(
    code: String,
    database: &State<Db>,
    tera: &State<Tera>,
) -> Result<Redirect, NotFound> {
    let url = database.get_redirect(&code).await;

    match url {
        Some(url) => Ok(Redirect::to(url)),
        None => Err(not_found(tera)),
    }
}

#[post("/?<code>&<url>")]
pub async fn create_redirect(
    code: String,
    url: String,
    database: &State<Db>,
) -> Result<String, String> {
    let result = database.create_redirect(&code, &url).await;

    match result {
        Ok(_) => Ok(format!("Created redirect /{} -> {}", code, url)),
        Err(err) => Err(err),
    }
}

#[derive(Responder)]
#[response(status = 404, content_type = "html")]
pub struct NotFound(String);

fn not_found(tera: &State<Tera>) -> NotFound {
    let mut context = Context::new();
    context.insert("title", "404 - URL not found");
    context.insert("message", "The URL you requested could not be found.");
    let body = tera.render("404.tera.html", &context).unwrap();

    NotFound(body)
}
