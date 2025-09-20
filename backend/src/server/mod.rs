use axum::Json;
use axum::extract::Path;
use chrono::{Local, NaiveDate};
use utoipa_axum::{routes, router::OpenApiRouter};
use utoipa_swagger_ui::SwaggerUi;

use crate::structs::{Meeting, Slot, User};

pub async fn start_server() -> anyhow::Result<()> {
    let (router, mut api) = OpenApiRouter::new()
        .routes(routes!(
                get_meeting
        )).split_for_parts();

    api.info.title = "hello".to_owned();
    api.info.description = Some("I am a happy boy!".to_owned());
    api.info.contact = None; // TODO perhaps set info.contact?
    api.info.license = None; // TODO set info.licence!
    api.info.version = "test-1".to_owned();

    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    let router = router .merge(SwaggerUi::new("/swagger-ui").url("/api-docs/openapi.json", api));
    axum::serve(listener, router).await.unwrap();

    Ok(())
}


#[utoipa::path(
    get,
    path = "/api/meeting/{id}",
    responses(
        (status = 200, description = "Meeting found successfully", body = Meeting),
        (status = NOT_FOUND, description = "Meeting was not found")
    ),
    params(
        ("id" = Uuid, Path, description = "Meeting id to get"),
    )
)]
async fn get_meeting(Path(id): Path<String>) -> Json<Meeting> {
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
