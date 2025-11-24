mod models;
mod handlers;

use axum::{
    Router,
    serve,
};
use axum::response::Redirect;
#[allow(unused_imports)]
use axum::routing::{get, post, put, delete};
use utoipa_swagger_ui::SwaggerUi;
use crate::models::AppState;
use crate::handlers::*;
use utoipa::OpenApi;
use tokio::net::TcpListener;

async fn index() -> Redirect {
    Redirect::temporary("/docs")
}

#[tokio::main]
async fn main() {
    let state = AppState::default();

    let api_routes = Router::new()
        .route("/health", get(health))
        .route("/all", get(list_all).post(create_all))
        .route("/all/{id}", get(get_all).put(update_all).delete(delete_all))
        .with_state(state.clone());

    let app = Router::new()
        .route("/", get(index)) // <-- root
        .merge(api_routes)
        .merge(
            SwaggerUi::new("/docs")
                .url("/api-doc/openapi.json", ApiDoc::openapi())
        );

    let listener = TcpListener::bind("0.0.0.0:3000").await.unwrap();
    println!("Server in ascolto su http://{}", listener.local_addr().unwrap());
    serve(listener, app).await.unwrap();
}
