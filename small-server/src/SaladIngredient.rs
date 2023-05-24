use axum::{
    extract::{rejection::JsonRejection, Path, Query, State},
    http::StatusCode,
    Json,
};
use serde_json::Value;
use sqlx::{Pool, Postgres};

use super::Pagination::{Pagination, RowCount};

#[derive(serde::Deserialize, serde::Serialize)]
pub struct NewSaladIngredient {
    pub id_salad: i64,
    pub id_fruit: i64,
}

#[derive(serde::Deserialize, serde::Serialize)]
pub struct SaladIngredient {
    pub id: i64,
    pub id_salad: i64,
    pub id_fruit: i64,
}

pub async fn get_salad_ingredient_by_id(
    Path(salad_ingredient_id): Path<i64>,
    State(database_connection_pool): State<Pool<Postgres>>,
) -> (StatusCode, Json<Value>) {
    let query_result = sqlx::query_as!(
        SaladIngredient,
        "SELECT * FROM SALAD_INGREDIENTS WHERE ID = $1",
        salad_ingredient_id
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

pub async fn list_salad_ingredients(
    maybe_pagination: Option<Query<Pagination>>,
    State(database_connection_pool): State<Pool<Postgres>>,
) -> (StatusCode, Json<Value>) {
    let pagination = maybe_pagination.unwrap_or_default();
    let size = i64::from(pagination.size.unwrap_or(10));
    let offset = size * pagination.page.unwrap_or(0);
    let query_result = sqlx::query_as!(
        SaladIngredient,
        r#"
        SELECT * FROM SALAD_INGREDIENTS 
        LIMIT $1 OFFSET $2
        "#,
        size,
        offset,
    )
    .fetch_all(&database_connection_pool)
    .await;

    let row_query_result = sqlx::query_as!(RowCount, "SELECT COUNT(1) from SALAD_INGREDIENTS")
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

pub async fn insert_salad_ingredient(
    State(database_connection_pool): State<Pool<Postgres>>,
    body: Result<Json<NewSaladIngredient>, JsonRejection>,
) -> (StatusCode, Json<Value>) {
    match body {
        Ok(ingredient_json) => {
            let query_result = sqlx::query_as!(
                SaladIngredient,
                r#"
                INSERT INTO SALAD_INGREDIENTS ( ID_SALAD, ID_FRUIT ) 
                VALUES ( $1, $2 ) 
                RETURNING ID, ID_SALAD, ID_FRUIT
                "#,
                ingredient_json.id_salad,
                ingredient_json.id_fruit,
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
