use axum::{
    extract::{Path, Query, State},
    http::StatusCode,
    Json,
};
use serde_json::Value;
use sqlx::{Pool, Postgres};

use super::Pagination::{Pagination, RowCount};

#[derive(serde::Deserialize, serde::Serialize)]
pub struct NewPerson {
    pub person_name: String,
    pub age: i32,
    pub email: String,
}

#[derive(serde::Deserialize, serde::Serialize)]
pub struct Person {
    id: i64,
    person_name: String,
    age: i32,
    email: String,
}

pub async fn get_person_by_id(
    Path(user_id): Path<i64>,
    State(database_connection_pool): State<Pool<Postgres>>,
) -> (StatusCode, Json<Value>) {
    let query_result = sqlx::query_as!(Person, "SELECT * FROM PERSON WHERE ID = $1", user_id)
        .fetch_one(&database_connection_pool)
        .await;
    match query_result {
        Ok(person) => {
            return (StatusCode::OK, Json(serde_json::json!(person)));
        }
        Err(error) => {
            return (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(serde_json::json!({ "error": error.to_string() })),
            );
        }
    }
}

pub async fn list_person(
    maybe_pagination: Option<Query<Pagination>>,
    State(database_connection_pool): State<Pool<Postgres>>,
) -> (StatusCode, Json<Value>) {
    let Query(pagination) = maybe_pagination.unwrap_or_default();
    let size = i64::from(pagination.size.unwrap_or(10));
    let offset = size * pagination.page.unwrap_or(0);
    let query_result = sqlx::query_as!(
        Person,
        "SELECT * FROM PERSON LIMIT $1 OFFSET $2",
        size,
        offset,
    )
    .fetch_all(&database_connection_pool)
    .await;

    let row_query_result = sqlx::query_as!(RowCount, "SELECT COUNT(1) from PERSON")
        .fetch_one(&database_connection_pool)
        .await;

    match (query_result, row_query_result) {
        (Ok(person_vec), Ok(row_query)) => {
            return (
                StatusCode::OK,
                Json(
                    serde_json::json!({"total":row_query.count.unwrap_or_default(), "hits":person_vec}),
                ),
            );
        }
        (Err(error), Ok(_)) | (Ok(_), Err(error)) => {
            return (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(serde_json::json!({"error":error.to_string()})),
            );
        }
        (Err(error), Err(error_count)) => {
            return (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(serde_json::json!({"errors":[error.to_string(), error_count.to_string()]})),
            );
        }
    }
}

pub async fn insert_person(
    State(database_connection_pool): State<Pool<Postgres>>,
    Json(new_person_json): Json<NewPerson>,
) -> (StatusCode, Json<Value>) {
    let query_result = sqlx::query_as!(
                Person,
                "INSERT INTO PERSON ( PERSON_NAME, AGE, EMAIL ) VALUES ( $1, $2, $3 ) RETURNING ID, PERSON_NAME, AGE, EMAIL",
                new_person_json.person_name,
                new_person_json.age,
                new_person_json.email
            )
            .fetch_one(&database_connection_pool)
            .await;
    match query_result {
        Ok(person) => {
            return (StatusCode::CREATED, Json(serde_json::json!(person)));
        }
        Err(json_error) => {
            return (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(serde_json::json!({"error":json_error.to_string()})),
            );
        }
    }
}
