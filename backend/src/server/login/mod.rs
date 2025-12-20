use axum::extract::Path;
use utoipa_axum::router::OpenApiRouter;
use utoipa_axum::routes;

pub fn login_router() -> OpenApiRouter {
    OpenApiRouter::new()
        .routes(routes!(
                request_login,
                attempt_login
        ))
}

#[utoipa::path(
    get,
    path = "/{id}/{email}",
    responses(
        (status = 200, description = "Meeting found successfully"),
        (status = NOT_FOUND, description = "Meeting was not found")
    ),
    params(
        ("id" = Uuid, Path, description = "Meeting id to login to"),
    )
)]
pub async fn request_login(Path((id,email)): Path<(String,String)>) -> String {
    format!("id {id}, email {email}")
}

#[utoipa::path(
    post,
    path = "/{id}/{email}",
    responses(
        (status = 200, description = "Meeting found successfully"),
        (status = NOT_FOUND, description = "Meeting was not found")
    ),
    params(
        ("id" = Uuid, Path, description = "Meeting id to login to"),
    )
)]
pub async fn attempt_login(Path((id,email)): Path<(String,String)>, body: String) -> String {
    format!("id {id}, email {email}, body {body}")
}
