use sqlx::{postgres::PgPoolOptions, PgPool};
use tracing::info;

pub async fn setup_db_pool(database_url: &str) -> Result<PgPool, sqlx::Error> {
    let pool = PgPoolOptions::new()
        .max_connections(5)
        .connect(database_url)
        .await?;

    // Optional: Run migrations if you have them
    // sqlx::migrate!("./migrations")
    //     .run(&pool)
    //     .await?;
    // info!("Database migrations applied.");

    Ok(pool)
}
