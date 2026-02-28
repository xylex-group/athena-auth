mod models;
mod handlers;
mod middleware;
mod utils;

use axum::{
    middleware as axum_middleware,
    routing::{get, post, delete},
    Router,
};
use std::sync::Arc;
use tower_http::cors::{CorsLayer, Any};
use tower_http::trace::TraceLayer;
use athena_rs::AthenaClient;

pub struct AppState {
    pub client: AthenaClient,
}

#[tokio::main]
async fn main() {
    // Load environment variables
    dotenv::dotenv().ok();

    // Initialize tracing
    tracing_subscriber::fmt::init();

    // Get database configuration from environment
    let database_url = std::env::var("DATABASE_URL")
        .unwrap_or_else(|_| "postgresql://localhost/athena_auth".to_string());
    let database_key = std::env::var("DATABASE_KEY")
        .unwrap_or_else(|_| "".to_string());

    // Initialize Athena client
    let client = AthenaClient::new(database_url, database_key)
        .await
        .expect("Failed to create Athena client");

    let state = Arc::new(AppState { client });

    // Protected routes
    let protected_routes = Router::new()
        .route("/auth/verify", get(handlers::auth::verify))
        .route("/auth/me", get(handlers::auth::me))
        .route("/auth/api-keys", post(handlers::api_keys::create_api_key))
        .route("/auth/api-keys", get(handlers::api_keys::list_api_keys))
        .route("/auth/api-keys/:id", delete(handlers::api_keys::revoke_api_key))
        .layer(axum_middleware::from_fn(middleware::auth::auth_middleware));

    // Build application routes
    let app = Router::new()
        // Public routes
        .route("/auth/register", post(handlers::auth::register))
        .route("/auth/login", post(handlers::auth::login))
        // Merge protected routes
        .merge(protected_routes)
        .with_state(state)
        .layer(CorsLayer::new().allow_origin(Any).allow_methods(Any).allow_headers(Any))
        .layer(TraceLayer::new_for_http());

    // Get port from environment or use default
    let port = std::env::var("PORT")
        .unwrap_or_else(|_| "3000".to_string())
        .parse::<u16>()
        .unwrap_or(3000);

    let addr = format!("0.0.0.0:{}", port);
    let listener = tokio::net::TcpListener::bind(&addr)
        .await
        .expect("Failed to bind to address");

    tracing::info!("Server listening on {}", addr);

    axum::serve(listener, app)
        .await
        .expect("Failed to start server");
}
