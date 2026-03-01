use std::convert::Infallible;
use std::sync::Arc;

use async_stream::try_stream;
use axum::Json;
use axum::extract::Path;
use axum::http::StatusCode;
use axum::response::Sse;
use axum::response::sse::{Event, KeepAlive};
use axum_extra::extract::CookieJar;
use axum_extra::extract::cookie::Cookie;
use futures_util::Stream;
use tracing::{debug, info, warn};
use cookie::time::Duration;

use crate::config::get_jwt_secret;
use crate::db::meeting::{MEETING_DB, subscribe_to_meeting_queue};
use crate::server::admin::claim::AdminClaims;
use crate::server::admin::login::is_correct_admin_password;
use crate::structs::{Meeting, Slot};

pub mod claim;
pub mod login;


pub async fn api_admin_create_meeting(
    Path(id): Path<String>,
    cookies: CookieJar,
) -> Result<Json<Meeting>, StatusCode> {
    match AdminClaims::get_and_validate(&cookies) {
        Err(error) => {
            warn!("error getting admin claims {error}");
            Err(StatusCode::FORBIDDEN)
        }
        Ok(_) => {
            let arc = Arc::clone(&MEETING_DB);
            let meeting_db = arc.lock().await;
            match meeting_db.create_meeting(&id) {
                Ok(meeting) => {
                    info!("Admin creeated meeting {id}");
                    Ok(Json(meeting))
                }
                Err(error) => {
                    warn!("Failed to create meeting for admin {id}: {error}");
                    Err(StatusCode::INTERNAL_SERVER_ERROR)
                }
            }
        }
    }
}

pub async fn api_admin_update_comment(
    Path(id): Path<String>,
    cookies: CookieJar,
    comment: String
) -> Result<Json<Meeting>, StatusCode> {
    match AdminClaims::get_and_validate(&cookies) {
        Err(error) => {
            warn!("error getting admin claims {error}");
            Err(StatusCode::FORBIDDEN)
        }
        Ok(_) => {
            let arc = Arc::clone(&MEETING_DB);
            let meeting_db = arc.lock().await;
            match meeting_db.update_comment(&id, &comment) {
                Ok(meeting) => {
                    info!("admin udated comment {comment:?} into {id}");
                    Ok(Json(meeting))
                }
                Err(error) => {
                    warn!("admin failed to update comment {comment:?} into {id}: {error}");
                    Err(StatusCode::INTERNAL_SERVER_ERROR)
                }
            }

        }
    }
}

pub async fn api_admin_add_slot(
    Path(id): Path<String>,
    cookies: CookieJar,
    Json(slot): Json<Slot>
) -> Result<Json<Meeting>, StatusCode> {
    match AdminClaims::get_and_validate(&cookies) {
        Err(error) => {
            warn!("error getting admin claims {error}");
            Err(StatusCode::FORBIDDEN)
        }
        Ok(_) => {
            let arc = Arc::clone(&MEETING_DB);
            let meeting_db = arc.lock().await;
            match meeting_db.add_slot_unsafe(&id, slot.to_owned()) {
                Ok(meeting) => {
                    info!("admin insert slot {slot:?} into {id}");
                    Ok(Json(meeting))
                }
                Err(error) => {
                    warn!("admin failed to insert slot {slot:?} into {id}: {error}");
                    Err(StatusCode::INTERNAL_SERVER_ERROR)
                }
            }

        }
    }
}

pub async fn api_admin_rm_slot(
    Path(id): Path<String>,
    cookies: CookieJar,
    Json(slot): Json<Slot>
) -> Result<Json<Meeting>, StatusCode> {
    match AdminClaims::get_and_validate(&cookies) {
        Err(error) => {
            warn!("error getting admin claims {error}");
            Err(StatusCode::FORBIDDEN)
        }
        Ok(_) => {
            let arc = Arc::clone(&MEETING_DB);
            let meeting_db = arc.lock().await;
            match meeting_db.rm_slot_unsafe(&id, slot.to_owned()) {
                Ok(meeting) => {
                    info!("admin removed slot {slot:?} into {id}");
                    Ok(Json(meeting))
                }
                Err(error) => {
                    warn!("admin failed to remove slot {slot:?} into {id}: {error}");
                    Err(StatusCode::INTERNAL_SERVER_ERROR)
                }
            }

        }
    }
}

