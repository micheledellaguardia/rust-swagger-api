use utoipa::OpenApi;
use crate::models::{All, CreateAll, UpdateAll};
use crate::handlers::{health, list_all, create_all, get_all, update_all, delete_all};

#[derive(OpenApi)]
#[openapi(
    paths(health, list_all, create_all, get_all, update_all, delete_all),
    components(schemas(All, CreateAll, UpdateAll)),
    tags((name = "alls", description = "Operazioni sui All"))
)]
pub struct ApiDoc;
