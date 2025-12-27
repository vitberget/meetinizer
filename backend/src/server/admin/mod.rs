use axum::Json;
use axum::extract::Path;
use axum::http::StatusCode;
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
pub async fn api_admin_list_meetings( cookies: CookieJar) -> Result<Json<Vec<String>>, StatusCode> {
    if let Some(admin) = cookies.get("admin") {
        if let Ok(secret) = get_jwt_secret() {
            if let Ok(_claims) = <(&str, &str) as TryInto<AdminClaims>>::try_into((admin.value(), &secret)) {
                Ok(Json(vec![
                        "fake1".to_string(), 
                        "fake2".to_string()
                ]))
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
            if let Ok(token) = AdminClaims::default().to_jwt(&secret) {
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

pub async fn api_admin_logout() -> Result<CookieJar, StatusCode> {
    let cookies = CookieJar::new()
        .add(Cookie::build(("admin",""))
            .path("/api/admin/")
            .http_only(true)
            
            // .max_age(axum_extra::extract::cookie::)
            // .expires(now)
        );

    Ok(cookies)
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
