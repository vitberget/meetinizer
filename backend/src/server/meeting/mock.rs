use axum_extra::extract::cookie::Cookie;
use chrono::{Local, NaiveDate};

use crate::structs::{Meeting, Slot, User};

pub fn get_meeting_mock(id: &str, login: Option<&Cookie>) -> Meeting {
    println!("logincookie {login:?}");
    let mut meeting = Meeting::new(&format!("Hello {id}"));
    let _ = meeting.add_user(User::new("Kenneth Hedman", "test@vitberget.se"));
    let slot = Slot { 
        start: NaiveDate::from_ymd_opt(2025, 6, 4).unwrap().and_hms_opt(15,0,0).unwrap().and_local_timezone(Local).earliest().unwrap(),
        end: NaiveDate::from_ymd_opt(2025, 6, 4).unwrap().and_hms_opt(15,0,0).unwrap().and_local_timezone(Local).earliest().unwrap(),
    };
    meeting.add_slot(slot.clone());
    meeting.add_vote(User::new("Kenneth Hedman", "test@vitberget.se"), slot).unwrap();
    meeting
}
