use std::collections::HashSet;

use anyhow::ensure;
use chrono::{DateTime, Local};
use serde::Serialize;
use utoipa::ToSchema;
use uuid::Uuid;

#[derive(Clone, Debug, PartialEq, Eq, Hash, Serialize, ToSchema)]
pub struct User {
    pub name: String,
    pub email: String
}

#[derive(Clone, Debug, PartialEq, Eq, Hash, Serialize, ToSchema)]
pub struct Slot {
    pub start: DateTime<Local>,
    pub end: DateTime<Local>,
}

#[derive(Clone, Debug, PartialEq, Eq, Hash, Serialize, ToSchema)]
pub struct Vote {
    pub user: User,
    pub slot: Slot
}

#[derive(Clone, Debug, Serialize, ToSchema)]
pub struct Meeting {
    id: Uuid,
    name: String,
    comment: String,
    slots: HashSet<Slot>,
    users: HashSet<User>,
    votes: HashSet<Vote>
}

impl Meeting {
    pub fn new(name: String) -> Self {
        Self {
            id: Uuid::new_v4(),
            name,
            comment: "".to_string(),
            slots: HashSet::new(),
            users: HashSet::new(),
            votes: HashSet::new(),
        }
    }

    pub fn get_id(&self) -> Uuid { self.id }

    pub fn get_name(&self) -> String { self.name.to_owned() }
    pub fn set_name(&mut self, new_name: String) { self.name = new_name; }

    pub fn get_comment(&self) -> String { self.comment.to_owned() }
    pub fn set_comment(&mut self, new_comment: String) { self.comment = new_comment; }

    pub fn add_user(&mut self, user: User) { self.users.insert(user); }

    pub fn add_slot(&mut self, slot: Slot) { self.slots.insert(slot); }

    pub fn remove_user(&mut self, user: User) -> anyhow::Result<()> { 
        let has_vote = self.votes.iter()
            .any(|vote| vote.user == user);
        ensure!(!has_vote, "User has votes");

        self.users.remove(&user);
        Ok(())
    }

    pub fn remove_slot(&mut self, slot: Slot) -> anyhow::Result<()> { 
        let has_vote = self.votes.iter()
            .any(|vote| vote.slot == slot);
        ensure!(!has_vote, "Slot has votes");

        self.slots.remove(&slot);
        Ok(())
    }

    pub fn add_vote(&mut self, user: User, slot: Slot) -> anyhow::Result<()> {
        ensure!(self.users.contains(&user), "User not in list of users");
        ensure!(self.slots.contains(&slot), "Slot not in list of slots");

        self.votes.insert(Vote { user, slot });

        Ok(())
    }

    pub fn remove_vote(&mut self, vote: &Vote) {
        self.votes.remove(vote);
    }
}
