use std::sync::{Arc, LazyLock};

use anyhow::bail;
use chrono::{DateTime, Duration, Utc};
use tokio::sync::Mutex;
use tracing::{debug, info, warn};
use uuid::Uuid;

use crate::config::{get_host, get_jwt_secret, get_jwt_valid_seconds};
use crate::server::meeting::login::claims::MeetingEmailClaims;
use crate::server::meeting::login::mail::mail_link;

#[derive(Debug)]
pub struct Login {
    pub meeting: String,
    pub email: String,
    pub secret: String,
    pub valid_until: DateTime<Utc>
}

static FAKE_DB: LazyLock<Arc<Mutex<Vec<Login>>>> = LazyLock::new(Default::default);

pub async fn register_login(meeting: &str, email: &str) -> anyhow::Result<i64> {
    info!("register_login {meeting} {email}");

    let valid_time: Duration = Duration::seconds(get_jwt_valid_seconds()?);
    let secret = Uuid::new_v4().to_string();
    let valid_until = Utc::now() + valid_time;

    let login = Login {
        meeting: meeting.to_owned(),
        email: email.to_owned(),
        secret: secret.to_owned(),
        valid_until,
    };

    let login_url = format!("{host}api/meeting/{meeting}/login/{email}/{secret}",
        host = get_host()?,
        meeting = urlencoding::encode(meeting),
        email = urlencoding::encode(email),
        secret = urlencoding::encode(&secret)
    );

    mail_link(email, meeting, &login_url).await?;

    debug!("  Login url: {login_url}");

    FAKE_DB.lock().await.push(login);

    Ok(valid_time.as_seconds_f32() as i64)
}



#[tracing::instrument]
pub async fn attempt_login(meeting: &str, email: &str, token: &str) -> anyhow::Result<String> {
    info!("attempt_login");
    let mut lock = FAKE_DB.lock().await;
    let wee = lock.iter()
        .find(|login| login.meeting == meeting && login.email == email && login.secret == token);

    match wee {
        Some(login) => {
            let valid = Utc::now() < login.valid_until;
            lock.retain(|l| !(l.email == email && l.meeting == meeting && l.secret == token));

            if valid {
                info!("Login successful");
                Ok(MeetingEmailClaims::new(meeting, email).to_jwt(&get_jwt_secret()?)?)
            } else {
                warn!("To late to login");
                bail!("To late loooser")
            }
        },
        None => {
            warn!("No maching login");
            bail!("Go home, you are drunk")
        }
    }
}
