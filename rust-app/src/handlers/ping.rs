use std::sync::Arc;

use axum::Extension;
use sea_orm::sqlx::types::chrono::Utc;
use sea_orm::DatabaseConnection;
use serde_json::json;
use uuid::Uuid;
use axum::{http::StatusCode};

use sea_orm::ColumnTrait;
use sea_orm::EntityTrait;
use sea_orm::QueryFilter;
use sea_orm::ActiveModelTrait;
use sea_orm::ActiveValue::Set;

use crate::models::ping;

pub async fn post_ping(Extension(pool): Extension<Arc<DatabaseConnection>>) -> Result<(StatusCode, String), (StatusCode, String)> {
    let ping = ping::Entity::find()
        .filter(ping::Column::Service.eq("rust"))
        .one(pool.as_ref())
        .await
        .map_err(|err| {
            (
                StatusCode::INTERNAL_SERVER_ERROR,
                json!({ "error": "error on getting record", "log": err.to_string() }).to_string()
            )
        })?;

    let result = match ping {
        Some(record) => {
            let mut ping: ping::ActiveModel = record.into();

            ping.amount = Set(ping.amount.as_ref() + 1);
            ping.updated_at = Set(Some(Utc::now().naive_utc()));

            let result = ping
                .update(pool.as_ref())
                .await
                .map_err(|err| {
                    (
                        StatusCode::INTERNAL_SERVER_ERROR,
                        json!({ "error": "error on creating record", "log": err.to_string() }).to_string()
                    )
                })?;

            (StatusCode::OK, json!(result).to_string())
        },
        None => {
            let id = format!("{}", Uuid::now_v7());

            let ping = ping::ActiveModel {
                id: Set(id),
                amount: Set(1),
                service:  Set("rust".to_string()),
                ..Default::default()
            };

            let result = ping
                .insert(pool.as_ref())
                .await
                .map_err(|err| {
                    (
                        StatusCode::INTERNAL_SERVER_ERROR,
                        json!({ "error": "error on updating record", "log": err.to_string() }).to_string()
                    )
                })?;


            (StatusCode::CREATED, json!(result).to_string())
        },
    };

    Ok(result)
}
