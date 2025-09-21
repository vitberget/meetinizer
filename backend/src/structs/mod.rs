use std::collections::HashSet;

use anyhow::ensure;
use chrono::{DateTime, Local};
use serde::{Deserialize, Serialize};
use utoipa::ToSchema;
use uuid::Uuid;

#[derive(Clone, Debug, PartialEq, Eq, Hash, Serialize, Deserialize, ToSchema)]
pub struct User {
    pub name: String,
    pub email: String
}

impl User {
    pub fn new(name: &str, email: &str) -> Self {
        Self {
            name: name.to_owned(),
            email: email. to_owned()
        }
    }
}

#[derive(Clone, Debug, PartialEq, Eq, Hash, Serialize, Deserialize, ToSchema)]
pub struct Slot {
    pub start: DateTime<Local>,
    pub end: DateTime<Local>,
}

#[derive(Clone, Debug, PartialEq, Eq, Hash, Serialize, Deserialize, ToSchema)]
pub struct Vote {
    pub user_email: String,
    pub slot: Slot
}

#[derive(Clone, Debug, Serialize, Deserialize, ToSchema)]
pub struct Meeting {
    id: Uuid,
    revision: Uuid,
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
            revision: Uuid::new_v4(),
            name,
            comment: "".to_string(),
            slots: HashSet::new(),
            users: HashSet::new(),
            votes: HashSet::new(),
        }
    }

    pub fn get_id(&self) -> Uuid { self.id }

    pub fn get_name(&self) -> String { self.name.to_owned() }
    pub fn set_name(&mut self, new_name: String) { 
        self.name = new_name;
        self.revision = Uuid::new_v4();
    }

    pub fn get_comment(&self) -> String { self.comment.to_owned() }
    pub fn set_comment(&mut self, new_comment: String) {
        self.comment = new_comment; 
        self.revision = Uuid::new_v4();
    }

    pub fn add_user(&mut self, user: User) { self.users.insert(user); }

    pub fn add_slot(&mut self, slot: Slot) {
        self.slots.insert(slot); 
        self.revision = Uuid::new_v4();
    }

    pub fn remove_user(&mut self, user: User) -> anyhow::Result<()> { 
        let has_vote = self.votes.iter()
            .any(|vote| vote.user_email == user.email);
        ensure!(!has_vote, "User has votes");

        self.users.remove(&user);
        self.revision = Uuid::new_v4();
        Ok(())
    }

    pub fn remove_slot(&mut self, slot: Slot) -> anyhow::Result<()> { 
        let has_vote = self.votes.iter()
            .any(|vote| vote.slot == slot);
        ensure!(!has_vote, "Slot has votes");

        self.slots.remove(&slot);
        self.revision = Uuid::new_v4();
        Ok(())
    }

    pub fn add_vote(&mut self, user: User, slot: Slot) -> anyhow::Result<()> {
        ensure!(self.users.contains(&user), "User not in list of users");
        ensure!(self.slots.contains(&slot), "Slot not in list of slots");

        self.votes.insert(Vote { user_email: user.email, slot });
        self.revision = Uuid::new_v4();

        Ok(())
    }

    pub fn remove_vote(&mut self, vote: &Vote) {
        self.votes.remove(vote);
        self.revision = Uuid::new_v4();
    }
}
