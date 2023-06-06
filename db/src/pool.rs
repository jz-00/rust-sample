use sqlx::postgres::PgPool;

pub type Pool = PgPool;
use sqlx::pool::PoolOptions;

pub async fn create_default_pool() -> Pool {
    create_pool(10).await
}

pub async fn create_pool(max_connections: u32) -> Pool {
    let database_url = std::env::var("DATABASE_URL").expect("DATABASE_URL must be set");

    PoolOptions::new()
        .max_connections(max_connections)
        .connect(&database_url)
        .await
        .expect("database connection error")
}