pub async fn api_admin_lock(
    Path(id): Path<String>,
    cookies: CookieJar,
    Json(lock): Json<bool>
) -> Result<Json<Meeting>, StatusCode> {
    match AdminClaims::get_and_validate(&cookies) {
        Err(error) => {
            warn!("error getting admin claims {error}");
            Err(StatusCode::FORBIDDEN)
        }
        Ok(_) => {
            let arc = Arc::clone(&MEETING_DB);
            let meeting_db = arc.lock().await;
            match meeting_db.update_locked(&id, lock) {
                Ok(meeting) => {
                    info!("admin set locked to {lock} for {id}");
                    Ok(Json(meeting))
                }
                Err(error) => {
                    warn!("admin failed to set locked to {lock} for {id}: {error}");
                    Err(StatusCode::INTERNAL_SERVER_ERROR)
                }
            }

        }
    }
}


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

pub async fn api_admin_list_meetings(cookies: CookieJar) -> Result<Json<Vec<String>>, StatusCode> {
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

pub async fn api_admin_login(body: String) -> Result<CookieJar, StatusCode> {
    match is_correct_admin_password(&body) {
        Ok(true) => match get_jwt_secret() {
            Ok(secret) => match AdminClaims::default().to_jwt(&secret) {
                Ok(token) => {
                    info!("Admin logged in");
                    let cookie_jar = CookieJar::new()
                        .add(Cookie::build(("admin", token))
                            .max_age(Duration::days(30))
                            .path("/api/admin/")
                            .http_only(true));
                    Ok(cookie_jar)
                }
                Err(error) => {
                    warn!("Error encoding to jwt {error}");
                    Err(StatusCode::FORBIDDEN)
                }
            }
            Err(error) => {
                warn!("Error gettin jwt secret {error}");
                Err(StatusCode::FORBIDDEN)
            },
        }
        Ok(false) => {
            warn!("Wrong admin password");
            Err(StatusCode::FORBIDDEN)
        }
        Err(error) => {
            warn!("Error with admin password {error}");
            Err(StatusCode::FORBIDDEN)
        },
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

pub async fn sse_admin_meeting(
    Path(id): Path<String>,
    cookies: CookieJar
) -> Result<Sse<impl Stream<Item = Result<Event, Infallible>>>, StatusCode> {
    info!("client connected");
    match AdminClaims::get_and_validate(&cookies) {
        Err(error) => {
            warn!("Error getting claims {error}");
            Err(StatusCode::FORBIDDEN)
        }
        Ok(_) => {
            let mut queue = subscribe_to_meeting_queue();

            Ok(Sse::new(try_stream! {
                {
                    let arc = Arc::clone(&MEETING_DB);
                    match arc.lock().await.get_meeting_by_name(&id) {
                        Ok(meeting) => {
                            if let Ok(json) = serde_json::to_string(&meeting) {
                                debug!("releasing event from db {json}");
                                let event = Event::default().data(json);
                                yield event;
                            }
                        }
                        Err(err) => {
                            warn!("Error getting meeting {err}");
                            // Err(StatusCode::FORBIDDEN)
                        }
                    }
                }

                while let Ok(meeting) = queue.recv().await {
                    if meeting.get_name() == id
                        && let Ok(json) = serde_json::to_string(&meeting) {
                            debug!("releasing event {json}");
                            let event = Event::default().data(json);
                            yield event;
                    }
                }
            }).keep_alive(KeepAlive::default()))
        }
    }
}

pub async fn sse_admin_all_meetings(cookies: CookieJar) -> Result<Sse<impl Stream<Item = Result<Event, Infallible>>>, StatusCode> {
    match AdminClaims::get_and_validate(&cookies) {
        Err(error) => {
            warn!("Error getting claims {error}");
            Err(StatusCode::FORBIDDEN)
        }
        Ok(_) => {
            let mut queue = subscribe_to_meeting_queue();

            Ok(Sse::new(try_stream! {
                while let Ok(meeting) = queue.recv().await {
                    if let Ok(json) = serde_json::to_string(&meeting) {
                        let event = Event::default().data(json);
                        yield event;
                    }
                }
            }).keep_alive(KeepAlive::default()))
        }
    }
}
