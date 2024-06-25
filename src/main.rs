use std::sync::Arc;

use axum::{
    http::{
        header::{ACCEPT, AUTHORIZATION, CONTENT_TYPE},
        HeaderValue, Method,
    },
    routing::get,
    serve, Router,
};
use controller::{
    task_controller::{
        create_task, delete_specific_tasks_by_user_id, get_all_tasks_by_user_id,
        get_specific_tasks_by_user_id, update_specific_tasks_by_user_id,
    },
    user_controller::{create_user, get_all_user},
};

use model::{task_model::Tasks, user_model::Users};
use tokio::sync::RwLock;
use tower_http::cors::CorsLayer;
pub mod config;
pub mod controller;
pub mod model;

#[derive(Debug, Clone)]
pub struct SingletonContext {
    user_data: Users,
    task_data: Tasks,
}
impl SingletonContext {
    fn new() -> Self {
        Self {
            task_data: Tasks::new(),
            user_data: Users::new(),
        }
    }
}

#[tokio::main]
async fn main() {
    let cache = Arc::new(RwLock::new(SingletonContext::new()));

    let origins = ["http://localhost:3000".parse::<HeaderValue>().unwrap()];
    let cors = CorsLayer::new()
        .allow_origin(origins)
        .allow_methods([Method::POST, Method::GET, Method::PUT, Method::DELETE])
        .allow_credentials(true)
        .allow_headers([AUTHORIZATION, ACCEPT, CONTENT_TYPE]);

    let app = Router::new()
        .route("/users", get(get_all_user).post(create_user))
        .route(
            "/user/:user_id/tasks",
            get(get_all_tasks_by_user_id).post(create_task),
        )
        .route(
            "/user/:user_id/tasks/:task_id",
            get(get_specific_tasks_by_user_id)
                .put(update_specific_tasks_by_user_id)
                .delete(delete_specific_tasks_by_user_id),
        )
        .with_state(cache)
        .layer(cors);
    let listener = tokio::net::TcpListener::bind("127.0.0.1:8080")
        .await
        .unwrap();
    println!("🚀 server started successfully");
    serve(listener, app).await.unwrap();
}
