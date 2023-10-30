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
mod structs;
use structs::comman::DatabaseConfig;
use tower_http::{
    limit::RequestBodyLimitLayer, set_header::SetResponseHeaderLayer, timeout::TimeoutLayer,
    trace::TraceLayer,
};

use mongodb::bson::oid::ObjectId;
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
    println!(
        "connected to database {}",
        client.database("Recipes").name()
    );
    // let db = client.database("recipes");
    // for collection_name in db.list_collection_names(None).await? {
    //     println!("{}", collection_name);
    // }
    let app = Router::new()
        .route("/", get(health_check))
        .with_state(client);
    axum::Server::bind(&SocketAddr::from(([127, 0, 0, 1], 8080)))
        .serve(app.into_make_service())
        .await
        .unwrap();
    Ok(())
}

#[derive(Serialize, Debug, Deserialize, Clone)]
pub struct MyDocument {
    #[serde(rename = "_id")]
    pub id: ObjectId,
    pub description: String,
    pub img_Base64: String,
    pub isActive: bool,
    pub name: String,
    pub slug: String,
    pub subName: String,
    pub createdAt: String,
    pub updatedAt: String,
    pub __v: i32,
}

use serde_with::skip_serializing_none;
#[skip_serializing_none]
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Response {
    pub success: bool,
    pub data: Option<Vec<MyDocument>>,
    pub error_message: Option<String>,
}
pub async fn health_check(
    axum::extract::State(client): axum::extract::State<Client>,
) -> impl axum::response::IntoResponse {
    let client = client.clone();
    println!("{}", client.database("Recipes").name());
    let db = client.database("Recipe");
    //categories
    let category_collection: Collection<MyDocument> = db.collection::<MyDocument>("categories");
    let mut option = FindOptions::default();
    let mut category_cursor = category_collection
        .find(None, None)
        .await
        .expect("Could not find categories");
    let mut categories: Vec<MyDocument> = Vec::new();
    while let Some(category) = category_cursor.next().await {
        println!("{:#?}", category);
        match category {
            Ok(category) => {
                categories.push(category);
            }
            Err(err) => {
                println!("{:#?}", err);
            }
        }
    }
    println!("{:#?}", categories);
    let response = Response {
        success: true,
        data: Some(categories),
        error_message: None,
    };
    (StatusCode::OK, axum::Json(response))
}
