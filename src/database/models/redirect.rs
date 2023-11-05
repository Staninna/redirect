use sqlx::FromRow;

#[derive(FromRow)]
pub struct Redirect {
    pub id: i32,
    pub code: String,
    pub url: String,
}
