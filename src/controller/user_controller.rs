use std::sync::{Arc, Mutex};

use axum::{extract::State, http::StatusCode, response::IntoResponse, Json};
use serde_json::{json, Value};

use crate::{
    model::user_model::{User, UserInput},
    SingletonContext,
};

pub async fn create_user(
    State(state): State<Arc<Mutex<SingletonContext>>>,
    Json(body): Json<UserInput>,
) -> Result<impl IntoResponse, (StatusCode, Json<Value>)> {
    let mut data = state.lock().unwrap();
    println!("{:?}", data);
    data.user_data.add(User::new(body.name));
    Ok((StatusCode::OK, Json(json!("Success"))))
}
pub async fn get_all_user(
    State(state): State<Arc<Mutex<SingletonContext>>>,
) -> Result<impl IntoResponse, (StatusCode, Json<Value>)> {
    let data = state.lock().unwrap();
    Ok((StatusCode::OK, Json(json!({"data":data.user_data}))))
}
