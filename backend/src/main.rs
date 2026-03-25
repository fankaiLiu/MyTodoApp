use salvo::oapi::extract::*;
use salvo::prelude::*;

mod models;
// use models::user;
mod db;
use db::migrations::init_database;
use db::pool::create_pool;

// use crate::utils::id_generator::test_sonyflake_id;
mod utils;

#[endpoint]
async fn hello(name: QueryParam<String, false>) -> String {
    // format!("Hello, {}!", name.as_deref().unwrap_or("World"))
    format!("Hello, {}!", name.as_deref().unwrap_or("World"))
}

#[tokio::main]
async fn main() {
    tracing_subscriber::fmt().init();

    if let Err(e) = run_database().await {
        tracing::error!("数据库初始化失败: {:?}", e);
        return;
    }

    if let Err(e) = utils::id_generator::test_sonyflake_id() {
        tracing::error!("测试sonflake id失败: {:?}", e);
        return;
    }

    let router = Router::new().push(Router::with_path("hello").get(hello));

    let doc = OpenApi::new("test api", "0.0.1").merge_router(&router);

    let router = router
        .unshift(doc.into_router("/api-doc/openapi.json"))
        .unshift(SwaggerUi::new("/api-doc/openapi.json").into_router("/swagger-ui"));

    // let acceptor = TcpListener::new("0.0.0.0:8698").bind().await;
    let acceptor = TcpListener::new("localhost:8698").bind().await;
    Server::new(acceptor).serve(router).await;
}

async fn run_database() -> Result<(), Box<dyn std::error::Error>> {
    let _pool = init_database().await?;
    create_pool().await?;
    Ok(())
}
