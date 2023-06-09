use axum::{
    extract::{rejection::JsonRejection, Path, Query, State},
    http::StatusCode,
    routing::{get, post},
    Json, Router,
};
use serde_json::Value;
use sqlx::{Pool, Postgres};

use super::Pagination::{Pagination, RowCount};

#[derive(serde::Deserialize)]
pub struct NewFruit {
    pub fruit_name: String,
    pub color_red: i16,
    pub color_green: i16,
    pub color_blue: i16,
    pub fruit_weight: i32,
}

#[derive(serde::Deserialize, serde::Serialize)]
pub struct Fruit {
    pub id: i64,
    pub fruit_name: String,
    pub color_red: i16,
    pub color_green: i16,
    pub color_blue: i16,
    pub fruit_weight: i32,
}

pub fn get_router() -> Router<Pool<Postgres>> {
    return Router::new()
        .route("/", post(insert_fruit))
        .route("/:fruit_id", get(get_fruit_by_id))
        .route("/", get(list_fruit));
}

pub async fn get_fruit_by_id(
    Path(fruit_id): Path<i64>,
    State(database_connection_pool): State<Pool<Postgres>>,
) -> (StatusCode, Json<Value>) {
    let query_result = sqlx::query_as!(Fruit, "SELECT * FROM FRUIT WHERE ID = $1", fruit_id)
        .fetch_one(&database_connection_pool)
        .await;
    match query_result {
        Ok(fruit) => {
            return (StatusCode::OK, Json(serde_json::json!(fruit)));
        }
        Err(error) => {
            return (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(serde_json::json!({ "error": error.to_string() })),
            );
        }
    }
}

pub async fn list_fruit(
    maybe_pagination: Option<Query<Pagination>>,
    State(database_connection_pool): State<Pool<Postgres>>,
) -> (StatusCode, Json<Value>) {
    let pagination = maybe_pagination.unwrap_or_default();
    let size = i64::from(pagination.size.unwrap_or(10));
    let offset = size * pagination.page.unwrap_or(0);
    let query_result = sqlx::query_as!(
        Fruit,
        "SELECT * FROM FRUIT LIMIT $1 OFFSET $2",
        size,
        offset,
    )
    .fetch_all(&database_connection_pool)
    .await;

    let row_query_result = sqlx::query_as!(RowCount, "SELECT COUNT(1) from FRUIT")
        .fetch_one(&database_connection_pool)
        .await;

    let Ok(row_count) = row_query_result else {
        return (
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(serde_json::json!({"error": row_query_result.err().unwrap().to_string()})),
        );
    };

    match query_result {
        Ok(fruit_vec) => {
            return (
                StatusCode::OK,
                Json(serde_json::json!({"total":row_count.count,"hits":fruit_vec})),
            );
        }
        Err(error) => {
            return (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(serde_json::json!({"error":error.to_string()})),
            );
        }
    }
}

pub async fn insert_fruit(
    State(database_connection_pool): State<Pool<Postgres>>,
    body: Result<Json<NewFruit>, JsonRejection>,
) -> (StatusCode, Json<Value>) {
    match body {
        Ok(fruit_json) => {
            let query_result = sqlx::query_as!(
                Fruit,
                r#"
                INSERT INTO FRUIT ( FRUIT_NAME, COLOR_RED, COLOR_GREEN, COLOR_BLUE, FRUIT_WEIGHT ) 
                VALUES ( $1, $2, $3, $4, $5 ) 
                RETURNING ID, FRUIT_NAME, COLOR_RED, COLOR_GREEN, COLOR_BLUE, FRUIT_WEIGHT
                "#,
                fruit_json.fruit_name,
                fruit_json.color_red,
                fruit_json.color_green,
                fruit_json.color_blue,
                fruit_json.fruit_weight
            )
            .fetch_one(&database_connection_pool)
            .await;
            match query_result {
                Ok(fruit) => {
                    return (StatusCode::CREATED, Json(serde_json::json!(fruit)));
                }
                Err(json_error) => {
                    return (
                        StatusCode::INTERNAL_SERVER_ERROR,
                        Json(serde_json::json!({"error":json_error.to_string()})),
                    );
                }
            }
        }
        Err(json_error) => {
            return (
                json_error.status(),
                Json(serde_json::json!({"error":json_error.to_string()})),
            );
        }
    }
}
