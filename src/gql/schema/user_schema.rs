use async_graphql::{InputObject, SimpleObject};
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use sqlx::prelude::FromRow;

use super::task_schema::Tasks;

#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
pub struct User {
    pub id: i32,
    pub name: String,
    pub user_name: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub created_at: Option<DateTime<Utc>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub modified_at: Option<DateTime<Utc>>,
}

#[derive(Debug, Clone, Serialize, Deserialize, InputObject)]
pub struct UserCreateInput {
    #[graphql(validator(min_length = 1))]
    pub name: String,
    #[graphql(validator(min_length = 1))]
    pub user_name: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, SimpleObject)]
pub struct UserResponse {
    pub id: i32,
    pub name: String,
    pub user_name: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tasks: Option<Vec<Tasks>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub created_at: Option<DateTime<Utc>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub modified_at: Option<DateTime<Utc>>,
}
