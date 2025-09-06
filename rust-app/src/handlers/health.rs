use std::sync::Arc;

use axum::{http::StatusCode, Extension};
use sea_orm::{ConnectionTrait, DatabaseConnection};

pub async fn get_health(Extension(pool): Extension<Arc<DatabaseConnection>>) -> Result<StatusCode, StatusCode> {

    pool
        .execute(
        sea_orm::Statement::from_string(
                pool.get_database_backend(),
                "SELECT 1".to_owned()
            )
        )
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    Ok(StatusCode::OK)
}
