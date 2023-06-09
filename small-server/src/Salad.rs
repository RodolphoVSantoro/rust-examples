use axum::{
    extract::{rejection::JsonRejection, Path, Query, State},
    http::StatusCode,
    routing::{get, post},
    Json, Router,
};
use serde_json::Value;
use sqlx::{Pool, Postgres};

use super::Pagination::{Pagination, RowCount};

#[derive(serde::Deserialize, serde::Serialize)]
pub struct NewFruitSalad {
    pub id_creator: i64,
    pub salad_name: String,
}

#[derive(serde::Deserialize, serde::Serialize)]
pub struct FruitSalad {
    pub id: i64,
    pub id_creator: i64,
    pub salad_name: String,
}

#[derive(serde::Serialize)]
pub struct SaladView {
    pub id: i64,
    pub person_name: String,
    pub salad_name: String,
}

#[derive(serde::Serialize)]
pub struct SaladIngredientsView {
    pub person_name: String,
    pub salad_name: String,
    pub fruit_name: String,
}

pub fn get_router() -> Router<Pool<Postgres>> {
    return Router::new()
        .route("/", post(insert_salad))
        .route("/", get(list_salad))
        .route("/:salad_id", get(get_salad_by_id))
        .route("/:salad_id/ingredients", get(list_salad_all_ingredients));
}

pub async fn get_salad_by_id(
    Path(salad_id): Path<i64>,
    State(database_connection_pool): State<Pool<Postgres>>,
) -> (StatusCode, Json<Value>) {
    let query_result = sqlx::query_as!(
        FruitSalad,
        "SELECT * FROM FRUIT_SALAD WHERE ID = $1",
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

pub async fn list_salads_by_user_id(
    maybe_pagination: Option<Query<Pagination>>,
    Path(user_id): Path<i64>,
    State(database_connection_pool): State<Pool<Postgres>>,
) -> (StatusCode, Json<Value>) {
    let pagination = maybe_pagination.unwrap_or_default();
    let size = i64::from(pagination.size.unwrap_or(10));
    let offset = size * pagination.page.unwrap_or(0);
    let query_result = sqlx::query_as!(
        SaladView,
        r#"
        SELECT FRUIT_SALAD.id, person_name, salad_name FROM FRUIT_SALAD 
        JOIN PERSON ON ID_CREATOR = PERSON.ID 
        where ID_CREATOR = $3
        LIMIT $1 OFFSET $2
        "#,
        size,
        offset,
        user_id
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

pub async fn list_salad(
    maybe_pagination: Option<Query<Pagination>>,
    State(database_connection_pool): State<Pool<Postgres>>,
) -> (StatusCode, Json<Value>) {
    let pagination = maybe_pagination.unwrap_or_default();
    let size = i64::from(pagination.size.unwrap_or(10));
    let offset = size * pagination.page.unwrap_or(0);
    let query_result = sqlx::query_as!(
        FruitSalad,
        r#"
        SELECT * FROM FRUIT_SALAD
        LIMIT $1 OFFSET $2
        "#,
        size,
        offset,
    )
    .fetch_all(&database_connection_pool)
    .await;

    let row_query_result = sqlx::query_as!(RowCount, "SELECT COUNT(1) from FRUIT_SALAD")
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

pub async fn list_salad_all_ingredients(
    maybe_pagination: Option<Query<Pagination>>,
    Path(salad_id): Path<i64>,
    State(database_connection_pool): State<Pool<Postgres>>,
) -> (StatusCode, Json<Value>) {
    let pagination = maybe_pagination.unwrap_or_default();
    let size = i64::from(pagination.size.unwrap_or(10));
    let offset = size * pagination.page.unwrap_or(0);
    let query_result = sqlx::query_as!(
        SaladIngredientsView,
        r#"
        SELECT person_name, salad_name, fruit_name FROM FRUIT_SALAD 
        JOIN PERSON ON ID_CREATOR = PERSON.ID 
        JOIN SALAD_INGREDIENTS ON ID_SALAD = FRUIT_SALAD.ID
        JOIN FRUIT ON SALAD_INGREDIENTS.ID_FRUIT = FRUIT.ID
        where FRUIT_SALAD.ID = $3
        LIMIT $1 OFFSET $2
        "#,
        size,
        offset,
        salad_id
    )
    .fetch_all(&database_connection_pool)
    .await;

    let row_query_result = sqlx::query_as!(
        RowCount,
        r#"
        SELECT COUNT(1) from FRUIT_SALAD 
        JOIN PERSON ON ID_CREATOR = PERSON.ID 
        JOIN SALAD_INGREDIENTS ON ID_SALAD = FRUIT_SALAD.ID
        JOIN FRUIT ON SALAD_INGREDIENTS.ID_FRUIT = FRUIT.ID 
        where FRUIT_SALAD.ID = $1
        "#,
        salad_id
    )
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
    body: Result<Json<NewFruitSalad>, JsonRejection>,
) -> (StatusCode, Json<Value>) {
    match body {
        Ok(salad_json) => {
            let query_result = sqlx::query_as!(
                FruitSalad,
                r#"
                INSERT INTO FRUIT_SALAD ( ID_CREATOR, SALAD_NAME ) 
                VALUES ( $1, $2 ) 
                RETURNING ID, ID_CREATOR, SALAD_NAME
                "#,
                salad_json.id_creator,
                salad_json.salad_name
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
