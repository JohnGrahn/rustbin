use crate::models::{PasteData, ExpirationTime};

use dioxus::prelude::*;


#[server(CreatePaste)]
pub async fn create_paste(content: String, expiration: ExpirationTime) -> Result<String, ServerFnError> {
    use rand::Rng;
    use sqlx::PgPool;

    let pool = PgPool::connect(&std::env::var("DATABASE_URL")?).await?;
    let id: String = rand::thread_rng()
        .sample_iter(&rand::distributions::Alphanumeric)
        .take(8)
        .map(char::from)
        .collect();

    let now = chrono::Utc::now();
    let expiration_duration = expiration.to_duration();
    let expires_at = now + expiration_duration;

    sqlx::query!(
        "INSERT INTO pastes (id, content, created_at, expires_at) VALUES ($1, $2, $3, $4)",
        id,
        content,
        now,
        expires_at
    )
    .execute(&pool)
    .await?;

    Ok(id)
}

#[server(GetPaste)]
pub async fn get_paste(id: String) -> Result<PasteData, ServerFnError> {
    use sqlx::PgPool;

    let pool = PgPool::connect(&std::env::var("DATABASE_URL")?).await?;
    let paste = sqlx::query_as!(
        PasteData,
        "SELECT id, content, created_at, expires_at FROM pastes WHERE id = $1",
        id
    )
    .fetch_one(&pool)
    .await?;

    Ok(paste)
}