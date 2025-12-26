use axum::Router;
use axum::routing::get;
use tracing::{Level, info};

use crate::config::get_bind;
use crate::server::login::{api_attempt_login, api_request_login};
use crate::server::meeting::{get_meeting, get_whoami};

pub mod admin;
pub mod meeting;
pub mod login;

pub async fn start_server() -> anyhow::Result<()> {
    tracing_subscriber::fmt()
        .with_max_level(Level::DEBUG)
        .init();

    info!("Starting Meetinizer");

    let router = Router::new()
        .route("/api/meeting/{id}", get(get_meeting))
        .route("/api/meeting/{id}/whoami", get(get_whoami))
        .route("/api/meeting/{id}/request-login/{email}", get(api_request_login))
        .route("/api/meeting/{id}/login/{email}/{token}", get(api_attempt_login));

    let bind = get_bind()?;

    info!("Starting web server on {bind}");

    let listener = tokio::net::TcpListener::bind(bind).await?;
    
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

