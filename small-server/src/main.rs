use axum::{
    routing::{get, post},
    Router,
};
use dotenv::dotenv;
use sqlx::postgres::PgPoolOptions;
use std::{net::SocketAddr, time::Duration};

#[allow(non_snake_case)]
mod Fruit;
#[allow(non_snake_case)]
mod Pagination;
#[allow(non_snake_case)]
mod Person;
#[allow(non_snake_case)]
mod Salad;

#[allow(non_snake_case)]
mod SaladIngredient;

use Fruit::{get_fruit_by_id, insert_fruit, list_fruit};
use Person::{get_person_by_id, insert_person, list_person};
use Salad::{
    get_salad_by_id, insert_salad, list_salad, list_salad_all_ingredients, list_salads_by_user_id,
};
use SaladIngredient::{
    get_salad_ingredient_by_id, insert_salad_ingredient, list_salad_ingredients,
};

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

    let person_router = Router::new()
        .route("/", post(insert_person))
        .route("/:user_id", get(get_person_by_id))
        .route("/:user_id/salad", get(list_salads_by_user_id))
        .route("/", get(list_person));

    let fruit_router = Router::new()
        .route("/", post(insert_fruit))
        .route("/:fruit_id", get(get_fruit_by_id))
        .route("/", get(list_fruit));

    let salad_router = Router::new()
        .route("/", post(insert_salad))
        .route("/", get(list_salad))
        .route("/:salad_id", get(get_salad_by_id))
        .route("/:salad_id/ingredients", get(list_salad_all_ingredients));

    let ingredient_router = Router::new()
        .route("/", post(insert_salad_ingredient))
        .route("/", get(list_salad_ingredients))
        .route("/:ingredient_id", get(get_salad_ingredient_by_id));

    let app = Router::new()
        .nest("/person", person_router)
        .nest("/fruit", fruit_router)
        .nest("/salad", salad_router)
        .nest("/ingredient", ingredient_router)
        .with_state(database_connection_pool);

    let port: SocketAddr = "127.0.0.1:3000".parse().unwrap();
    axum::Server::bind(&port)
        .serve(app.into_make_service())
        .await
        .unwrap();
}
