use axum::{extract::Path, Json};
use chrono::{Local, NaiveDate};
use utoipa_axum::{router::OpenApiRouter, routes};

use crate::structs::{Meeting, Slot, User};

pub fn user_router() -> OpenApiRouter {
    OpenApiRouter::new()
        .routes(routes!(
                get_meeting,
                // create_meeting,
                // update_meeting
        ))
}

#[utoipa::path(
    get,
    path = "/get/{id}",
    responses(
        (status = 200, description = "Meeting found successfully", body = Meeting),
        (status = NOT_FOUND, description = "Meeting was not found")
    ),
    params(
        ("id" = Uuid, Path, description = "Meeting id to get"),
    )
)]
pub async fn get_meeting(Path(id): Path<String>) -> Json<Meeting> {
    let mut meeting = Meeting::new(format!("Hello {id}"));
    meeting.add_user(User::new("Kenneth Hedman", "test@vitberget.se"));
    let slot = Slot { 
        start: NaiveDate::from_ymd_opt(2025, 6, 4).unwrap().and_hms_opt(15,0,0).unwrap().and_local_timezone(Local).earliest().unwrap(),
        end: NaiveDate::from_ymd_opt(2025, 6, 4).unwrap().and_hms_opt(15,0,0).unwrap().and_local_timezone(Local).earliest().unwrap(),
    };
    meeting.add_slot(slot.clone());
    meeting.add_vote(User::new("Kenneth Hedman", "test@vitberget.se"), slot).unwrap();
    Json(meeting)
}
