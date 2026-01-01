use std::sync::{Arc, LazyLock};

use anyhow::bail;
use rusqlite::{Error, named_params};
use serde_json::json;
use tokio::sync::broadcast::{Receiver, Sender, channel};
use tokio::sync::{Mutex};
use tracing::error;
use uuid::Uuid;

use crate::db::get_meeting_connection;
use crate::structs::{Meeting, Slot, User, Vote};

static MEETING_QUEUE: LazyLock<Arc<Sender<Meeting>>> = LazyLock::new( || {
        let (sender, _receiver) = channel(100);
        Arc::new(sender)
    } 
);

pub fn subscribe_to_meeting_queue() -> Receiver<Meeting> {
    let meeting_queue = Arc::clone(&MEETING_QUEUE);
    meeting_queue.subscribe()
}

pub static MEETING_DB: LazyLock<Arc<Mutex<MeetingDB>>> = LazyLock::new(|| Arc::new(Mutex::new(MeetingDB { not_for_you: true })));

pub struct MeetingDB { 
    #[allow(unused)]
    not_for_you: bool
}

impl MeetingDB {
    fn insert_meeting(&self, meeting: &Meeting) -> anyhow::Result<()> {
        get_meeting_connection()?.execute(
            "INSERT INTO meetings (name, uuid, version, json) VALUES (:name, :uuid, :version, :json)", 
            named_params! {
                ":name": meeting.get_name(),
                ":uuid": meeting.get_id().to_string(),
                ":version": meeting.get_revision().to_string(),
                ":json": json!(&meeting).to_string()
            })?;

        let meeting = meeting.to_owned();
        tokio::task::spawn(async {
            let queue = Arc::clone(&MEETING_QUEUE);
            let _ = queue.send(meeting);
        });

        Ok(())
    }

    pub fn create_meeting(&self, meeting_name: &str) -> anyhow::Result<()> {
        let meeting = Meeting::new(meeting_name);
        self.insert_meeting(&meeting)?;
        Ok(())
    }


    pub fn get_meeting_names(&self) -> anyhow::Result<Vec<String>> {
        let conn = get_meeting_connection()?;
        let mut stmt = conn.prepare("SELECT DISTINCT name from meetings")?;
        Ok(stmt
            .query_map([], |row| row.get("name"))?
            .flatten()
            .collect())
    }

