#![allow(non_camel_case_types, non_snake_case, dead_code, unused_variables, unused_mut, unused_imports)]
use salvo::oapi::extract::*;
use salvo::prelude::*;

mod models;
mod db;
use db::migrations::init_database;
use db::pool::create_pool;

mod middleware;
use middleware::logging::{logger, request_logger};

mod utils;

mod handlers;
mod routes;
mod services;

use routes::{user_routes, task_routes};

#[endpoint]
async fn hello(name: QueryParam<String, false>) -> String {
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

    let router = Router::new()
        .push(Router::with_path("hello").get(hello))
        .push(user_routes::user_router())
        .push(task_routes::task_router());

    let doc = OpenApi::new("test api", "0.0.1").merge_router(&router);

    let router = router
        .unshift(doc.into_router("/api-doc/openapi.json"))
        .unshift(SwaggerUi::new("/api-doc/openapi.json").into_router("/swagger-ui"));

    let acceptor = TcpListener::new("localhost:8698").bind().await;
    Server::new(acceptor).serve(router).await;
}

async fn run_database() -> Result<(), Box<dyn std::error::Error>> {
    let _pool = init_database().await?;
    create_pool().await?;
    Ok(())
}
