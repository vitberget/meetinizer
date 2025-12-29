use std::sync::Arc;

use axum::Json;
use axum::extract::Path;
use axum::http::StatusCode;
use axum_extra::extract::CookieJar;
use serde::{Deserialize, Serialize};
use tracing::{info, warn};
use uuid::Uuid;

use crate::db::meeting::MEETING_DB;
use crate::server::meeting::login::claims::MeetingEmailClaims;
use crate::structs::{Meeting, User};

pub mod mock;
pub mod login;

pub async fn get_meeting(
    Path(id): Path<String>,
    cookies: CookieJar
) -> Result<Json<Meeting>, StatusCode> {
    match MeetingEmailClaims::get_and_validate(&id, &cookies) {
        Ok(_) => {
            let arc = Arc::clone(&MEETING_DB);
            match arc.lock().await.get_meeting_by_name(&id) {
                Ok(meeting) => Ok(Json(meeting)),
                Err(error) => {
                    warn!("Error getting meeting: {error}");
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

pub async fn get_whoami(
    Path(id): Path<String>,
    cookies: CookieJar
) -> Result<String, StatusCode> {
    match MeetingEmailClaims::get_and_validate(&id, &cookies) {
        Ok(claims) => Ok(claims.get_email().to_string()),
        Err(error) => {
            warn!("Error getting claims {error}");
            Err(StatusCode::FORBIDDEN)
        }
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct RegisterName {
    pub meeting_uuid: Uuid,
    pub meeting_revision: Uuid,
    pub name: String
}

pub async fn post_register_name(
    Path(id): Path<String>,
    cookies: CookieJar,
    Json(name): Json<RegisterName>
) -> Result<StatusCode, StatusCode> {
    match MeetingEmailClaims::get_and_validate(&id, &cookies) {
        Ok(claims) => {
            let user = User::new(&name.name, claims.get_email());

            let arc = Arc::clone(&MEETING_DB);
            match arc.lock().await.add_user(&name.meeting_uuid, &name.meeting_revision, user.to_owned()) {
                Ok(_) => {
                    info!("User {user:?} registred on {id}");
                    Ok(StatusCode::OK)
                }
                Err(err) => {
                    warn!("Error adding user {user:?} to meeting {id}: {err}");
                    Err(StatusCode::FORBIDDEN)
                }
            } 
        }
        Err(error) => {
            warn!("Error getting claim: {error}");
            Err(StatusCode::FORBIDDEN)
        }
    }
}
