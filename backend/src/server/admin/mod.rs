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
use crate::structs::{Meeting, Slot};

pub mod claim;
pub mod login;

pub async fn api_admin_get_meeting(
    Path(id): Path<String>,
    cookies: CookieJar
) -> Result<Json<Meeting>, StatusCode> {
    match AdminClaims::get_and_validate(&cookies) {
        Ok(_) => {
            let arc = Arc::clone(&MEETING_DB);
            match arc.lock().await.get_meeting_by_name(&id) {
                Ok(meeting) => Ok(Json(meeting)),
                Err(err) => {
                    warn!("Error getting meeting {err}");
                    Err(StatusCode::FORBIDDEN)
                }
            }
        }
        Err(err) => {
            warn!("Error getting claims {err}");
            Err(StatusCode::FORBIDDEN)
        }
    }
}

pub async fn api_admin_list_meetings( cookies: CookieJar) -> Result<Json<Vec<String>>, StatusCode> {
    match AdminClaims::get_and_validate(&cookies) {
        Ok(_) => {
            let arc = Arc::clone(&MEETING_DB);
            match arc.lock().await.get_meeting_names() {
                Ok(meetings) => Ok(Json(meetings)),
                Err(err) => {
                    warn!("Error getting meeting names {err}");
                    Err(StatusCode::FORBIDDEN)
                }
            }
        }
        Err(err) => {
            warn!("Error getting claims: {err}");
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
}

pub async fn api_admin_login(body: String) -> Result<CookieJar, StatusCode> {
    match is_correct_admin_password(&body) {
        Ok(true) => match get_jwt_secret() {
            Ok(secret) => match AdminClaims::default().to_jwt(&secret) {
                Ok(token) => {
                    let cookie_jar = CookieJar::new()
                        .add(Cookie::build(("admin", token))
                            .path("/api/admin/")
                            .http_only(true));
                    Ok(cookie_jar)
                }
                Err(_) => Err(StatusCode::FORBIDDEN),
            }

            Err(_) => Err(StatusCode::FORBIDDEN),
        }
        _ => Err(StatusCode::FORBIDDEN),
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

