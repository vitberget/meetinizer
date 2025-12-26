use axum::{extract::Path, Json};
use chrono::{Local, NaiveDate};

use crate::structs::{Meeting, Slot, User};

pub async fn admin_get_meeting(Path(id): Path<String>) -> Json<Meeting> {
    println!("admin get meeting");
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
