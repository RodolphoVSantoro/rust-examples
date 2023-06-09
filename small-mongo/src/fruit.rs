use axum::{
    extract::{rejection::JsonRejection, State},
    http::StatusCode,
    routing::post,
    Json, Router,
};
use serde::{Deserialize, Serialize};
use serde_json::Value;
use wither::bson::{doc, oid::ObjectId};
use wither::mongodb::Database;
use wither::prelude::Model;

#[derive(serde::Deserialize)]
pub struct NewFruit {
    pub fruit_name: String,
    pub color_red: i16,
    pub color_green: i16,
    pub color_blue: i16,
    pub fruit_weight: i32,
}

#[derive(Debug, Model, Serialize, Deserialize)]
#[model(index(keys = r#"doc!{"email": 1}"#, options = r#"doc!{"unique": true}"#))]
pub struct Fruit {
    #[serde(rename = "_id", skip_serializing_if = "Option::is_none")]
    pub id: Option<ObjectId>,
    pub fruit_name: String,
    pub color_red: i16,
    pub color_green: i16,
    pub color_blue: i16,
    pub fruit_weight: i32,
}

pub fn get_router() -> Router<Database> {
    return Router::new().route("/fruit", post(insert_fruit));
}

pub async fn insert_fruit(
    State(database): State<Database>,
    body: Result<Json<NewFruit>, JsonRejection>,
) -> (StatusCode, Json<Value>) {
    let new_fruit = match body {
        Ok(body) => body,
        Err(json_error) => {
            return (
                json_error.status(),
                Json(serde_json::json!({"error":json_error.to_string()})),
            );
        }
    };

    let mut fruit = Fruit {
        id: Some(ObjectId::new()),
        fruit_name: new_fruit.fruit_name.to_owned(),
        color_red: new_fruit.color_red,
        color_green: new_fruit.color_green,
        color_blue: new_fruit.color_blue,
        fruit_weight: new_fruit.fruit_weight,
    };

    let result = fruit.save(&database, None).await;
    if let Err(error) = result {
        return (
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(serde_json::json!({"error":error.to_string()})),
        );
    }

    return (StatusCode::CREATED, Json(serde_json::json!(fruit)));
}
