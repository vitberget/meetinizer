use std::sync::{Arc, LazyLock};

use anyhow::bail;
use base64::{Engine, engine::general_purpose};
use chrono::{DateTime, Duration, Utc};
use tokio::sync::Mutex;
use uuid::Uuid;

use crate::server::login::claims::MeetingEmailClaims;

#[derive(Debug)]
pub struct Login {
    pub meeting: String,
    pub email: String,
    pub secret: String,
    pub valid_until: DateTime<Utc>
}

static FAKE_DB: LazyLock<Arc<Mutex<Vec<Login>>>> = LazyLock::new(Default::default);

pub async fn register_login(meeting: &str, email: &str) -> anyhow::Result<i64> {
    println!("register_login {meeting} {email}");

    const VALID_TIME: Duration = Duration::minutes(15);
    let secret = general_purpose::STANDARD_NO_PAD.encode(Uuid::new_v4().to_bytes_le());
    let valid_until = Utc::now() + VALID_TIME;

    let login = Login {
        meeting: meeting.to_owned(),
        email: email.to_owned(),
        secret: secret.to_owned(),
        valid_until,
    };

    println!(" Login {login:?}");

    FAKE_DB.lock().await.push(login);

    Ok(VALID_TIME.as_seconds_f32() as i64)
}


pub async fn attempt_login(meeting: &str, email: &str, secret: &str) -> anyhow::Result<String> {
    let mut lock = FAKE_DB.lock().await;
    let wee = lock.iter()
        .find(|login| login.meeting == meeting && login.email == email && login.secret == secret);

    match wee {
        Some(login) => {
            let valid = Utc::now() < login.valid_until;
            lock.retain(|l| !(l.email == email && l.meeting == meeting && l.secret == secret));

            if valid {
                Ok(MeetingEmailClaims::new(meeting, email).to_jwt("secret")?)
            } else {
                bail!("To late loooser")
            }
        },
        None => bail!("Go home, you are drunk")
    }
}
