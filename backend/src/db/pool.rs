/*
PostgreSQL 数据库连接池
*/
use sqlx::{Pool, Postgres, postgres::PgPoolOptions};
use std::time::Duration;

pub type DbPool = Pool<Postgres>;

// postgres://mytodoapp:mytodoapp@localhost:5432/mytodoapp_db
pub const MY_TODOAPP_DATABASE_URL: &str =
    "postgres://mytodoapp:mytodoapp@localhost:5432/mytodoapp_db";

pub async fn create_pool() -> Result<DbPool, sqlx::Error> {
    let database_url = MY_TODOAPP_DATABASE_URL;

    let pool = PgPoolOptions::new()
        .max_connections(20)
        .acquire_timeout(Duration::from_secs(30))
        .idle_timeout(Duration::from_secs(300))
        .max_lifetime(Duration::from_secs(1800))
        .connect(&database_url)
        .await?;
    tracing::info!("数据库连接池创建成功");

    Ok(pool)
}

// 测试连接
pub async fn _test_connection(pool: &DbPool) -> Result<(), sqlx::Error> {
    sqlx::query("SELECT 1").fetch_one(pool).await?;
    Ok(())
}
