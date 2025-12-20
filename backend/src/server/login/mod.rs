use axum::extract::Path;
use axum::http::StatusCode;
use axum::response::Redirect;
use axum_extra::extract::CookieJar;
use axum_extra::extract::cookie::Cookie;
use utoipa_axum::router::OpenApiRouter;
use utoipa_axum::routes;

pub mod db;
pub mod claims;
pub mod admin;

pub fn login_request_router() -> OpenApiRouter {
    OpenApiRouter::new().routes(routes!(api_request_login))
}

pub fn login_attempt_router() -> OpenApiRouter {
    OpenApiRouter::new().routes(routes!(api_attempt_login))
}

#[utoipa::path(
    get,
    tag = "LoginUser",
    path = "/{id}/{email}",
    responses(
        (status = 200, description = "Meeting found successfully"),
        (status = NOT_FOUND, description = "Meeting was not found")
    ),
    params(
        ("id" = String, Path, description = "Meeting id to login to"),
    )
)]
pub async fn api_request_login(Path((id,email)): Path<(String,String)>) -> String {
    let valid = db::register_login(&id, &email).await.unwrap();
    format!("{valid}")

}

#[utoipa::path(
    get,
    tag = "LoginUser",
    path = "/{id}/{email}/{body}",
    responses(
        (status = 200, description = "Meeting found successfully"),
        (status = NOT_FOUND, description = "Meeting was not found")
    ),
    params(
        ("id" = String, Path, description = "Meeting id to login to"),
    )
)]
pub async fn api_attempt_login(Path((id, email, body)): Path<(String, String, String)>) -> Result<(CookieJar, Redirect), StatusCode> {
    println!("fkdlsjfkdls");
    if let Ok(token) = db::attempt_login(&id, &email, &body).await {
        let jar = CookieJar::new();

        let cookie = Cookie::build(("login", token))
            .path(format!("/api/meeting/{id}"))
            .http_only(true);

        let jar = jar.add(cookie);

        Ok((jar, Redirect::to(&format!("/api/meeting/login/{id}"))))
    } else {
        Err(StatusCode::UNAUTHORIZED)
    }
}


