use axum::{
    extract::{rejection::JsonRejection, Path, Query, State},
    http::StatusCode,
    Json,
};
use serde_json::Value;
use sqlx::{Pool, Postgres};

use super::Pagination::{Pagination, RowCount};

#[derive(serde::Deserialize, serde::Serialize)]
pub struct FavoriteSalads {
    pub id_salad: i64,
    pub id_creator: i64,
    pub id_fruit: i64,
}

pub async fn get_salad_by_id(
    Path(salad_id): Path<i64>,
    State(database_connection_pool): State<Pool<Postgres>>,
) -> (StatusCode, Json<Value>) {
    let query_result = sqlx::query_as!(
        FavoriteSalads,
        "SELECT * FROM FAVORITE_SALADS WHERE ID_SALAD = $1",
        salad_id
    )
    .fetch_one(&database_connection_pool)
    .await;
    match query_result {
        Ok(salad) => {
            return (StatusCode::OK, Json(serde_json::json!(salad)));
        }
        Err(error) => {
            return (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(serde_json::json!({ "error": error.to_string() })),
            );
        }
    }
}

pub async fn list_salad(
    maybe_pagination: Option<Query<Pagination>>,
    State(database_connection_pool): State<Pool<Postgres>>,
) -> (StatusCode, Json<Value>) {
    let pagination = maybe_pagination.unwrap_or_default();
    let size = pagination.size.unwrap_or(10) as i64;
    let offset = size * pagination.page.unwrap_or(0);
    let query_result = sqlx::query_as!(
        FavoriteSalads,
        "SELECT * FROM FAVORITE_SALADS LIMIT $1 OFFSET $2",
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
        Ok(salad_vec) => {
            return (
                StatusCode::OK,
                Json(serde_json::json!({"total":row_count.count,"hits":salad_vec})),
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

pub async fn insert_salad(
    State(database_connection_pool): State<Pool<Postgres>>,
    body: Result<Json<FavoriteSalads>, JsonRejection>,
) -> (StatusCode, Json<Value>) {
    match body {
        Ok(salad_json) => {
            let query_result = sqlx::query_as!(
                FavoriteSalads,
                r#"
                INSERT INTO FAVORITE_SALADS ( ID_SALAD, ID_CREATOR, ID_FRUIT ) 
                VALUES ( $1, $2, $3 ) 
                RETURNING ID_SALAD, ID_CREATOR, ID_FRUIT
                "#,
                salad_json.id_salad,
                salad_json.id_creator,
                salad_json.id_fruit
            )
            .fetch_one(&database_connection_pool)
            .await;
            match query_result {
                Ok(salad) => {
                    return (StatusCode::CREATED, Json(serde_json::json!(salad)));
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
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(serde_json::json!({"error":json_error.to_string()})),
            );
        }
    }
}
