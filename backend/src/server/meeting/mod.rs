use axum::Json;
use axum::extract::Path;
use axum::http::StatusCode;
use axum_extra::extract::CookieJar;

pub mod mock;
pub mod login;

pub async fn get_meeting(
    Path(id): Path<String>,
    cookies: CookieJar
) -> Result<Json<Meeting>, StatusCode> {
    if let Some(login) = cookies.get("login") {
        if let Ok(secret) = get_jwt_secret() {
            if let Ok(claims) = <(&str, &str) as TryInto<MeetingEmailClaims>>::try_into((login.value(), &secret)) {
                if claims.get_meeting() == id {
                    Ok(Json(get_meeting_mock(&id)))
                } else {
                    warn!("wrong meeting in claim");
                    Err(StatusCode::FORBIDDEN)
                }
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

pub async fn get_whoami(
    Path(id): Path<String>,
    cookies: CookieJar
) -> Result<String, StatusCode> {
    if let Some(login) = cookies.get("login") {
        if let Ok(secret) = get_jwt_secret() {
            if let Ok(claims) = <(&str, &str) as TryInto<MeetingEmailClaims>>::try_into((login.value(), &secret)) {
                if claims.get_meeting() == id {
                    // Ok(Json(get_meeting_mock(&id)))
                    Ok(claims.get_email().to_string())
                } else {
                    warn!("wrong meeting in claim");
                    Err(StatusCode::FORBIDDEN)
                }
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

use chrono::{Local, NaiveDate};
use tracing::{debug, warn};

use crate::{config::get_jwt_secret, server::meeting::login::claims::MeetingEmailClaims, structs::{Meeting, Slot, User}};

pub fn get_meeting_mock(id: &str) -> Meeting {
    debug!("meeting mock");

    let mut meeting = Meeting::new(&format!("Hello {id}"));
    meeting.add_user(User::new("Kenneth Hedman", "test@vitberget.se"));
    let slot = Slot { 
        start: NaiveDate::from_ymd_opt(2025, 6, 4).unwrap().and_hms_opt(18,0,0).unwrap().and_local_timezone(Local).earliest().unwrap(),
        end: NaiveDate::from_ymd_opt(2025, 6, 4).unwrap().and_hms_opt(21,0,0).unwrap().and_local_timezone(Local).earliest().unwrap(),
    };
    let slot2 = Slot { 
        start: NaiveDate::from_ymd_opt(2025, 6, 7).unwrap().and_hms_opt(18,0,0).unwrap().and_local_timezone(Local).earliest().unwrap(),
        end: NaiveDate::from_ymd_opt(2025, 6, 7).unwrap().and_hms_opt(21,0,0).unwrap().and_local_timezone(Local).earliest().unwrap(),
    };
    meeting.add_slot(slot.clone());
    meeting.add_slot(slot2);
    meeting.add_vote(User::new("Kenneth Hedman", "test@vitberget.se"), slot).unwrap();
    meeting
}
