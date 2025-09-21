use utoipa_axum::router::OpenApiRouter;
use utoipa_swagger_ui::SwaggerUi;

use crate::config::get_bind;
use crate::server::admin::admin_router;
use crate::server::user::user_router;

pub mod admin;
pub mod user;

pub async fn start_server() -> anyhow::Result<()> {
    let (router, mut api) = OpenApiRouter::new()
        .nest("/api/admin/", admin_router())
        .nest("/api/user/", user_router())
        .split_for_parts();

    api.info.title = "hello".to_owned();
    api.info.description = Some("I am a happy boy!".to_owned());
    api.info.contact = None; // TODO perhaps set info.contact?
    api.info.license = None; // TODO set info.licence!
    api.info.version = "test-1".to_owned();

    // TODO logging
    let listener = tokio::net::TcpListener::bind(get_bind()?).await?;
    let router = router.merge(SwaggerUi::new("/swagger-ui").url("/api-docs/openapi.json", api));
    axum::serve(listener, router).await?;

    Ok(())
}


// TODO user action: add add_user
// TODO user action: add remove_user
// TODO user action: add add_vote
// TODO user action: add remove_vote
// TODO user action: create user
// TODO user action: change name
// TODO user action: login email

