use std::sync::Arc;

use axum::{extract::State, http::StatusCode, response::IntoResponse, Json};
use serde_json::{json, Value};
use tokio::sync::RwLock;

use crate::{
    config::validator_config::ValidatedJson, model::user_model::UserInput, SingletonContext,
};

pub async fn create_user(
    State(state): State<Arc<RwLock<SingletonContext>>>,
    ValidatedJson(body): ValidatedJson<UserInput>,
) -> Result<impl IntoResponse, (StatusCode, Json<Value>)> {
    let mut data = state.write().await;
    data.user_data.add(body.name);
    Ok((StatusCode::OK, Json(json!("Success"))))
}
pub async fn get_all_user(
    State(state): State<Arc<RwLock<SingletonContext>>>,
) -> Result<impl IntoResponse, (StatusCode, Json<Value>)> {
    let data = state.read().await;
    println!("{:?}", data);
    Ok((
        StatusCode::OK,
        Json(json!({"data":data.user_data.get_all_user()})),
    ))
}
