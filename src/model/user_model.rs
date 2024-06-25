use std::collections::HashMap;

use serde::{Deserialize, Serialize};
use uuid::Uuid;
use validator::Validate;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct User {
    id: Uuid,
    user: String,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct Users(HashMap<Uuid, String>);
impl Users {
    pub fn new() -> Self {
        Self::default()
    }
    pub fn get_all_user(&self) -> Vec<User> {
        let mut user_list: Vec<User> = Vec::new();
        for (k, v) in &self.0 {
            user_list.push(User {
                id: *k,
                user: v.to_string(),
            })
        }
        user_list
    }
    pub fn add(&mut self, name: String) {
        self.0.insert(Uuid::new_v4(), name);
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, Validate)]
pub struct UserInput {
    #[validate(length(min = 1, message = "Can not be empty"))]
    pub name: String,
}
