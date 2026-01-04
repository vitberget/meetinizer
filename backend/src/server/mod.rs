use axum::Router;
use axum::http::StatusCode;
use axum::routing::{get, post};
use tracing::{Level, info};

use crate::config::get_bind;
use crate::db::init::init_meeting;
use crate::server::admin::{api_admin_add_slot, api_admin_create_meeting, api_admin_get_meeting, api_admin_list_meetings, api_admin_login, api_admin_logout, api_admin_rm_slot, api_admin_update_comment, sse_admin_all_meetings, sse_admin_meeting};
use crate::server::meeting::login::{api_attempt_login, api_logout, api_request_login};
use crate::server::meeting::{get_meeting, get_whoami, post_register_name, post_vote_add, post_vote_rm, sse_meeting};

pub mod admin;
pub mod meeting;

pub async fn start_server() -> anyhow::Result<()> {
    tracing_subscriber::fmt()
        .with_max_level(Level::DEBUG)
        .init();

    info!("Starting Meetinizer");

    init_meeting()?;

    let router = Router::new()
        .route("/hc", get(get_healthcheck))

        .route("/api/admin/meeting/{id}", get(api_admin_get_meeting))
        .route("/api/admin/meeting/{id}/sse", get(sse_admin_meeting))
        .route("/api/admin/meeting/{id}/create", get(api_admin_create_meeting))
        .route("/api/admin/meeting/{id}/slot/add", post(api_admin_add_slot))
        .route("/api/admin/meeting/{id}/comment", post(api_admin_update_comment))
        .route("/api/admin/meeting/{id}/slot/rm", post(api_admin_rm_slot))
        .route("/api/admin/meetings/sse", get(sse_admin_all_meetings))
        .route("/api/admin/meetings/list", get(api_admin_list_meetings))
        .route("/api/admin/login", post(api_admin_login))
        .route("/api/admin/logout", get(api_admin_logout))

        .route("/api/meeting/{id}", get(get_meeting))
        .route("/api/meeting/{id}/sse", get(sse_meeting))
        .route("/api/meeting/{id}/whoami", get(get_whoami))
        .route("/api/meeting/{id}/register-name", post(post_register_name))
        .route("/api/meeting/{id}/vote/add", post(post_vote_add))
        .route("/api/meeting/{id}/vote/rm", post(post_vote_rm))
        .route("/api/meeting/{id}/request-login/{email}", get(api_request_login))
        .route("/api/meeting/{id}/login/{email}/{token}", get(api_attempt_login))
        .route("/api/meeting/{id}/logout", get(api_logout));

    let bind = get_bind()?;

    info!("Starting web server on {bind}");

    let listener = tokio::net::TcpListener::bind(bind).await?;
    
    axum::serve(listener, router).await?;

    Ok(())
}

async fn get_healthcheck() -> StatusCode {
    StatusCode::OK
}
