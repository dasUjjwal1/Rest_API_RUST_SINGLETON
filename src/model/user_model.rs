use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct User {
    pub id: Uuid,
    pub name: String,
}

impl User {
    pub fn new(name: String) -> Self {
        Self {
            id: Uuid::new_v4(),
            name,
        }
    }
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct Users(Vec<User>);
impl Users {
    pub fn new() -> Self {
        Self::default()
    }
    pub fn get_all_user(self) -> Vec<User> {
        self.0
    }
    pub fn add(&mut self, user: User) {
        self.0.push(user);
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UserInput {
    pub name: String,
}
