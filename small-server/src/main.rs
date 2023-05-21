use axum::{extract::Path, http::StatusCode, response::Json, routing::get, Router};
use serde_json::Value;
use std::net::SocketAddr;

async fn get_person_by_id(Path(user_id): Path<String>) -> Json<Value> {
    return Json(serde_json::json!({ "id": user_id }));
}

async fn insert_person() -> (StatusCode, Json<Value>) {
    return (
        StatusCode::CREATED,
        Json(serde_json::json!({
            "message": "hi",
        })),
    );
}

#[tokio::main]
async fn main() {
    let app = Router::new()
        .route("/person/:user_id", get(get_person_by_id))
        .route("/hello", get(insert_person));

    let port: SocketAddr = "127.0.0.1:3000".parse().unwrap();
    axum::Server::bind(&port)
        .serve(app.into_make_service())
        .await
        .unwrap();
}
