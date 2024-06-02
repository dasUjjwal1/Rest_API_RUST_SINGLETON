use std::sync::{Arc, Mutex};

// use async_graphql::{EmptySubscription, Schema};
// use async_graphql_axum::{GraphQLRequest, GraphQLResponse};
use axum::{
    http::{
        header::{ACCEPT, AUTHORIZATION, CONTENT_TYPE},
        HeaderValue, Method,
    },
    routing::get,
    serve, Router,
};
// use config::db_config::Config;
use controller::{
    task_controller::{
        create_task, delete_specific_tasks_by_user_id, get_all_tasks_by_user_id,
        get_specific_tasks_by_user_id, update_specific_tasks_by_user_id,
    },
    user_controller::{create_user, get_all_user},
};
// use dotenv::dotenv;
// use gql::resolver::{Mutation, Query};
use model::{task_model::Tasks, user_model::Users};
// use sqlx::{postgres::PgPoolOptions, Pool, Postgres};
use tower_http::cors::CorsLayer;
pub mod config;
pub mod controller;
// pub mod gql;
pub mod model;
// type RootSchema = Schema<Query, Mutation, EmptySubscription>;

// lazy_static! {
//     static ref USER_MAP: Mutex<HashMap<String, String>> = {
//         let mut map = HashMap::new();
//         Mutex::new(map)
//     };
// }
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

// async fn graphql_handler(State(schema): State<RootSchema>, req: GraphQLRequest) -> GraphQLResponse {
//     schema.execute(req.into_inner()).await.into()
// }
// pub struct ApplicationState {
//     pub env: Config,
//     pub pg_pool: Pool<Postgres>,
// }
#[tokio::main]
async fn main() {
    // dotenv().ok();
    let cache = Arc::new(Mutex::new(SingletonContext::new()));
    // let config = Config::init();

    let origins = [
        // "https://studio.apollographql.com"
        //     .parse::<HeaderValue>()
        //     .unwrap(),
        "http://localhost:3000".parse::<HeaderValue>().unwrap(),
    ];
    let cors = CorsLayer::new()
        .allow_origin(origins)
        .allow_methods([Method::POST, Method::GET, Method::PUT, Method::DELETE])
        .allow_credentials(true)
        .allow_headers([AUTHORIZATION, ACCEPT, CONTENT_TYPE]);
    // let pool = match PgPoolOptions::new()
    //     .max_connections(10)
    //     .connect(&config.database_url)
    //     .await
    // {
    //     Ok(pool) => {
    //         println!("🚀 database connected");
    //         pool
    //     }
    //     Err(err) => {
    //         println!("Database connected error ,{:?}", err);
    //         std::process::exit(1);
    //     }
    // };
    // let schema = Schema::build(Query::default(), Mutation::default(), EmptySubscription)
    //     .data(Arc::new(ApplicationState {
    //         env: config,
    //         pg_pool: pool,
    //     }))
    //     .finish();
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
        // .route("/graphql", post(graphql_handler))
        // .with_state(schema)
        .layer(cors);
    let listener = tokio::net::TcpListener::bind("127.0.0.1:8080")
        .await
        .unwrap();
    println!("🚀 server started successfully");
    serve(listener, app).await.unwrap();
}
