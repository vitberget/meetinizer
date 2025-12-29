use std::convert::Infallible;
use std::sync::Arc;

use async_stream::try_stream;
use axum::Json;
use axum::extract::Path;
use axum::http::StatusCode;
use axum::response::Sse;
use axum::response::sse::{Event, KeepAlive};
use axum_extra::extract::CookieJar;
use futures_util::Stream;
use serde::{Deserialize, Serialize};
use tracing::{info, warn};

use crate::db::meeting::{MEETING_DB, subscribe_to_meeting_queue};
use crate::server::meeting::login::claims::MeetingEmailClaims;
use crate::structs::{Meeting, Slot, User, Vote};

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


pub async fn sse_meeting(
    Path(id): Path<String>,
    cookies: CookieJar
) -> Result<Sse<impl Stream<Item = Result<Event, Infallible>>>, StatusCode> {
    match MeetingEmailClaims::get_and_validate(&id, &cookies) {
        Err(error) => {
            warn!("Error getting claims {error}");
            Err(StatusCode::FORBIDDEN)
        }
        Ok(_) => {
            let mut queue = subscribe_to_meeting_queue();

            Ok(Sse::new(try_stream! {
                while let Ok(meeting) = queue.recv().await {
                    if meeting.get_name() == id
                        && let Ok(json) = serde_json::to_string(&meeting) {
                            let event = Event::default().data(json);
                            yield event;
                    }
                }
            }).keep_alive(KeepAlive::default()))
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
    pub name: String
}

pub async fn post_vote_add(
    Path(id): Path<String>,
    cookies: CookieJar,
    Json(slot): Json<Slot>
) -> Result<StatusCode, StatusCode> {
    match MeetingEmailClaims::get_and_validate(&id, &cookies) {
        Err(error) => {
            warn!("Error getting claim: {error}");
            Err(StatusCode::FORBIDDEN)
        }
        Ok(claims) => {
            let vote = Vote {
                user_email: claims.get_email().to_owned(),
                slot,
            };
            let arc = Arc::clone(&MEETING_DB);
            match arc.lock().await.add_vote_unsafe(&id, vote) {
                Ok(_) => {
                    info!("User {user:?} adding vote on {vote:?} in {id}", user = claims.get_email());
                    Ok(StatusCode::OK)
                }
                Err(err) => {
                    warn!("Error when user {user:?} adding vote on {vote:?} in {id}", user = claims.get_email());
                    Err(StatusCode::FORBIDDEN)
                }
            } 
        }
    }
}

pub async fn post_vote_rm(
    Path(id): Path<String>,
    cookies: CookieJar,
    Json(slot): Json<Slot>
) -> Result<StatusCode, StatusCode> {
    match MeetingEmailClaims::get_and_validate(&id, &cookies) {
        Err(error) => {
            warn!("Error getting claim: {error}");
            Err(StatusCode::FORBIDDEN)
        }
        Ok(claims) => {
            let vote = Vote {
                user_email: claims.get_email().to_owned(),
                slot,
            };
            let arc = Arc::clone(&MEETING_DB);
            match arc.lock().await.rm_vote_unsafe(&id, vote) {
                Ok(_) => {
                    info!("User {user:?} removed vote on {vote:?} in {id}", user = claims.get_email());
                    Ok(StatusCode::OK)
                }
                Err(err) => {
                    warn!("Error when user {user:?} removing vote on {vote:?} in {id}", user = claims.get_email());
                    Err(StatusCode::FORBIDDEN)
                }
            } 
        }
    }
}

pub async fn post_register_name(
    Path(id): Path<String>,
    cookies: CookieJar,
    Json(name): Json<RegisterName>
) -> Result<StatusCode, StatusCode> {
    match MeetingEmailClaims::get_and_validate(&id, &cookies) {
        Err(error) => {
            warn!("Error getting claim: {error}");
            Err(StatusCode::FORBIDDEN)
        }
        Ok(claims) => {
            let user = User::new(&name.name, claims.get_email());

            let arc = Arc::clone(&MEETING_DB);
            match arc.lock().await.add_user_unsafe(&id, user.to_owned()) {
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
    }
}
