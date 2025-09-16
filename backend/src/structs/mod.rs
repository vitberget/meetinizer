use std::collections::HashSet;

use anyhow::bail;
use chrono::{DateTime, Local};

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub struct User {
    pub name: String,
    pub email: String
}

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub struct Slot {
    pub start: DateTime<Local>,
    pub end: DateTime<Local>,
}

#[derive(Clone, Debug)]
pub struct Meeting {
    pub name: String,
    pub comment: String,
    pub slots: HashSet<Slot>,
    pub users: HashSet<User>,
    pub votes: HashSet<(User, Slot)>
}

impl Meeting {
    pub fn add_vote(&mut self, user: User, slot: Slot) -> anyhow::Result<()> {
        if !self.users.contains(&user) {
            bail!("User not in users")
        }

        Ok(())
    }
}