    pub fn get_meeting_by_name(&self, name: &str) -> anyhow::Result<Meeting> {
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

    pub fn get_meeting_by_uuid(&self, uuid: &Uuid) -> anyhow::Result<Meeting> {
        let conn = get_meeting_connection()?;
        let mut stmt = conn.prepare("SELECT name, uuid, version, json from meetings where uuid = :uuid order by created desc limit 1")?;

        Ok(stmt.query_row(
            named_params! { ":uuid": uuid.to_string() },
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

    pub fn add_user(&self, meeting_uuid: &Uuid, revision: &Uuid, user: User) -> anyhow::Result<Meeting> {
        let mut meeting = self.get_meeting_by_uuid(meeting_uuid)?;
        if meeting.get_revision() == *revision {
            meeting.add_user(user)?;
            self.insert_meeting(&meeting)?;
            Ok(meeting)
        } else {
            bail!("Wrong revision");
        }
    }
    pub fn add_user_unsafe(&self, meeting_id: &str, user: User) -> anyhow::Result<Meeting> {
        let mut meeting = self.get_meeting_by_name(meeting_id)?;
        meeting.add_user(user)?;
        self.insert_meeting(&meeting)?;
        Ok(meeting)
    }

    pub fn add_slot(&self, meeting_uuid: &Uuid, revision: &Uuid, slot: Slot) -> anyhow::Result<Meeting> {
        let mut meeting = self.get_meeting_by_uuid(meeting_uuid)?;
        if meeting.get_revision() == *revision {
            // let slot = Slot::from_str(start, end)?;
            meeting.add_slot(slot);
            self.insert_meeting(&meeting)?;
            Ok(meeting)
        } else {
            bail!("Wrong revision");
        }
    }
    pub fn add_slot_unsafe(&self, meeting_name: &str, slot: Slot) -> anyhow::Result<Meeting> {
        let mut meeting = self.get_meeting_by_name(meeting_name)?;
        meeting.add_slot(slot);
        self.insert_meeting(&meeting)?;
        Ok(meeting)
    }
    pub fn rm_slot_unsafe(&self, meeting_name: &str, slot: Slot) -> anyhow::Result<Meeting> {
        let mut meeting = self.get_meeting_by_name(meeting_name)?;
        meeting.remove_slot_unsafe(slot);
        self.insert_meeting(&meeting)?;
        Ok(meeting)
    }
    pub fn add_vote_unsafe(&self, meeting_name: &str, vote: Vote) -> anyhow::Result<Meeting> {
        let mut meeting = self.get_meeting_by_name(meeting_name)?;
        meeting.add_vote(vote)?;
        self.insert_meeting(&meeting)?;
        Ok(meeting)
    }
    pub fn rm_vote_unsafe(&self, meeting_name: &str, vote: Vote) -> anyhow::Result<Meeting> {
        let mut meeting = self.get_meeting_by_name(meeting_name)?;
        meeting.remove_vote(&vote);
        self.insert_meeting(&meeting)?;
        Ok(meeting)
    }
}

#[cfg(test)]
mod tests {
    use std::str::FromStr;

    use anyhow::bail;
    use chrono::{DateTime, Datelike, Local, NaiveDate, Timelike};

    use super::*;

    #[tokio::test]
    #[ignore]
    async fn test_get_meeting_by_uuid() -> anyhow::Result<()> {
        let arc = Arc::clone(&MEETING_DB);
        let real_db = arc.lock().await;
        let meeting = real_db.get_meeting_by_uuid(&Uuid::from_str("497eb28f-2f5a-4668-8275-22904646bfe5")?)?;
        println!("meeting {meeting:?}");

        bail!("meh")
    }


    #[tokio::test]
    #[ignore]
    async fn test_get_meeting_names() -> anyhow::Result<()> {
        let arc = Arc::clone(&MEETING_DB);
        let real_db = arc.lock().await;
        let meetings = real_db.get_meeting_names()?;
        println!("meetings {meetings:?}");

        bail!("meh")
    }

    #[test]
    fn test_parse_datetime() -> anyhow::Result<()> {
        let text = "\"2025-06-04T15:00:00+02:00\"";
        let dt: DateTime<Local> = serde_json::from_str(text)?;
        assert_eq!(dt.year(), 2025);
        assert_eq!(dt.month(), 6);
        assert_eq!(dt.day(), 4);
        assert_eq!(dt.hour(), 15);
        Ok(())
    }

    #[tokio::test]
    #[ignore]
    async fn test_insert_meeting() -> anyhow::Result<()> {
        let arc = Arc::clone(&MEETING_DB);
        let real_db = arc.lock().await;

        let meeting = Meeting::new("alive");
        real_db.insert_meeting(&meeting)?;
        Ok(())
    }

    #[tokio::test]
    #[ignore]
    async fn test_update_meeting() -> anyhow::Result<()> {
        let arc = Arc::clone(&MEETING_DB);
        let real_db = arc.lock().await;

        let mut meeting = real_db.get_meeting_by_name("alive")?;
        meeting.set_comment("Some rando comment");
        let slot = Slot { 
            start: NaiveDate::from_ymd_opt(2025, 6, 4).unwrap().and_hms_opt(15,0,0).unwrap().and_local_timezone(Local).earliest().unwrap(),
            end: NaiveDate::from_ymd_opt(2025, 6, 4).unwrap().and_hms_opt(22,0,0).unwrap().and_local_timezone(Local).earliest().unwrap(),
        };
        meeting.add_slot(slot);

        real_db.insert_meeting(&meeting)?;
        bail!("e")
    }

    #[tokio::test]
    #[ignore]
    async fn test_get_meeting() -> anyhow::Result<()> {
        let arc = Arc::clone(&MEETING_DB);
        let real_db = arc.lock().await;

        let meeting = real_db.get_meeting_by_name("alive")?;

        println!("meeting {meeting:?}");

        bail!("e")
    }
}
