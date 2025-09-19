use axum::extract::Path;
use axum::{Json};
use utoipa_axum::{routes, router::OpenApiRouter};
use utoipa_swagger_ui::SwaggerUi;

use crate::structs::Meeting;

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


/// Get pet by id
///
/// Get pet from database by pet id
#[utoipa::path(
    get,
    path = "/api/meeting/{id}",
    responses(
        (status = 200, description = "Pet found successfully", body = Meeting),
        (status = NOT_FOUND, description = "Pet was not found")
    ),
    params(
        ("id" = Uuid, Path, description = "Pet database id to get Pet for"),
    )
)]
async fn get_meeting(Path(id): Path<String>) -> Json<Meeting> {
    let meeting = Meeting::new(format!("Hello {id}"));
    Json(meeting)
}
