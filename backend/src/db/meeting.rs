use std::sync::{Arc, LazyLock};

use rusqlite::{Error, named_params};
use serde_json::json;
use tokio::sync::Mutex;
use tracing::error;

use crate::db::get_meeting_connection;
use crate::structs::Meeting;


static REAL_DB: LazyLock<Arc<Mutex<RealDB>>> = LazyLock::new(|| Arc::new(Mutex::new(RealDB { not_for_you: true })));

pub struct RealDB { 
    #[allow(unused)]
    not_for_you: bool
}

impl RealDB {
    pub fn insert_meeting(&self, meeting: &Meeting) -> anyhow::Result<()> {
        get_meeting_connection()?.execute(
            "INSERT INTO meetings (name, uuid, version, json) VALUES (:name, :uuid, :version, :json)", 
            named_params! {
                ":name": meeting.get_name(),
                ":uuid": meeting.get_id().to_string(),
                ":version": meeting.get_revision().to_string(),
                ":json": json!(&meeting).to_string()
            })?;

        Ok(())
    }

    pub fn get_meeting(&self, name: &str) -> anyhow::Result<Meeting> {
        let conn = get_meeting_connection()?;
        let mut stmt = conn.prepare("SELECT name, uuid, version, json from meetings where name = :name order by created desc limit 1")?;

        Ok(stmt.query_row(
            named_params! { ":name": name },
            |row| {
                let json: String = row.get("json")?;
                match serde_json::from_str::<Meeting>(&json) {
                    Ok(meeting) => Ok(meeting),
                    Err(err) => {
                        error!("Failed to parse json to meeting {json} {err}");
                        Err(Error::InvalidColumnName("Not json in json".to_string()))
                    }
                }
            }
        )?)
    }
}

#[cfg(test)]
mod tests {
    use anyhow::bail;

    use super::*;

    #[tokio::test]
    #[ignore]
    async fn test_insert_meeting() -> anyhow::Result<()> {
        let arc = Arc::clone(&REAL_DB);
        let real_db = arc.lock().await;

        let meeting = Meeting::new("I am alive");
        real_db.insert_meeting(&meeting)?;
        Ok(())
    }

    #[tokio::test]
    #[ignore]
    async fn test_update_meeting() -> anyhow::Result<()> {
        let arc = Arc::clone(&REAL_DB);
        let real_db = arc.lock().await;

        let mut meeting = real_db.get_meeting("I am alive")?;
        meeting.set_comment("Some rando comment");
        println!("meeting {meeting:?}");

        real_db.insert_meeting(&meeting)?;
        bail!("e")
    }

    #[tokio::test]
    #[ignore]
    async fn test_get_meeting() -> anyhow::Result<()> {
        let arc = Arc::clone(&REAL_DB);
        let real_db = arc.lock().await;

        let meeting = real_db.get_meeting("I am alive")?;

        println!("meeting {meeting:?}");

        bail!("e")
    }
}
