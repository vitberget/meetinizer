use axum::{Json, extract::Path, http::StatusCode};
use axum_extra::extract::CookieJar;
use utoipa_axum::{router::OpenApiRouter, routes};


pub mod mock;

pub fn meeting_router() -> OpenApiRouter {
    OpenApiRouter::new()
        .routes(routes!(
                get_meeting,
                // create_meeting,
                // update_meeting
        ))
}

#[utoipa::path(
    get,
    tag = "Meeting",
    path = "/{id}",
    responses(
        (status = 200, description = "Meeting found successfully", body = Meeting),
        (status = NOT_FOUND, description = "Meeting was not found")
    ),
    params(
        ("id" = String, Path, description = "Meeting id to get"),
    )
)]
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
