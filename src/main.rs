use axum::{
    http::{header, HeaderValue, StatusCode},
    routing::{get, post},
    Router,
};
use dotenv::dotenv;
use futures::stream::StreamExt;
use mongodb::{options::ClientOptions, options::FindOptions, Client, Collection, Database};
use serde::{Deserialize, Serialize};
use std::net::SocketAddr;
use std::time::Duration;
mod handlers;
mod structs;
use handlers::recipe_handler::{
    get_all_categories, get_all_recipe, get_categorie_by_id, get_recipe_by_slug,
};
use mongodb::bson::oid::ObjectId;
use structs::comman::DatabaseConfig;
use tower_http::{
    limit::RequestBodyLimitLayer, set_header::SetResponseHeaderLayer, timeout::TimeoutLayer,
    trace::TraceLayer,
};
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};
#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    dotenv().ok();
    tracing_subscriber::registry()
        .with(tracing_subscriber::EnvFilter::new(
            std::env::var("RUST_LOG").unwrap_or_else(|_| {
                "rust_axum=debug,axum=debug,tower_http=debug,mongodb=debug".into()
            }),
        ))
        .with(tracing_subscriber::fmt::layer())
        .init();
    let database_config = DatabaseConfig::new();
    let mut client_options = ClientOptions::parse(database_config.uri).await.unwrap();
    client_options.connect_timeout = database_config.connection_timeout;
    client_options.max_pool_size = database_config.max_pool_size;
    client_options.min_pool_size = database_config.min_pool_size;
    client_options.compressors = database_config.compressors;
    let client = Client::with_options(client_options).unwrap();
    let app = Router::new()
        .route("/", get(get_all_categories))
        .route("/categories/:id", get(get_categorie_by_id))
        .route("/recipes", get(get_all_recipe))
        .route("/recipes/getRecipeBySlug/:slug", get(get_recipe_by_slug))
        .with_state(client);
    axum::Server::bind(&SocketAddr::from(([127, 0, 0, 1], 8080)))
        .serve(app.into_make_service())
        .await
        .unwrap();
    Ok(())
}
