use axum::{Json, extract::Path, http::StatusCode};
use axum_extra::extract::CookieJar;
use axum_extra::extract::cookie::Cookie;
use tracing::warn;

use crate::config::get_jwt_secret;
use crate::server::admin::claim::AdminClaims;
use crate::server::admin::login::is_correct_admin_password;
use crate::server::meeting::get_meeting_mock;
use crate::structs::Meeting;

pub mod claim;
pub mod login;

pub async fn api_admin_get_meeting(
    Path(id): Path<String>,
    cookies: CookieJar
) -> Result<Json<Meeting>, StatusCode> {
    if let Some(admin) = cookies.get("admin") {
        if let Ok(secret) = get_jwt_secret() {
            if let Ok(_claims) = <(&str, &str) as TryInto<AdminClaims>>::try_into((admin.value(), &secret)) {
                Ok(Json(get_meeting_mock(&id)))
            } else {
                warn!("not a claim in cookie");
                Err(StatusCode::FORBIDDEN)
            }
        } else {
            warn!("failed get_jwt_secret");
            Err(StatusCode::FORBIDDEN)
        }
    } else {
        warn!("missing login cookie");
        Err(StatusCode::FORBIDDEN)
    }
}

pub async fn api_admin_login(body: String) -> Result<CookieJar, StatusCode> {
    if let Ok(true) = is_correct_admin_password(&body) {
        if let Ok(secret) = get_jwt_secret() {
            if let Ok(token) = AdminClaims::new().to_jwt(&secret) {
                let cookie_jar = CookieJar::new()
                    .add(Cookie::build(("admin", token)).path("/api/admin/").http_only(true));

                Ok(cookie_jar)

            } else {
                Err(StatusCode::FORBIDDEN)
            }
        } else {
            Err(StatusCode::FORBIDDEN)
        }
    } else {
        Err(StatusCode::FORBIDDEN)
    }
}


// TODO protect, only Admin
pub async fn admin_create_meeting(Path(name): Path<String>) -> Json<Meeting> {
    let meeting = Meeting::new(name);
    Json(meeting)
}

// TODO protect, only Admin
pub async fn admin_update_meeting(Json(meeting): Json<Meeting>) -> Json<Meeting> {
    // let meeting = Meeting::new(name);
    Json(meeting)
}
