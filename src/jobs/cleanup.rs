use dioxus::prelude::*;




#[server(StartCleanupJob)]
async fn start_cleanup_job() -> Result<(), ServerFnError> {
    use tokio::time::Duration;
    loop {
        tokio::time::sleep(Duration::from_secs(60)).await;
        if let Err(e) = cleanup_expired_pastes().await {
            eprintln!("Error cleaning up expired pastes: {}", e);
        }
    }
}

#[server(CleanupExpiredPastes)]
async fn cleanup_expired_pastes() -> Result<(), ServerFnError> {
    use sqlx::PgPool;

    let pool = PgPool::connect(&std::env::var("DATABASE_URL")?).await?;
    sqlx::query!("DELETE FROM pastes WHERE expires_at < NOW()")
        .execute(&pool)
        .await?;
    Ok(())
}

#[cfg(feature = "server")]
pub async fn run_cleanup_job() {
    start_cleanup_job().await.unwrap();
}