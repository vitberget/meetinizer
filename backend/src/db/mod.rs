pub mod init;
pub mod meeting;

pub fn get_meeting_connection() -> anyhow::Result<rusqlite::Connection> {
    Ok(rusqlite::Connection::open("data/meeting.sqlite")?)
}
