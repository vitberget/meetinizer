use tracing::info;

use crate::db::get_meeting_connection;

mod embedded {
    use refinery::embed_migrations;
    embed_migrations!("sql_migrations");
}

pub fn init_meeting() -> anyhow::Result<()> {
    info!("Running init_meeting");

    let mut conn = get_meeting_connection()?;
    embedded::migrations::runner().run(&mut conn)?;

    info!("Running init_meeting finished successfully");

    Ok(())
}
