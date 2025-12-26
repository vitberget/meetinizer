use axum::{Json, extract::Path, http::StatusCode};
use axum_extra::extract::CookieJar;
use utoipa_axum::{router::OpenApiRouter, routes};

pub mod mock;

pub async fn get_meeting(
    Path(id): Path<String>,
    cookies: CookieJar
) -> Result<Json<Meeting>, StatusCode> {
    if let Some(login) = cookies.get("login") {
        if let Ok(claims) = <(&str, &str) as TryInto<MeetingEmailClaims>>::try_into((login.value(), "secret")) {
            if claims.get_meeting() == id {
                Ok(Json(get_meeting_mock(&id)))
            } else {
                println!("wrong meeting inclaim");
                Err(StatusCode::FORBIDDEN)
            }
        } else {
            println!("not a claim in cookie");
            Err(StatusCode::FORBIDDEN)
        }
    } else {
        println!("missing loginb cookie");
        Err(StatusCode::FORBIDDEN)
    }
}

pub async fn get_whoami(
    Path(id): Path<String>,
    cookies: CookieJar
) -> Result<String, StatusCode> {
    if let Some(login) = cookies.get("login") {
        if let Ok(claims) = <(&str, &str) as TryInto<MeetingEmailClaims>>::try_into((login.value(), "secret")) {
            if claims.get_meeting() == id {
                // Ok(Json(get_meeting_mock(&id)))
                Ok(claims.get_email().to_string())
            } else {
                println!("wrong meeting inclaim");
                Err(StatusCode::FORBIDDEN)
            }
        } else {
            println!("not a claim in cookie");
            Err(StatusCode::FORBIDDEN)
        }
    } else {
        println!("missing loginb cookie");
        Err(StatusCode::FORBIDDEN)
    }
}

use chrono::{Local, NaiveDate};

use crate::{server::login::claims::MeetingEmailClaims, structs::{Meeting, Slot, User}};

pub fn get_meeting_mock(id: &str) -> Meeting {
    println!("meeting mock");

    let mut meeting = Meeting::new(format!("Hello {id}"));
    meeting.add_user(User::new("Kenneth Hedman", "test@vitberget.se"));
    let slot = Slot { 
        start: NaiveDate::from_ymd_opt(2025, 6, 4).unwrap().and_hms_opt(15,0,0).unwrap().and_local_timezone(Local).earliest().unwrap(),
        end: NaiveDate::from_ymd_opt(2025, 6, 4).unwrap().and_hms_opt(15,0,0).unwrap().and_local_timezone(Local).earliest().unwrap(),
    };
    meeting.add_slot(slot.clone());
    meeting.add_vote(User::new("Kenneth Hedman", "test@vitberget.se"), slot).unwrap();
    meeting
}
