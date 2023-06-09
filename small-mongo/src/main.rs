mod errors;
mod fruit;

use crate::errors::UnwrapPrint;
use crate::fruit::Fruit;
use axum::Router;
use std::net::SocketAddr;
use wither::mongodb::{Client, Database};
use wither::prelude::Model;

#[tokio::main]
async fn main() {
    dotenv::dotenv().expect("Failed to read environment file");

    let mongo_url = std::env::var("MONGO_URL").expect("Failed to read MONGO_URL from environment");

    let client = Client::with_uri_str(&mongo_url)
        .await
        .expect("Failed to connect to MongoDB");

    let database_name =
        std::env::var("DATABASE_NAME").expect("Failed to read DATABASE_NAME from environment");

    let database = client.database(&database_name);

    sync_models(&database).await.expect("Failed to sync models");

    let app = Router::new()
        .nest("/", fruit::get_router())
        .with_state(database);

    let port = get_server_socket_addr().unwrap_print();
    println!("Server starting on {}", &port);
    axum::Server::bind(&port)
        .serve(app.into_make_service())
        .await
        .expect("Failed to start server");
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

async fn sync_models(database: &Database) -> Result<(), wither::WitherError> {
    let result = Fruit::sync(database).await;
    if let Err(error) = result {
        return Err(error);
    }
    return Ok(());
}
