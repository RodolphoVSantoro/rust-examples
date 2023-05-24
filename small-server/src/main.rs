use axum::{
    extract::Path,
    extract::{rejection::JsonRejection, State},
    http::StatusCode,
    response::Json,
    routing::{get, post},
    Router,
};
use dotenv::dotenv;
use serde_json::Value;
use sqlx::{postgres::PgPoolOptions, Pool, Postgres};
use std::{net::SocketAddr, time::Duration};

#[derive(serde::Deserialize, serde::Serialize)]
struct NewPerson {
    person_name: String,
    age: i32,
    email: String,
}

#[derive(serde::Deserialize, serde::Serialize)]
struct Person {
    id: i64,
    person_name: String,
    age: i32,
    email: String,
}

async fn get_person_by_id(
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

async fn insert_person(
    State(database_connection_pool): State<Pool<Postgres>>,
    body: Json<NewPerson>,
) -> (StatusCode, Json<Value>) {
    let person_json = body;
    let query_result = sqlx::query_as!(
                Person,
                "INSERT INTO PERSON ( PERSON_NAME, AGE, EMAIL ) VALUES ( $1, $2, $3 ) RETURNING ID, PERSON_NAME, AGE, EMAIL",
                person_json.person_name,
                person_json.age,
                person_json.email
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

#[derive(serde::Deserialize, serde::Serialize)]
struct Fruit {
    id: i64,
    fruit_name: String,
    color_red: i16,
    color_green: i16,
    color_blue: i16,
    fruit_weight: i32,
}

async fn insert_fruit(
    State(database_connection_pool): State<Pool<Postgres>>,
    body: Result<Json<Fruit>, JsonRejection>,
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
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(serde_json::json!({"error":json_error.to_string()})),
            );
        }
    }
}

#[derive(serde::Deserialize, serde::Serialize)]
struct FavoriteSalads {
    id_salad: i64,
    id_creator: i64,
    id_fruit: i64,
}

async fn insert_salad(
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

#[tokio::main]
async fn main() {
    dotenv().expect("Failed to read environment file");
    let database_url: String =
        std::env::var("DATABASE_URL").expect("Unable to read DATABASE_URL env var");
    let database_connection_pool = PgPoolOptions::new()
        .max_connections(10)
        .acquire_timeout(Duration::from_secs(10))
        .connect(&database_url)
        .await
        .expect("Could not connect to Database");

    let app = Router::new()
        .route("/person/:user_id", get(get_person_by_id))
        .route("/person", post(insert_person))
        .route("/fruit", post(insert_fruit))
        .with_state(database_connection_pool);

    let port: SocketAddr = "127.0.0.1:3000".parse().unwrap();
    axum::Server::bind(&port)
        .serve(app.into_make_service())
        .await
        .unwrap();
}
