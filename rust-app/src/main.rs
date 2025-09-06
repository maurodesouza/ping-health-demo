mod models;
mod handlers;

use std::{env, sync::Arc};

use sea_orm::{ConnectOptions, Database};
use axum::{routing::{get, post}, Extension, Router};


#[tokio::main]
async fn main() -> Result<(), sea_orm::DbErr> {
    let port =  env::var("PORT").unwrap_or("9090".to_string());
    let database_url = env::var("DATABASE_URL").expect("Database URL wasn't specified");

    let connect_option = ConnectOptions::new(&database_url);
    let pool: sea_orm::DatabaseConnection = Database::connect(connect_option).await?;

    let pool = Arc::new(pool);

    let app = Router::new()
        .route("/ping", post(handlers::ping::post_ping))
        .route("/health", get(handlers::health::get_health))
        .layer(Extension(pool.clone()));

    let listener = tokio::net::TcpListener::bind(format!("0.0.0.0:{}", port)).await.unwrap();

    println!("🚀 server is running at port: {}", port);

    axum::serve(listener, app).await.unwrap();

    Ok(())
}
