use axum::{
    extract::{Path, State},
    response::IntoResponse,
    Json,
};
use crate::models::{AppState, All, CreateAll, UpdateAll};
use utoipa::OpenApi;

#[utoipa::path(
    get,
    path = "/health",
    responses((status = 200, description = "Service healthy"))
)]
pub async fn health() -> &'static str {
    "OK"
}

#[utoipa::path(
    get,
    path = "/all",
    responses((status = 200, description = "List All", body = [All]))
)]
pub async fn list_all(State(state): State<AppState>) -> impl IntoResponse {
    let alls = state.alls.lock().await.clone();
    Json(alls)
}

#[utoipa::path(
    post,
    path = "/all",
    request_body = CreateAll,
    responses((status = 201, description = "Created All", body = All))
)]
pub async fn create_all(State(state): State<AppState>, Json(payload): Json<CreateAll>) -> impl IntoResponse {
    let mut alls = state.alls.lock().await;
    let id = (alls.len() as u64) + 1;
    let all = All { id, title: payload.title, done: false };
    alls.push(all.clone());
    (axum::http::StatusCode::CREATED, Json(all))
}

#[utoipa::path(
    get,
    path = "/all/{id}",
    params(("id" = u64, Path, description = "All ID")),
    responses(
        (status = 200, description = "Found All", body = All),
        (status = 404, description = "All not found")
    )
)]
pub async fn get_all(State(state): State<AppState>, Path(id): Path<u64>) -> impl IntoResponse {
    let alls = state.alls.lock().await;
    match alls.iter().cloned().find(|t| t.id == id) {
        Some(all) => (axum::http::StatusCode::OK, Json(all)).into_response(),
        None => axum::http::StatusCode::NOT_FOUND.into_response(),
    }
}

#[utoipa::path(
    put,
    path = "/all/{id}",
    request_body = UpdateAll,
    params(("id" = u64, Path, description = "All ID")),
    responses(
        (status = 200, description = "Updated All", body = All),
        (status = 404, description = "All not found")
    )
)]
pub async fn update_all(
    State(state): State<AppState>,
    Path(id): Path<u64>,
    Json(payload): Json<UpdateAll>,
) -> impl IntoResponse {
    let mut alls = state.alls.lock().await;
    if let Some(all) = alls.iter_mut().find(|t| t.id == id) {
        if let Some(title) = payload.title { all.title = title; }
        if let Some(done) = payload.done { all.done = done; }
        return (axum::http::StatusCode::OK, Json(all.clone())).into_response();
    }
    axum::http::StatusCode::NOT_FOUND.into_response()
}

#[utoipa::path(
    delete,
    path = "/all/{id}",
    params(("id" = u64, Path, description = "All ID")),
    responses(
        (status = 204, description = "Deleted"),
        (status = 404, description = "All not found")
    )
)]
pub async fn delete_all(State(state): State<AppState>, Path(id): Path<u64>) -> impl IntoResponse {
    let mut alls = state.alls.lock().await;
    let len_before = alls.len();
    alls.retain(|t| t.id != id);
    if alls.len() < len_before {
        axum::http::StatusCode::NO_CONTENT
    } else {
        axum::http::StatusCode::NOT_FOUND
    }
}

// #[derive(OpenApi)]
// #[openapi(
//     paths(health, list_all, create_all, get_all, update_all, delete_all),
//     components(schemas(All, CreateAll, UpdateAll)),
//     tags((name = "all", description = "Operations on All"))
// )]
#[derive(OpenApi)]
#[openapi(
    info(
        title = "Basic API + Swagger written in Rust",
        description = "A simple API example written in Rust with Swagger UI - Author: Michele Della Guardia - License: MIT",
        contact(
            name = "Michele",
            email = "micheledellaguardia@gmail.com"
        ),
        license(
            name = "MIT",
            url = "https://opensource.org/licenses/MIT"
        )
    ),
    paths(health, list_all, create_all, get_all, update_all, delete_all),
    components(schemas(All, CreateAll, UpdateAll)),
    tags((name = "all", description = "Operations on All"))
)]
pub struct ApiDoc;
