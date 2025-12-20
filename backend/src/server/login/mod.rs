use axum::extract::Path;
use utoipa_axum::router::OpenApiRouter;
use utoipa_axum::routes;

pub mod db;
pub mod admin;

pub fn login_router() -> OpenApiRouter {
    OpenApiRouter::new()
        .routes(routes!(
                api_request_login,
                api_attempt_login
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
pub async fn api_request_login(Path((id,email)): Path<(String,String)>) -> String {
    // format!("id {id}, email {email}")
    let valid = db::register_login(&id, &email).await.unwrap();
    format!("{valid}")

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
pub async fn api_attempt_login(Path((id,email)): Path<(String,String)>, body: String) -> String {
    db::attempt_login(&id, &email, &body).await.unwrap()
}


