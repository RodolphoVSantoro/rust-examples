use crate::Errors::{DatabaseConnectionError, UnwrapPrint};
use axum::Router;
use sqlx::postgres::PgPoolOptions;
use sqlx::{Pool, Postgres};
use std::net::SocketAddr;
use std::time::Duration;

#[allow(non_snake_case)]
mod Errors;
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

async fn get_postgres_connection_pool() -> Result<Pool<Postgres>, Errors::DatabaseConnectionError> {
    let database_url_result = std::env::var("DATABASE_URL");

    let database_url = match database_url_result {
        Ok(database_url) => database_url,
        Err(error) => return Err(DatabaseConnectionError::VarError(error)),
    };

    let connection_result = PgPoolOptions::new()
        .max_connections(10)
        .acquire_timeout(Duration::from_secs(10))
        .connect(&database_url)
        .await;

    match connection_result {
        Ok(connection_pool) => return Ok(connection_pool),
        Err(error) => return Err(DatabaseConnectionError::ConnectionError(error)),
    }
}

fn get_server_socket_addr() -> Result<SocketAddr, std::env::VarError> {
    let address_result = std::env::var("SOCKET_ADDRESS");
    let address = match address_result {
        Ok(address) => address,
        Err(error) => return Err(error),
    };

    let address_result: Result<SocketAddr, _> = address.parse();
    let address = match address_result {
        Ok(address) => address,
        Err(_) => return Err(std::env::VarError::NotPresent),
    };
    return Ok(address);
}

#[tokio::main]
async fn main() {
    dotenv::dotenv().expect("Failed to read environment file");

    let database_connection_pool = get_postgres_connection_pool().await.unwrap_print();

    let app = Router::new()
        .nest("/person", crate::Person::get_router())
        .nest("/fruit", crate::Fruit::get_router())
        .nest("/salad", crate::Salad::get_router())
        .nest("/ingredient", crate::SaladIngredient::getRouter())
        .with_state(database_connection_pool);

    let port = get_server_socket_addr().unwrap_print();
    println!("Server starting on {}", &port);
    axum::Server::bind(&port)
        .serve(app.into_make_service())
        .await
        .expect("Failed to start server");
}
