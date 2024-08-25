use crate::models::PasteData;
use dioxus::prelude::*;


#[server(CreatePaste)]
pub async fn create_paste(content: String) -> Result<String, ServerFnError> {
    use rand::Rng;
    use sqlx::PgPool;

    let pool = PgPool::connect(&std::env::var("DATABASE_URL")?).await?;
    let id: String = rand::thread_rng()
        .sample_iter(&rand::distributions::Alphanumeric)
        .take(8)
        .map(char::from)
        .collect();

    sqlx::query!(
        "INSERT INTO pastes (id, content, created_at) VALUES ($1, $2, $3)",
        id,
        content,
        chrono::Utc::now()
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
        "SELECT id, content, created_at FROM pastes WHERE id = $1",
        id
    )
    .fetch_one(&pool)
    .await?;

    Ok(paste)
}