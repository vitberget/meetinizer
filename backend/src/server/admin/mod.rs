use std::sync::Arc;

use axum::Json;
use axum::extract::Path;
use axum::http::StatusCode;
use axum_extra::extract::CookieJar;
use axum_extra::extract::cookie::Cookie;
use serde::{Deserialize, Serialize};
use tracing::warn;
use uuid::Uuid;

use crate::config::get_jwt_secret;
use crate::db::meeting::MEETING_DB;
use crate::server::admin::claim::AdminClaims;
use crate::server::admin::login::is_correct_admin_password;
use crate::server::meeting::get_meeting_mock;
use crate::structs::{Meeting, Slot};

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
    match cookies.get("admin") {
        Some(admin) => match get_jwt_secret() {
            Ok(secret) => match <(&str, &str) as TryInto<AdminClaims>>::try_into((admin.value(), &secret)) {
                Ok(_claims) => {
                    let arc = Arc::clone(&MEETING_DB);
                    let real_db = arc.lock().await;
                    match real_db.get_meeting_names() {
                        Ok(meetings) => Ok(Json(meetings)),
                        Err(err) => {
                            warn!("Error getting meeting names {err}");
                            Err(StatusCode::FORBIDDEN)
                        }
                    }
                }
                Err(_) => {
                    warn!("not a claim in cookie");
                    Err(StatusCode::FORBIDDEN)
                }
            }

            Err(_) => {
                warn!("failed get_jwt_secret");
                Err(StatusCode::FORBIDDEN)
            }
        }
        None => {
            warn!("missing login cookie");
            Err(StatusCode::FORBIDDEN)
        }
    }
}
#[derive(Debug, Serialize, Deserialize)]
pub struct AddSlot {
    pub meeting_uuid: Uuid,
    pub meeting_revision: Uuid,
    pub slot: Slot
}

pub async fn api_admin_add_slot(Json(add_slot): Json<AddSlot>) -> Result<Meeting, (StatusCode, String)> {
    let arc = Arc::clone(&MEETING_DB);
    let meeting_db = arc.lock().await;
    match meeting_db.add_slot(&add_slot.meeting_uuid, &add_slot.meeting_revision, add_slot.slot) {
        Ok(meeting) => Ok(meeting),
        Err(err) => Err((StatusCode::INTERNAL_SERVER_ERROR, format!("{err}")))
    }


    // todo!()
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
        );

    Ok(cookies)
}

