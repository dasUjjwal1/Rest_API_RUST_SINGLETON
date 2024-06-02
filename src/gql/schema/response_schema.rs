use async_graphql::SimpleObject;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, SimpleObject, Serialize, Deserialize)]
pub struct SuccessMessage {
    pub message: String,
}
